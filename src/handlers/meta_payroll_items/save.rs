use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<PayrollItemsPayload>,
) -> impl IntoResponse
where
    PayrollItemsPayload: DeserializeOwned + Send,
{
    let query = format!(
        "INSERT INTO meta_payroll_items (code, short_name, description, transaction_type) 
        VALUES ($1, $2, $3, $4) RETURNING *"
    );

    let result = sqlx::query_as::<_, PayrollItemsResponse>(&query)
        .bind(&payload.code)
        .bind(&payload.short_name)
        .bind(&payload.description)
        .bind(&payload.transaction_type)
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
            eprintln!("Failed to save bank details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
