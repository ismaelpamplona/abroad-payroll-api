use crate::handlers::cities::RETURN_QUERY;

use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<TimeServedAbroadPayload>,
) -> impl IntoResponse {
    let query = format!(
        "INSERT INTO time_served_abroad AS tsa (city_id, person_id, start_date, end_date, law, law_date)
        VALUES ($1, $2, $3, $4, $5, $6) {}", RETURN_QUERY
    );

    let result = sqlx::query_as::<_, TimeServedAbroadResponse>(&query)
        .bind(&payload.city_id)
        .bind(&payload.person_id)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
        .bind(&payload.law)
        .bind(&payload.law_date)
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
            eprintln!("Failed to save time_served_abroad details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
