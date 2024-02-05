use super::*;
use serde::de::DeserializeOwned;

pub async fn save(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<FcByCityPayload>,
) -> impl IntoResponse
where
    FcByCityPayload: DeserializeOwned + Send,
{
    let query = format!(
        "INSERT INTO fc_rf_by_city AS f (city_id, value, law, law_date) 
        VALUES ($1, $2, $3, $4) 
        RETURNING f.id, f.city_id, 
                  (SELECT name FROM cities c WHERE c.id = f.city_id) as city_name, 
                  (SELECT country_id FROM cities c WHERE c.id = f.city_id) as country_id,
                  (SELECT name FROM countries co WHERE co.id = (SELECT country_id FROM cities c WHERE c.id = f.city_id)) as country_name, 
                  f.value, f.law, f.law_date, f.e_tag"
    );

    let result = sqlx::query_as::<_, FcByCityResponse>(&query)
        .bind(&payload.city_id)
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
            eprintln!("Failed to save fc_rf_by_city details: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
