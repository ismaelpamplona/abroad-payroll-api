use crate::response::{ApiResponse, ErrorDetail};
use axum::{
    body::Body,
    extract::{Extension, Path},
    http::{header::HeaderMap, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, FromRow, Debug)]
pub struct ETagResponse {
    e_tag: String,
}

pub async fn check_etag(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let table: Option<String> = request.extensions().get::<String>().cloned();
    println!("{:?}", table);
    if let Some(if_match) = headers.get("If-Match") {
        if let Ok(if_match_str) = if_match.to_str() {
            let query = format!("SELECT * FROM {} WHERE id = $1", table.unwrap());

            let current_etag_result = sqlx::query_as::<_, ETagResponse>(&query)
                .bind(&id)
                .fetch_one(&pool)
                .await;

            match current_etag_result {
                Ok(current_etag_row) => {
                    let current_etag = current_etag_row.e_tag;

                    if if_match_str == current_etag {
                        let response = next.run(request).await;
                        return response.into_response();
                    } else {
                        let error = ApiResponse::<()>::error(ErrorDetail {
                            code: StatusCode::PRECONDITION_FAILED.as_u16(),
                            message: "ETag does not match.".to_string(),
                        });
                        return (StatusCode::PRECONDITION_FAILED, Json(error)).into_response();
                    }
                }
                Err(_) => {
                    let error = ApiResponse::<()>::error(ErrorDetail {
                        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to fetch ETag.".to_string(),
                    });
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response();
                }
            }
        }
    }

    let error = ApiResponse::<()>::error(ErrorDetail {
        code: StatusCode::BAD_REQUEST.as_u16(),
        message: "If-Match header is required.".to_string(),
    });
    return (StatusCode::BAD_REQUEST, Json(error)).into_response();
}
