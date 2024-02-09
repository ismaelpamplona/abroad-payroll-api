use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<PayrollItemsPayload>,
) -> impl IntoResponse {
    let query = "UPDATE meta_payroll_items 
        SET code = $1, short_name = $2, description = $3, transaction_type = $4
        WHERE id = $5 RETURNING *";

    let result = sqlx::query_as::<_, PayrollItemsResponse>(&query)
        .bind(&payload.code)
        .bind(&payload.short_name)
        .bind(&payload.description)
        .bind(&payload.transaction_type)
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
            eprintln!("Failed to update payroll item details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
