use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<PersonPayload>,
) -> impl IntoResponse {
    let query = format!(
        "INSERT INTO people (name, role_id, class_id, cpf, bank_id, bank_agency, bank_agency_account)
        VALUES ($1, $2, $3, $4, $5, $6, $7) {}", RETURN_QUERY);

    let result = sqlx::query_as::<_, PersonResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.role_id)
        .bind(&payload.class_id)
        .bind(&payload.cpf)
        .bind(&payload.bank_id)
        .bind(&payload.bank_agency)
        .bind(&payload.bank_agency_account)
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
            eprintln!("Failed to save person details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
