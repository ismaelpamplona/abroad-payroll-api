use axum::response::IntoResponse;

pub mod banks;
pub mod cf_limit_exchange_rate;
pub mod cf_limit_value;
pub mod cities;
pub mod classes;
pub mod countries;
pub mod dependents;
pub mod dependents_types;
pub mod fc_rf_by_city;
pub mod fc_rf_by_roles;
pub mod people;
pub mod rf_payment_receipts;
pub mod roles;
pub mod roles_classes_indexes;
pub mod time_served_abroad;

pub async fn get_root() -> impl IntoResponse {
    "Welcome to the API!".to_string()
}
