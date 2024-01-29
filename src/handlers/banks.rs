use axum::{
    extract::{Extension, Path, Query},
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
pub struct BankResponse {
    id: Uuid,
    name: String,
    number: String,
}

#[derive(Deserialize)]
pub struct BankFilter {
    names: Option<String>,
    numbers: Option<String>,
}
pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<BankFilter>,
) -> impl IntoResponse {
    let filters = vec![
        Filter {
            name: "name",
            val: filters.names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "number",
            val: filters.numbers.as_ref(),
            conj: "OR",
        },
    ];

    let mut where_clause = generate_filter_clauses(filters);

    let count_query = format!("SELECT COUNT(*) FROM banks {}", where_clause);
    let total_count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    let offset = pagination.offset(total_count as usize);
    let page_size = pagination.page_size(total_count as usize);
    let query = format!(
        "SELECT * FROM banks {} ORDER BY number LIMIT {} OFFSET {}",
        where_clause, page_size, offset
    );

    let result = sqlx::query_as::<_, BankResponse>(&query)
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
            eprintln!("Failed to fetch banks: {}", error);
            let error = ErrorDetail {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error".to_string(),
            };
            let response: ApiResponse<String> = ApiResponse::error(error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

pub async fn get_bank_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = sqlx::query_as::<_, BankResponse>("SELECT * FROM banks WHERE id = $1").bind(&id);

    let result = query.fetch_all(&pool).await;

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
            eprintln!("Failed to fetch banks: {}", error);
            let error = ErrorDetail {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error".to_string(),
            };
            let response: ApiResponse<String> = ApiResponse::error(error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}
