use axum::response::IntoResponse;

pub mod test;

pub async fn get_root() -> impl IntoResponse {
    "Welcome to the API!".to_string()
}
