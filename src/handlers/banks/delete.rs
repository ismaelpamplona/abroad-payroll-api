use super::*;
use crate::response::ErrorDetail;
use axum::extract::Path;

pub async fn delete(Extension(pool): Extension<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let delete_query = sqlx::query("DELETE FROM banks WHERE id = $1")
        .bind(&id)
        .execute(&pool);

    match delete_query.await {
        Ok(delete_result) => {
            if delete_result.rows_affected() == 0 {
                let not_found_id = ErrorDetail {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: format!("Bank with id {} does not exist.", id),
                };
                let res: ApiResponse<String> = ApiResponse::error(not_found_id);
                return (StatusCode::NOT_FOUND, Json(res)).into_response();
            }

            let meta = Meta {
                total_count: Some(1),
                page: Some(1),
                page_size: Some(1),
            };
            let response = ApiResponse::success_list(vec![] as Vec<BankResponse>, meta);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            eprintln!("Failed to delete bank: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
