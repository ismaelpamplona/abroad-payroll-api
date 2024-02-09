use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<FcByCityPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE fc_rf_by_city AS f
        SET city_id = $1, value = $2, law = $3, law_date = $4
        WHERE id = $5 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, FcByCityResponse>(&query)
        .bind(&payload.city_id)
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
            eprintln!("Failed to update fc_rf_by_city details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
