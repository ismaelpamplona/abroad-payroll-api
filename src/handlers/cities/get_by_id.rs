use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::response::{ApiResponse, ErrorDetail, Meta};

use super::*;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = format!(
        "SELECT 
            cities.id as id,
            cities.name as name,
            countries.id as country_id,
            countries.name as country,
            cities.latitude,
            cities.longitude,
            cities.fc_rb,
            cities.fc_irex
        FROM 
            cities
        JOIN 
            countries ON cities.country = countries.id
        WHERE 
            cities.id = $1" // Use $1 as a parameter placeholder
    );

    let result = sqlx::query_as::<_, CityResponse>(&query)
        .bind(&id)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(items) => {
            let meta = Meta {
                total_count: Some(1),
                page: Some(1),
                page_size: Some(1),
            };
            let response = ApiResponse::success_list(items, meta);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to fetch cities: {}", error);
            let error = ErrorDetail {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error".to_string(),
            };
            let response: ApiResponse<String> = ApiResponse::error(error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}
