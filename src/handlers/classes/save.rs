use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ClassPayload>,
) -> impl IntoResponse
where
    ClassPayload: DeserializeOwned + Send,
{
    let query = "INSERT INTO classes (name) VALUES ($1) RETURNING *";

    let result = sqlx::query_as::<_, ClassResponse>(&query)
        .bind(&payload.name)
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
            eprintln!("Failed to save class details: {}", error);

            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
