use super::*;
use axum::extract::Path;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
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
        WHERE 
            p.id = $1"
    );

    let result = sqlx::query_as::<_, PersonResponse>(&query)
        .bind(&id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(item) => {
            let response = ApiResponse::success_one(item);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to fetch person: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
