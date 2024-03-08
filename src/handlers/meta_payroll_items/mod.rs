use crate::response::{get_error_status, handle_error, ApiResponse, Meta};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Type};
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

#[derive(Debug, Type, Serialize, Deserialize, PartialEq, Clone)]
#[sqlx(type_name = "transaction_type", rename_all = "snake_case")]
pub enum TransactionType {
    Credit,
    Debit,
}
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct PayrollItemsPayload {
    code: String,
    short_name: String,
    description: String,
    transaction_type: TransactionType,
}

#[derive(Serialize, FromRow, Debug)]
pub struct PayrollItemsResponse {
    pub id: Uuid,
    pub code: String,
    pub short_name: String,
    pub description: String,
    pub transaction_type: TransactionType,
    pub consider_for_ir: bool,
    pub e_tag: String,
}

#[derive(Deserialize)]
pub struct PayrollItemsFilter {
    codes: Option<String>,
    names: Option<String>,
    descs: Option<String>,
    types: Option<String>,
    ir: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT * FROM meta_payroll_items m";
