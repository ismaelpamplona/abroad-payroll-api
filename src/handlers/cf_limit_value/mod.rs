use crate::response::{get_error_status, handle_error, ApiResponse, Meta};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub mod delete;
pub mod get_by_id;
pub mod list;
pub mod save;

pub use delete::delete;
pub use get_by_id::get_by_id;
pub use list::list;
pub use save::save;

#[derive(Deserialize, Serialize, FromRow)]
pub struct CFLimitPayload {
    law: String,
    law_date: NaiveDate,
    value: f64,
}

#[derive(Serialize, FromRow, Debug)]
pub struct CFLimitResponse {
    id: Uuid,
    value: f64,
    law: String,
    law_date: NaiveDate,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct CFLimitFilter {
    laws: Option<String>,
}

pub const SELECT_QUERY: &str = "SELECT * FROM cf_limit_value l";
