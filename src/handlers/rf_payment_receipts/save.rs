use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RFPaymentReceiptsPayload>,
) -> impl IntoResponse
where
    RFPaymentReceiptsPayload: DeserializeOwned + Send,
{
    let query =
        "INSERT INTO rf_payment_receipts AS rf (person_id, start_date, end_date, rate, value)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING rf.id, rf.person_id, 
             (SELECT name FROM people WHERE id = rf.person_id) as person_name, 
             rf.start_date, rf.end_date,  rf.rate, rf.value, rf.e_tag";

    let result = sqlx::query_as::<_, RFPaymentReceiptsResponse>(&query)
        .bind(&payload.person_id)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
        .bind(&payload.rate)
        .bind(&payload.value)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => {
            let meta = Meta {
                total_count: Some(1),
                page: Some(1),
                page_size: Some(1),
            };

            let response = ApiResponse::success_list(record, meta);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to save dependent details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
