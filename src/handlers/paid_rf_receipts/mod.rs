use crate::response::{get_error_status, handle_error, ApiResponse, Meta};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub mod get_by_id;
pub mod list;

pub use get_by_id::get_by_id;
pub use list::list;

// CREATE TABLE public.paid_rf_receipts (
//     id uuid NOT NULL DEFAULT uuid_generate_v4(),
//     rf_receipt_id uuid NOT NULL UNIQUE,
//     payroll_closed_item_id uuid NOT NULL UNIQUE,
//     created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     CONSTRAINT paid_rf_receipts_pkey PRIMARY KEY (id)
// );

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct ManualEntryPayload {
    rf_receipt_id: Uuid,
    payroll_closed_item_id: Uuid,
}

#[derive(Serialize, FromRow, Debug)]
pub struct ManualEntriesResponse {
    id: Uuid,
    person_id: Uuid,
    person_name: String,
    payroll_item_id: Uuid,
    payroll_item_code: String,
    payroll_item_short_name: String,
    payroll_item_description: String,
    value: f64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct ManualEntryFilter {
    codes: Option<String>,
    names: Option<String>,
    item_names: Option<String>,
    item_descs: Option<String>,
    start_dates: Option<String>,
    end_dates: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        m.id,
        m.person_id,
        p.name as person_name,
        m.payroll_item as payroll_item_id,
        i.code as payroll_item_code,
        i.short_name as payroll_item_short_name,
        i.description as payroll_item_description,
        m.value,
        m.start_date,
        m.end_date,
        m.e_tag
    FROM manual_entries m";

pub const JOINS_QUERY: &str = "
    JOIN people p ON p.id = m.person_id
    JOIN meta_payroll_items i ON i.id = m.payroll_item
    ";

pub const RETURN_QUERY: &str = "
    RETURNING m.id, m.person_id,  
        (SELECT name FROM people WHERE id = m.person_id) as person_name,
        m.payroll_item as payroll_item_id,
        (SELECT code FROM meta_payroll_items WHERE id = m.payroll_item) as payroll_item_code, 
        (SELECT short_name FROM meta_payroll_items WHERE id = m.payroll_item) as payroll_item_short_name, 
        (SELECT description FROM meta_payroll_items WHERE id = m.payroll_item) as payroll_item_description, 
        m.value, m.start_date, m.end_date, m.e_tag";
