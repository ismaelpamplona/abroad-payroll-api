use crate::response::{get_error_status, handle_error, ApiResponse, Meta};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub mod delete;
pub mod get_by_id;
pub mod list;
pub mod save;
pub mod update;

pub use delete::delete;
pub use get_by_id::get_by_id;
pub use list::list;
pub use save::save;
pub use update::update;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct RolePayload {
    name: String,
}

#[derive(Serialize, FromRow)]
pub struct RoleResponse {
    id: Uuid,
    name: String,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct RoleFilter {
    names: Option<String>,
}
