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

// id uuid NOT NULL DEFAULT uuid_generate_v4(),
// from_value float8 NOT NULL,
// to_value float8 NOT NULL,
// tax_rate float8 NOT NULL,
// parcel_deductible_value float8 NOT NULL,
// law varchar(200) NOT NULL,
// law_date date NOT NULL,
// start_from date NOT NULL,
// created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
// updated_at timestamp NULL,
// e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
// CONSTRAINT progressive_income_tax_table_pkey PRIMARY KEY (id)

#[derive(Deserialize, Serialize, FromRow)]
pub struct ProgressiveIncomeTaxPayload {
    from_value: f64,
    to_value: f64,
    tax_rate: f64,
    parcel_deductible_value: f64,
    law: String,
    law_date: NaiveDate,
    start_from: NaiveDate,
}

#[derive(Serialize, FromRow)]
pub struct ProgressiveIncomeTaxResponse {
    id: Uuid,
    from_value: f64,
    to_value: f64,
    tax_rate: f64,
    parcel_deductible_value: f64,
    law: String,
    law_date: NaiveDate,
    start_from: NaiveDate,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct ProgressiveIncomeTaxFilter {
    start_from: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        pit.id,
        pit.from_value,
        pit.to_value,
        pit.tax_rate,
        pit.parcel_deductible_value,
        pit.law,
        pit.law_date,
        pit.start_from,
        pit.e_tag
    FROM progressive_income_tax_table pit";
