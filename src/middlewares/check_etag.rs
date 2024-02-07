use axum::{
    body::Body,
    extract::{Extension, Path},
    http::{header::HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
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
) -> Result<Response, StatusCode> {
    if let Some(if_match) = headers.get("If-Match") {
        if let Ok(if_match_str) = if_match.to_str() {
            let table_name = "banks";
            let query =
                sqlx::query_as::<_, ETagResponse>("SELECT * FROM banks WHERE id = $1").bind(&id);
            let current_etag_result = query.fetch_one(&pool).await;

            match current_etag_result {
                Ok(current_etag_row) => {
                    let current_etag = current_etag_row.e_tag;
                    println!("{:?}", current_etag);
                    println!("{:?}", if_match_str);

                    if if_match_str == current_etag {
                        let response = next.run(request).await;
                        return Ok(response);
                    } else {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
                Err(err) => {
                    eprintln!("Failed to fetch table: {}", err);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    }

    Err(StatusCode::BAD_REQUEST)
}
