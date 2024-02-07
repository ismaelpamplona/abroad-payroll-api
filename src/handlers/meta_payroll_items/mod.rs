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

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
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
    id: Uuid,
    code: String,
    short_name: String,
    description: String,
    transaction_type: TransactionType,
    e_tag: Uuid,
}

#[derive(Deserialize)]
pub struct PayrollItemsFilter {
    codes: Option<String>,
    names: Option<String>,
    descs: Option<String>,
    types: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT
        m.id,
        m.code,
        m.short_name,
        m.description,
        m.transaction_type,
        m.e_tag,

    FROM meta_payroll_items m";

// id uuid NOT NULL DEFAULT uuid_generate_v4(),
// code varchar(30) NULL,
// short_name varchar(10) NOT NULL,
// description varchar(100) NOT NULL,
// "transaction_type" public."transaction_type" NOT NULL,
// created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
// updated_at timestamp NULL,
// e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
// CONSTRAINT meta_payroll_items_pkey PRIMARY KEY (id),
// CONSTRAINT unique_code UNIQUE (code),
// CONSTRAINT unique_short_name UNIQUE (short_name)
