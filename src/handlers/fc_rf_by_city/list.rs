use super::*;
use crate::response::{generate_filter_clauses, Filter, Operator, Pagination};
use axum::extract::Query;

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<FcByCityFilter>,
) -> impl IntoResponse {
    let filters = vec![
        Filter {
            name: "c.name",
            op: Operator::ILIKE,
            val: filters.city_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "co.name",
            op: Operator::ILIKE,
            val: filters.country_names.as_ref(),
            conj: "OR",
        },
    ];

    let where_clause = generate_filter_clauses(filters);

    let count_query = format!(
        "SELECT COUNT(*) FROM fc_rf_by_city f {} {}",
        JOIN_QUERY, where_clause
    );
    let total_count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    let offset = pagination.offset(total_count as usize);
    let page_size = pagination.page_size(total_count as usize);
    let query = format!(
        "{} {} {} ORDER BY city_name  LIMIT {} OFFSET {}",
        SELECT_QUERY, JOIN_QUERY, where_clause, page_size, offset
    );

    let result = sqlx::query_as::<_, FcByCityResponse>(&query)
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
            eprintln!("Failed to fetch fc_rf_by_city: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
