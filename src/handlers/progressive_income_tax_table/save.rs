use super::*;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ProgressiveIncomeTaxPayload>,
) -> impl IntoResponse {
    let query = format!(
        "INSERT INTO progressive_income_tax_table (from_value, to_value, tax_rate, parcel_deductible_value, law, law_date, start_from)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    );

    let result = sqlx::query_as::<_, ProgressiveIncomeTaxResponse>(&query)
        .bind(&payload.from_value)
        .bind(&payload.to_value)
        .bind(&payload.tax_rate)
        .bind(&payload.parcel_deductible_value)
        .bind(&payload.law)
        .bind(&payload.law_date)
        .bind(&payload.start_from)
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
                "Failed to save progressive_income_tax_table details: {}",
                error
            );
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
