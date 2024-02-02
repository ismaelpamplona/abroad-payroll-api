use super::*;
use axum::extract::Path;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = format!("{} {} WHERE tsa.id = $1", SELECT_QUERY, JOIN_QUERY);

    let result = sqlx::query_as::<_, TimeServedAbroadResponse>(&query)
        .bind(&id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(item) => {
            let response = ApiResponse::success_one(item);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to fetch time_served_abroad: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
