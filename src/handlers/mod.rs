use axum::response::IntoResponse;

pub mod banks;
pub mod cities;
pub mod classes;
pub mod countries;
pub mod dependents;
pub mod dependents_types;
pub mod people;
pub mod roles;
pub mod roles_classes_indexes;
pub mod time_served_abroad;

pub async fn get_root() -> impl IntoResponse {
    "Welcome to the API!".to_string()
}
