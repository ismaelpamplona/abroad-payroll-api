use axum::response::IntoResponse;

pub async fn list() -> impl IntoResponse {
    "Test List!".to_string()
}
