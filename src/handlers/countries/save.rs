use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::response::{ApiResponse, ErrorDetail, Meta};

use serde::de::DeserializeOwned;

use super::*;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CountryPayload>,
) -> impl IntoResponse
where
    CountryPayload: DeserializeOwned + Send,
{
    let query = "INSERT INTO countries (name) VALUES ($1) RETURNING *";

    let result = sqlx::query_as::<_, CountryResponse>(&query)
        .bind(&payload.name)
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
            eprintln!("Failed to save country details: {}", error);
            let error = ErrorDetail {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error".to_string(),
            };
            let response: ApiResponse<String> = ApiResponse::error(error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}
