use super::*;
use crate::response::{ErrorDetail, SuccessDelete};
use axum::extract::Path;

pub async fn delete(Extension(pool): Extension<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let delete_query = sqlx::query("DELETE FROM time_served_abroad WHERE id = $1")
        .bind(&id)
        .execute(&pool);

    match delete_query.await {
        Ok(delete_result) => {
            if delete_result.rows_affected() == 0 {
                let not_found_id = ErrorDetail {
                    code: StatusCode::NOT_FOUND.as_u16(),
                    message: format!("Time_served_abroad with id {} does not exist.", id),
                };
                let res: ApiResponse<String> = ApiResponse::error(not_found_id);
                return (StatusCode::NOT_FOUND, Json(res)).into_response();
            }

            let res = ApiResponse::<SuccessDelete>::success_delete(id);
            (StatusCode::OK, res).into_response()
        }
        Err(error) => {
            eprintln!("Failed to delete time_served_abroad: {}", error);
            let err = handle_error(&error);

            let res: ApiResponse<String> = ApiResponse::error(err);
            (get_error_status(&error), Json(res)).into_response()
        }
    }
}
