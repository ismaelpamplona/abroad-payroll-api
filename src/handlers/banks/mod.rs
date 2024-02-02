use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

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
pub struct BankPayload {
    name: String,
    number: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct BankResponse {
    id: Uuid,
    name: String,
    number: String,
    e_tag: Uuid,
}

#[derive(Deserialize)]
pub struct BankFilter {
    names: Option<String>,
    numbers: Option<String>,
}
