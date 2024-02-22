use super::*;
use crate::response::ErrorDetail;
use calc_af::calc_af;
use calc_gets::calc_gets;
use calc_irfe::{calc_irfe, calc_receipts_to_pay};
use calc_manual_entry::calc_manual_entry_to_pay;
use calc_rb_or_irex::calc_item;
use sqlx::postgres::PgRow;
use std::marker::Send;

pub async fn calc(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CalcPayload>,
) -> impl IntoResponse {
    let where_clause = format!(
        "WHERE 
            (ts.end_date IS NULL
            OR (ts.end_date >= DATE_TRUNC('month', CAST('{}' AS TIMESTAMP))
                AND ts.end_date <= DATE_TRUNC('month', CAST('{}' AS TIMESTAMP)) + INTERVAL '1 month - 1 day'))",
        payload.payroll_date, payload.payroll_date
    );

    let payroll_date = &payload.payroll_date;

    let people_query = format!("{} {};", SELECT_PEOPLE_PAYROLL_QUERY, where_clause);
    let result_people: Vec<PeopleRes> = fetch_all(&people_query, payroll_date, &pool).await;

    let deps_query = format!("{} {};", SELECT_DEPENDENTS_QUERY, where_clause);
    let result_dependents: Vec<DependentsRes> = fetch_all(&deps_query, payroll_date, &pool).await;

    let receipts_query = format!("{} {};", SELECT_RF_RECEIPTS_QUERY, where_clause);
    let result_receipts: Vec<ReceiptsRes> = fetch_all(&receipts_query, payroll_date, &pool).await;

    let paid_reipts_query = format!("{};", SELECT_PAID_RECEIPTS_QUERY);
    let result_paid_recps: Vec<PaidReceiptsRes> =
        fetch_all(&paid_reipts_query, payroll_date, &pool).await;

    let manual_entries_query = format!(
        "{} WHERE me.end_date >= DATE_TRUNC('month', CAST('{}' AS TIMESTAMP));",
        SELECT_MANUAL_ENTRIES_QUERY, payload.payroll_date
    );
    let result_manual_entries: Vec<ManualEntriesRes> =
        fetch_all(&manual_entries_query, payroll_date, &pool).await;

    if result_people.is_empty() || result_dependents.is_empty() || result_receipts.is_empty() {
        eprintln!("Error to fetch data!");
        let error = ApiResponse::<()>::error(ErrorDetail {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "Failed to fetch data.".to_string(),
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
    }

    // dbg!(&result_people);

    let mut payroll_data: Vec<PayrollData> = vec![];
    for p in result_people {
        // RB - Retribuição Básica
        let rb = calc_item(
            p.rci_fc_rb,
            p.city_fc_rb,
            p.start_date,
            p.end_date,
            *payroll_date,
            Uuid::parse_str(&var("ID_RB").unwrap()).unwrap(),
            p.person_id,
        );
        payroll_data.push(rb.clone());

        // IREX - Indenização de Representação no Exterior
        let irex = calc_item(
            p.rci_fc_irex,
            p.city_fc_irex,
            p.start_date,
            p.end_date,
            *payroll_date,
            Uuid::parse_str(&var("ID_IREX").unwrap()).unwrap(),
            p.person_id,
        );
        payroll_data.push(irex.clone());

        // GETS - Gratificação no Exterior por tempo de serviço
        let time_served_abroad_query = format!(
            "{} WHERE ts.person_id = '{}';",
            SELECT_TIME_SERVED_ABROAD_QUERY, p.person_id
        );
        let result_tsa: Vec<TimeServedAbroadRes> =
            fetch_all(&time_served_abroad_query, payroll_date, &pool).await;

        if result_tsa.is_empty() {
            eprintln!("Error to fetch data!");
            let error = ApiResponse::<()>::error(ErrorDetail {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Failed to fetch data.".to_string(),
            });
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
        }

        let gets = calc_gets(result_tsa, *payroll_date, rb.value, p.person_id);
        payroll_data.push(gets.clone());

        // AF - Auxílio-Familiar
        let filtered_deps: Vec<&DependentsRes> = result_dependents
            .iter()
            .filter(|item| item.person_id == p.person_id)
            .collect();
        let af = calc_af(filtered_deps, *payroll_date, irex.value, p.person_id);
        payroll_data.push(af.clone());

        // IRFE -  Auxílio-Moradia no Exterior
        let filtered_recps: Vec<&ReceiptsRes> = result_receipts
            .iter()
            .filter(|item| item.person_id == p.person_id)
            .collect();

        let filtered_paid_recps: Vec<&PaidReceiptsRes> = result_paid_recps
            .iter()
            .filter(|item| item.person_id == p.person_id)
            .collect();

        let receipts_to_pay = calc_receipts_to_pay(
            filtered_recps,
            filtered_paid_recps,
            p.rci_fc_irfe,
            p.city_fc_irfe,
        );
        for r in receipts_to_pay {
            let irfe = calc_irfe(r.value, *payroll_date, p.person_id);
            payroll_data.push(irfe);
        }

        // Manual Entries
        let filtered_manual_entries: Vec<&ManualEntriesRes> = result_manual_entries
            .iter()
            .filter(|item| item.person_id == p.person_id)
            .collect();
        let manual_entries_to_pay =
            calc_manual_entry_to_pay(filtered_manual_entries, *payroll_date);
        for e in manual_entries_to_pay {
            payroll_data.push(e);
        }

        // PSS
        let pss = PayrollData {
            payroll_item: Uuid::parse_str(&var("ID_PSS").unwrap()).unwrap(),
            person_id: p.person_id,
            value: p.payroll_brl_pss / payload.rate,
            date: *payroll_date,
        };
        payroll_data.push(pss);

        // AP - Abono Permanência
        if p.has_retention_bonus {
            let ap = PayrollData {
                payroll_item: Uuid::parse_str(&var("ID_AP").unwrap()).unwrap(),
                person_id: p.person_id,
                value: p.payroll_brl_pss / payload.rate,
                date: *payroll_date,
            };
            payroll_data.push(ap);
        }
    }
    dbg!(&payroll_data);
    todo!()
}

async fn fetch_all<T>(query: &str, payroll_date: &NaiveDate, pool: &PgPool) -> Vec<T>
where
    T: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    sqlx::query_as::<_, T>(query)
        .bind(payroll_date)
        .fetch_all(pool)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Error to fetch data! {:?}", err);
            vec![]
        })
}
