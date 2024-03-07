use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ProgressiveIncomeTaxPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE progressive_income_tax_table
        SET from_value = $1, to_value = $2, tax_rate = $3, parcel_deductible_value = $4, law = $5, law_date = $6, start_from = $7
        WHERE id = $8 RETURNING *",
    );

    let result = sqlx::query_as::<_, ProgressiveIncomeTaxResponse>(&query)
        .bind(&payload.from_value)
        .bind(&payload.to_value)
        .bind(&payload.tax_rate)
        .bind(&payload.parcel_deductible_value)
        .bind(&payload.law)
        .bind(&payload.law_date)
        .bind(&payload.start_from)
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
            eprintln!(
                "Failed to update progressive_income_tax_table details: {}",
                error
            );
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
