use axum::response::IntoResponse;

pub mod banks;
pub mod cities;
pub mod classes;
pub mod countries;
pub mod roles;
pub mod roles_classes_indexes;

pub async fn get_root() -> impl IntoResponse {
    "Welcome to the API!".to_string()
}
