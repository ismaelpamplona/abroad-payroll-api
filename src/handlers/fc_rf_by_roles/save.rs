use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<FcByRolesPayload>,
) -> impl IntoResponse
where
    FcByRolesPayload: DeserializeOwned + Send,
{
    let query = format!(
        "INSERT INTO fc_rf_by_roles as f (role_id, class_id, value, law, law_date) 
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING f.id, f.role_id, 
                  (SELECT name FROM roles WHERE id = f.role_id) as role_name, 
                  f.class_id,
                  (SELECT name FROM classes WHERE id = f.class_id) as class_name, 
                  f.value, f.law, f.law_date, f.e_tag"
    );

    let result = sqlx::query_as::<_, FcByRolesResponse>(&query)
        .bind(&payload.role_id)
        .bind(&payload.class_id)
        .bind(&payload.value)
        .bind(&payload.law)
        .bind(&payload.law_date)
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
            eprintln!("Failed to save fc_rf_by_roles details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
