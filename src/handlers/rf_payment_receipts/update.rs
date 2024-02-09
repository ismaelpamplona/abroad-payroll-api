use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<RFPaymentReceiptsPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE rf_payment_receipts as rf
        SET person_id = $1, start_date = $2, end_date = $3, rate = $4, value = $5
        WHERE id = $6 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, RFPaymentReceiptsResponse>(&query)
        .bind(&payload.person_id)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
        .bind(&payload.rate)
        .bind(&payload.value)
        .bind(&id)
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
            eprintln!("Failed to update receipt details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
