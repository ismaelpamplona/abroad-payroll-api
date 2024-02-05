use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
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

#[derive(Deserialize, Serialize, FromRow)]
pub struct CFLimitExchangeRatePayload {
    law: String,
    law_date: NaiveDate,
    value: f64,
}

#[derive(Serialize, FromRow, Debug)]
pub struct CFLimitExchangeRateResponse {
    id: Uuid,
    value: f64,
    law: String,
    law_date: NaiveDate,
    e_tag: Uuid,
}

#[derive(Deserialize)]
pub struct CFLimitExchangeRateFilter {
    laws: Option<String>,
}

pub const SELECT_QUERY: &str = "SELECT * FROM cf_limit_exchange_rate l";
