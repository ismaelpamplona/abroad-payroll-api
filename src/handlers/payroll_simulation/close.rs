use super::*;
use sqlx::Error;

pub async fn close(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ClosePayload>,
) -> impl IntoResponse {
    let query_simulation = format!(
        "SELECT
            ps.id,
            ps.simulation_id,
            ps.payroll_item,
            ps.person_id,
            ps.value,
            ps.date,
            pr.rf_receipt_id
        FROM public.payroll_simulation ps
        LEFT JOIN public.simulation_paid_rf_receipts pr ON pr.payroll_simulation_item_id = ps.id
        WHERE ps.simulation_id = $1",
    );

    let result_simulation = sqlx::query_as::<_, SimulationResWithReceipt>(&query_simulation)
        .bind(&payload.simulation_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Error to fetch data! {:?}", err);
            vec![]
        });

    dbg!(&result_simulation);

    match insert_payroll_data(&pool, result_simulation).await {
        Ok(_) => {
            let res = ApiResponse::<SuccessInsert>::success_insert();
            (StatusCode::OK, res).into_response()
        }
        Err(error) => {
            eprintln!("Failed to insert items: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}

async fn insert_payroll_data(
    pool: &PgPool,
    simulation_data: Vec<SimulationResWithReceipt>,
) -> Result<(), Error> {
    if simulation_data.is_empty() {
        return Err(Error::RowNotFound);
    }
    let closed_id = Uuid::new_v4();
    let query_closed = "INSERT INTO public.payroll_closed (closed_id, payroll_item, person_id, value, date) VALUES ($1, $2, $3, $4, $5) RETURNING *";
    for data in simulation_data {
        let result_closed = sqlx::query_as::<_, ClosedRes>(query_closed)
            .bind(&closed_id)
            .bind(&data.payroll_item)
            .bind(&data.person_id)
            .bind(&data.value)
            .bind(&data.date)
            .fetch_one(pool)
            .await?;
        if let Some(receipt_id) = data.rf_receipt_id {
            let query_paid = "INSERT INTO public.paid_rf_receipts (rf_receipt_id, payroll_closed_item_id) VALUES ($1, $2)";
            sqlx::query(query_paid)
                .bind(&receipt_id)
                .bind(&result_closed.id)
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}
