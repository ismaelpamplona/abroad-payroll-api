use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

use crate::response::{get_error_status, handle_error, ApiResponse, Meta};

pub mod delete;
pub mod get_by_id;
pub mod list;
pub mod save;

pub use delete::delete;
pub use get_by_id::get_by_id;
pub use list::list;
pub use save::save;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DependentTypePayload {
    name: String,
    value: f64,
}

#[derive(Serialize, FromRow, Debug)]
pub struct DependentTypeResponse {
    id: Uuid,
    name: String,
    value: f64,
}

#[derive(Deserialize)]
pub struct DependentTypeFilter {
    names: Option<String>,
}
