use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CFLimitExchangeRatePayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE cf_limit_exchange_rate 
        SET value = $1, law = $2, law_date = $3 
        WHERE id = $4 RETURNING *"
    );

    let result = sqlx::query_as::<_, CFLimitExchangeRateResponse>(&query)
        .bind(&payload.value)
        .bind(&payload.law)
        .bind(&payload.law_date)
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
            eprintln!("Failed to update cf_limit_exchange_rate details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
