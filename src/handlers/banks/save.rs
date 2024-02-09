use super::*;
use axum::extract::{Extension, Path};

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<BankPayload>,
) -> impl IntoResponse {
    let query = "INSERT INTO banks (name, number) VALUES ($1, $2) RETURNING *";

    let result = sqlx::query_as::<_, BankResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.number)
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
