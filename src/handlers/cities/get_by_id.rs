use super::*;
use axum::extract::Path;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = format!(
        "SELECT 
            c.id as id,
            c.name as name,
            c.country_id,
            countries.name as country_name,
            c.latitude,
            c.longitude,
            c.fc_rb,
            c.fc_irex
        FROM 
            cities c
        JOIN 
            countries ON c.country_id = countries.id
        WHERE 
            c.id = $1"
    );

    let result = sqlx::query_as::<_, CityResponse>(&query)
        .bind(&id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(item) => {
            let response = ApiResponse::success_one(item);
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
