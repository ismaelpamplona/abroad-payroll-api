use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CityPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE cities 
        SET name = $1, country_id = $2, latitude = $3, longitude = $4, fc_rb = $5, fc_irex = $6
        WHERE id = $7 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, CityResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.country_id)
        .bind(&payload.latitude)
        .bind(&payload.longitude)
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
            eprintln!("Failed to update city details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
