use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CountryPayload>,
) -> impl IntoResponse {
    let query = "UPDATE countries SET name = $1 WHERE id = $2 RETURNING *";

    let result = sqlx::query_as::<_, CountryResponse>(&query)
        .bind(&payload.name)
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
            eprintln!("Failed to update country details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
