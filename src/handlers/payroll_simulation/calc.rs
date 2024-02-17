use super::*;
use crate::response::ErrorDetail;
use formulas::{calc_af, calc_gets, calc_item};
use sqlx::postgres::PgRow;
use std::marker::Send;

pub async fn calc(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CalcPayload>,
) -> impl IntoResponse {
    let where_clause = format!(
        "WHERE 
            (ts.end_date IS NULL
            OR (ts.end_date >= DATE_TRUNC('month', {})
                AND ts.end_date <= DATE_TRUNC('month', {}) + INTERVAL '1 month - 1 day'))",
        "CURRENT_DATE", "CURRENT_DATE"
    );

    let people_query = format!("{} {};", SELECT_PEOPLE_PAYROLL_QUERY, where_clause);
    let deps_query = format!("{} {};", SELECT_DEPENDENTS_QUERY, where_clause);
    let receipts_query = format!(
        "{} {} AND rf.its_paid = FALSE;",
        SELECT_RF_RECEIPTS_QUERY, where_clause
    );

    let payroll_date = &payload.payroll_date;
    let result_people: Vec<PeopleRes> = fetch_all(&people_query, payroll_date, &pool).await;
    let result_dependents: Vec<DependentsRes> = fetch_all(&deps_query, payroll_date, &pool).await;
    let result_receipts: Vec<ReceiptsRes> = fetch_all(&receipts_query, payroll_date, &pool).await;

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
        calc_af(filtered_deps, *payroll_date, irex.value, p.person_id);
        println!(" - - - - - ");
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
