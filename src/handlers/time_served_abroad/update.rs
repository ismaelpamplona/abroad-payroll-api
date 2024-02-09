use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<TimeServedAbroadPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE time_served_abroad AS tsa
        SET city_id = $1, person_id = $2, start_date = $3, end_date = $4, law = $5, law_date = $6
        WHERE id = $7 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, TimeServedAbroadResponse>(&query)
        .bind(&payload.city_id)
        .bind(&payload.person_id)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
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
            eprintln!("Failed to update period served abroad details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
