use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::response::{ApiResponse, ErrorDetail, Meta};

use serde::de::DeserializeOwned;

use super::*;

pub async fn handle_post_request(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<BankPayload>,
) -> impl IntoResponse
where
    BankPayload: DeserializeOwned + Send,
{
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
            let error = ErrorDetail {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error".to_string(),
            };
            let response: ApiResponse<String> = ApiResponse::error(error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}
