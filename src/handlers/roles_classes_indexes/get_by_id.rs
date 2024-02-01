use super::*;
use axum::extract::Path;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = format!(
        "SELECT 
            rci.id as id,
            r.role_id,
            r.name as role_name,
            r.class_id,
            c.name as class_name,
            rci.fc_rb,
            rci.fc_irex
        FROM 
            roles_classes_indexes rci
        JOIN 
            roles r ON rci.role_id = r.id
        JOIN 
            classes c ON rci.class_id = c.id
        WHERE 
            rci.id = $1"
    );

    let result = sqlx::query_as::<_, RoleClassIndexResponse>(&query)
        .bind(&id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(item) => {
            let response = ApiResponse::success_one(item);
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
