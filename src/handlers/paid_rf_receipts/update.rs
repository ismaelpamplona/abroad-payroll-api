use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ManualEntryPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE manual_entries AS m
        SET person_id = $1, payroll_item = $2, value = $3, start_date = $4, end_date = $5
        WHERE id = $6{}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, ManualEntriesResponse>(&query)
        .bind(&payload.person_id)
        .bind(&payload.payroll_item)
        .bind(&payload.value)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
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
            eprintln!("Failed to update manual entry details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
