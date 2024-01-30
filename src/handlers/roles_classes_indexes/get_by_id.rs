use super::*;
use axum::extract::Path;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = format!(
        "SELECT 
            rci.id as id,
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
        WHERE 
            rci.id = $1"
    );

    let result = sqlx::query_as::<_, RoleClassIndexResponse>(&query)
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
            eprintln!("Failed to fetch roles_classes_indexes: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
