use super::*;
use crate::response::{generate_filter_clauses, Filter, Pagination};
use axum::extract::Query;

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<DependentFilter>,
) -> impl IntoResponse {
    let filters = vec![
        Filter {
            name: "name",
            val: filters.names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "person",
            val: filters.people.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "ir",
            val: filters.ir.as_ref(),
            conj: "OR",
        },
    ];

    let where_clause = generate_filter_clauses(filters);

    let count_query = format!(
        "SELECT COUNT(*) 
         FROM dependents d
         JOIN people p ON d.person_id = p.id
         JOIN dependents_types t ON d.type_id = t.id 
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
            d.id,
            d.name,
            d.person_id,
            p.name as person_name,
            d.birth_date,
            d.start_date,
            d.end_date,
            d.type_id,
            t.name as type_name,
            d.ir,
            d.e_tag
        FROM dependents d
        JOIN people p ON d.person_id = p.id 
        JOIN dependents_types t ON d.type_id = t.id 
        {} ORDER BY name LIMIT {} OFFSET {}",
        where_clause, page_size, offset
    );

    let result = sqlx::query_as::<_, DependentResponse>(&query)
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
            eprintln!("Failed to fetch dependents: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
