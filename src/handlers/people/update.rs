use super::*;
use axum::extract::Path;

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<PersonPayload>,
) -> impl IntoResponse {
    let query = format!(
        "UPDATE people 
        SET name = $1, role_id = $2, class_id = $3, cpf = $4, bank_id = $5, bank_agency = $6, bank_agency_account = $7
        WHERE id = $8 {}",
        RETURN_QUERY
    );

    let result = sqlx::query_as::<_, PersonResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.role_id)
        .bind(&payload.class_id)
        .bind(&payload.cpf)
        .bind(&payload.bank_id)
        .bind(&payload.bank_agency)
        .bind(&payload.bank_agency_account)
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
            eprintln!("Failed to update person details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
