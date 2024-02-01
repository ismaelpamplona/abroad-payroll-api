use super::*;
use crate::response::{generate_filter_clauses, Filter, Pagination};
use axum::extract::Query;

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<PeopleFilter>,
) -> impl IntoResponse {
    let filters = vec![
        Filter {
            name: "name",
            val: filters.names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "role_name",
            val: filters.role_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "class_name",
            val: filters.class_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "cpf",
            val: filters.cpfs.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "bank_name",
            val: filters.bank_names.as_ref(),
            conj: "OR",
        },
        Filter {
            name: "agency_number",
            val: filters.agency_numbers.as_ref(),
            conj: "OR",
        },
    ];
    let where_clause = generate_filter_clauses(filters);

    let count_query = format!(
        "SELECT COUNT(*) 
         FROM public.people p
         JOIN public.roles r ON p.role = r.id
         JOIN public.classes c ON p.class = c.id
         JOIN public.banks b ON p.bank = b.id
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
            p.id as id,
            p.name as name,
            p.role as role_id,
            r.name as role_name,
            p.class as class_id,
            c.name as class_name,
            p.cpf,
            p.bank as bank_id,
            b.name as bank_name,
            b.number as bank_number,
            p.bank_agency,
            p.bank_agency_account,
            p.created_at,
            p.updated_at,
            p.e_tag
        FROM public.people p
        JOIN public.roles r ON p.role = r.id
        JOIN public.classes c ON p.class = c.id
        JOIN public.banks b ON p.bank = b.id
        {} 
        ORDER BY p.name 
        LIMIT {} OFFSET {}",
        where_clause, page_size, offset
    );

    let result = sqlx::query_as::<_, PersonResponse>(&query)
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
            eprintln!("Failed to fetch people: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
