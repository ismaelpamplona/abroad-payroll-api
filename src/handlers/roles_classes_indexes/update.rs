use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<RoleClassIndexPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE roles_classes_indexes AS rci
        SET role_id = $1, class_id = $2, fc_rb = $3, fc_irex = $4
        WHERE id = $5 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, RoleClassIndexResponse>(&query)
        .bind(&payload.role_id)
        .bind(&payload.class_id)
        .bind(&payload.fc_rb)
        .bind(&payload.fc_irex)
        .bind(&id)
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
            eprintln!("Failed to update roles/class index details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
