use super::*;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CityPayload>,
) -> impl IntoResponse {
    let query = format!(
        "INSERT INTO cities (name, country_id, latitude, longitude, fc_rb, fc_irex)
        VALUES ($1, $2, $3, $4, $5, $6) {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, CityResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.country_id)
        .bind(&payload.latitude)
        .bind(&payload.longitude)
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
