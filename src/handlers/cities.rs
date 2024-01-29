use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::response::{
    generate_filter_clauses, ApiResponse, ErrorDetail, Filter, Meta, Pagination,
};

#[derive(Serialize, FromRow)]
pub struct CityResponse {
    id: Uuid,
    name: String,
    country_id: Uuid,
    country: String,
    latitude: f64,
    longitude: f64,
    fc_rb: f64,
    fc_irex: f64,
}

#[derive(Deserialize)]
pub struct RoleFilter {
    names: Option<String>,
    countries: Option<String>,
}
pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<RoleFilter>,
) -> impl IntoResponse {
    let filters = vec![
        Filter {
            name: "name",
            val: filters.names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "c.name",
            val: filters.countries.as_ref(),
            conj: "OR",
        },
    ];
    let mut where_clause = generate_filter_clauses(filters);

    let count_query = format!(
        "SELECT COUNT(*) 
         FROM cities
         JOIN countries ON cities.country = countries.id
         {}",
        where_clause
    );

    let total_count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    let offset = pagination.offset(total_count as usize);
    let page_size = pagination.page_size(total_count as usize);

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
        {} 
        ORDER BY cities.name 
        LIMIT {} OFFSET {}",
        where_clause, page_size, offset
    );

    let result = sqlx::query_as::<_, CityResponse>(&query)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(items) => {
            let meta = Meta {
                total_count: Some(total_count as usize),
                page: Some(pagination.page.unwrap_or(1)),
                page_size: Some(page_size),
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
