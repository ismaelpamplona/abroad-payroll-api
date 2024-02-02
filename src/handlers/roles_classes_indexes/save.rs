use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RoleClassIndexPayload>,
) -> impl IntoResponse
where
    RoleClassIndexPayload: DeserializeOwned + Send,
{
    let query = format!(
        "INSERT INTO roles_classes_indexes AS rci (role_id, class_id, fc_rb, fc_irex)
        VALUES ($1, $2, $3, $4)
        RETURNING rci.id, rci.role_id, 
                  (SELECT name FROM roles WHERE id = rci.role_id) as role_name, 
                  rci.class_id,
                  (SELECT name FROM classes WHERE id = rci.class_id) as class_name, 
                  rci.fc_rb, rci.fc_irex, rci.e_tag"
    );

    let result = sqlx::query_as::<_, RoleClassIndexResponse>(&query)
        .bind(&payload.role_id)
        .bind(&payload.class_id)
        .bind(&payload.fc_rb)
        .bind(&payload.fc_irex)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => {
            let meta = Meta {
                total_count: Some(1),
                page: Some(1),
                page_size: Some(1),
            };

            let response = ApiResponse::success_list(record, meta);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to save bank details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
