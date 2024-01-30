use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use super::*;

use crate::response::{
    generate_filter_clauses, ApiResponse, ErrorDetail, Filter, Meta, Pagination,
};

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<RoleClassFilter>,
) -> impl IntoResponse {
    let filters = vec![
        Filter {
            name: "r.name",
            val: filters.role_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "c.name",
            val: filters.class_names.as_ref(),
            conj: "OR",
        },
    ];
    let mut where_clause = generate_filter_clauses(filters);

    let count_query = format!(
        "SELECT COUNT(*) 
         FROM roles_classes_indexes rci
         JOIN roles r ON rci.role = r.id
         JOIN classes c ON rci.class = c.id
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
            rci.id as rci_id,
            r.id as role_id,
            r.name as role_name,
            c.id as class_id,
            c.name as class_name,
            rci.fc_rb,
            rci.fc_irex
        FROM 
            roles_classes_indexes rci
        JOIN 
            roles r ON rci.role = r.id
        JOIN 
            classes c ON rci.class = c.id
        {} 
        ORDER BY r.name 
        LIMIT {} OFFSET {}",
        where_clause, page_size, offset
    );

    let result = sqlx::query_as::<_, RoleClassIndexResponse>(&query)
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
            eprintln!("Failed to fetch [roles_classes_indexes]: {}", error);
            let error = ErrorDetail {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error".to_string(),
            };
            let response: ApiResponse<String> = ApiResponse::error(error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}
