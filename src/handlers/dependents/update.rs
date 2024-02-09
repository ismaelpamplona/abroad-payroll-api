use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<DependentPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE dependents 
        SET name = $1, person_id = $2, type_id = $3, ir = $4, birth_date = $5, start_date = $6, end_date = $7
        WHERE id = $8 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, DependentResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.person_id)
        .bind(&payload.type_id)
        .bind(&payload.ir)
        .bind(&payload.birth_date)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
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
            eprintln!("Failed to update dependent details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
