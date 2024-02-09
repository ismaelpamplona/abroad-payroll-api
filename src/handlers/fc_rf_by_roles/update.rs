use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<FcByRolesPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE fc_rf_by_roles AS f
        SET role_id = $1, class_id = $2, value = $3, law = $4, law_date = $5
        WHERE id = $6 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, FcByRolesResponse>(&query)
        .bind(&payload.role_id)
        .bind(&payload.class_id)
        .bind(&payload.value)
        .bind(&payload.law)
        .bind(&payload.law_date)
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
            eprintln!("Failed to update fc_rf_by_roles details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
