use super::*;
use axum::extract::Path;

pub async fn get_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let query = format!(
        "SELECT 
            d.id as id,
            d.name as name,
            d.person_id,
            people.name as person_name,
            d.birth_date,
            d.start_date,
            d.end_date,
            d.type_id,
            d.type_id,
            t.name as type_name,
            d.ir,
        FROM dependents d
        JOIN people p ON d.person_id = p.id 
        JOIN dependents_types t ON d.type_id = t.id 
        WHERE 
            d.id = $1"
    );

    let result = sqlx::query_as::<_, DependentResponse>(&query)
        .bind(&id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(item) => {
            let response = ApiResponse::success_one(item);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to fetch cities: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
