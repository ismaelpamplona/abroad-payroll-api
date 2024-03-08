use super::*;
use crate::response::{generate_filter_clauses, Filter, Operator, Pagination};
use axum::extract::Query;

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<PeopleFilter>,
) -> impl IntoResponse {
    let filters = vec![
        Filter {
            name: "p.name",
            op: Operator::ILIKE,
            val: filters.names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "r.name",
            op: Operator::ILIKE,
            val: filters.role_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "c.name",
            op: Operator::ILIKE,
            val: filters.class_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "cpf",
            op: Operator::ILIKE,
            val: filters.cpfs.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "b.name",
            op: Operator::ILIKE,
            val: filters.bank_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "b.number",
            op: Operator::ILIKE,
            val: filters.bank_numbers.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "p.bank_agency",
            op: Operator::ILIKE,
            val: filters.bank_agencies.as_ref(),
            conj: "OR",
        },
    ];
    let where_clause = generate_filter_clauses(filters);

    let count_query = format!(
        "SELECT COUNT(*) AS total_records
        FROM ({} {} {} {}) AS subquery;",
        SELECT_QUERY, JOINS_QUERY, GROUP_BY_QUERY, where_clause
    );

    let total_count: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    dbg!(total_count);
    let offset = pagination.offset(total_count as usize);
    let page_size = pagination.page_size(total_count as usize);

    let query = format!(
        "{} {} {} {} ORDER BY p.name LIMIT {} OFFSET {}",
        SELECT_QUERY, JOINS_QUERY, GROUP_BY_QUERY, where_clause, page_size, offset
    );

    let result = sqlx::query_as::<_, PersonResponse>(&query)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(items) => {
            dbg!(&items);
            let meta = Meta {
                total_count: Some(total_count as usize),
                page: Some(pagination.page.unwrap_or(1)),
                page_size: Some(page_size),
            };
            let response = ApiResponse::success_list(items, meta);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to fetch people: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
