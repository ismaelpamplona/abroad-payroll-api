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
pub mod update;

pub use delete::delete;
pub use get_by_id::get_by_id;
pub use list::list;
pub use save::save;
pub use update::update;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct RFPaymentReceiptsPayload {
    person_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
    rate: f64,
    value: f64,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct RFPaymentReceiptsResponse {
    id: Uuid,
    person_id: Uuid,
    person_name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    rate: f64,
    value: f64,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct RFPaymentReceiptsFilter {
    names: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT
        rf.id,
        rf.person_id,
        p.name as person_name,
        rf.start_date,
        rf.end_date,
        rf.rate,
        rf.value,
        rf.e_tag
    FROM rf_payment_receipts rf";

pub const JOINS_QUERY: &str = "
    JOIN people p ON rf.person_id = p.id 
";

pub const RETURN_QUERY: &str = "
    RETURNING rf.id, rf.person_id, 
        (SELECT name FROM people WHERE id = rf.person_id) as person_name, 
        rf.start_date, rf.end_date,  rf.rate, rf.value, rf.e_tag";
