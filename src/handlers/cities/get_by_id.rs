use super::*;
use axum::extract::Path;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = format!(
        "SELECT 
            cities.id as id,
            cities.name as name,
            countries.id as country_id,
            countries.name as country,
            cities.latitude,
            cities.longitude,
            cities.fc_rb,
            cities.fc_irex
        FROM 
            cities
        JOIN 
            countries ON cities.country = countries.id
        WHERE 
            cities.id = $1"
    );

    let result = sqlx::query_as::<_, CityResponse>(&query)
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
            eprintln!("Failed to fetch cities: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
