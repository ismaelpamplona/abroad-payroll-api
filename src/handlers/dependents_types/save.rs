use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<DependentTypePayload>,
) -> impl IntoResponse
where
    DependentTypePayload: DeserializeOwned + Send,
{
    let query = "INSERT INTO dependents_types (name, value) VALUES ($1, $2) RETURNING *";

    let result = sqlx::query_as::<_, DependentTypeResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.value)
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
            eprintln!("Failed to save dependent_type details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
