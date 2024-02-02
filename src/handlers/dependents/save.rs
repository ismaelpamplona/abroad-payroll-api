use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<DependentPayload>,
) -> impl IntoResponse
where
    DependentPayload: DeserializeOwned + Send,
{
    let query =
        "INSERT INTO dependents (name, person_id, type_id, ir, birth_date, start_date, end_date)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING dependents.id, dependents.name, dependents.person_id, 
             (SELECT name FROM people WHERE id = dependents.person_id) as person_name, 
             dependents.type_id,
             (SELECT name FROM dependents_types WHERE id = dependents.type_id) as type_name, 
             dependents.ir, dependents.birth_date, dependents.start_date, dependents.end_date,  dependents.e_tag";

    let result = sqlx::query_as::<_, DependentResponse>(&query)
        .bind(&payload.name)
        .bind(&payload.person_id)
        .bind(&payload.type_id)
        .bind(&payload.ir)
        .bind(&payload.birth_date)
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
            eprintln!("Failed to save dependent details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
