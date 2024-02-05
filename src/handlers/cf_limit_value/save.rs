use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CFLimitPayload>,
) -> impl IntoResponse
where
    CFLimitPayload: DeserializeOwned + Send,
{
    let query = format!(
        "INSERT INTO cf_limit_value (value, law, law_date) VALUES ($1, $2, $3) RETURNING *"
    );

    let result = sqlx::query_as::<_, CFLimitResponse>(&query)
        .bind(&payload.value)
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
            eprintln!("Failed to save cf_limit_value details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
