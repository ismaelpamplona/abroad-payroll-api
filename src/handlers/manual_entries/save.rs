use super::*;
use axum::extract::{Extension, Path};

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ManualEntryPayload>,
) -> impl IntoResponse {
    let query = format!(
        "INSERT INTO manual_entries AS m (person_id, payroll_item, value, start_date, end_date) 
        VALUES ($1, $2, $3, $4, $5) {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, ManualEntriesResponse>(&query)
        .bind(&payload.person_id)
        .bind(&payload.payroll_item)
        .bind(&payload.value)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
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
            eprintln!("Failed to save manual entry details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
