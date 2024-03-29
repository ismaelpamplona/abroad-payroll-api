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

// CREATE TABLE public.payroll_closed (
//     id uuid NOT NULL DEFAULT uuid_generate_v4(),
//     closed_id uuid NOT NULL,
//     payroll_item uuid NULL,
//     person_id uuid NULL,
//     value float8 NOT NULL,
//     "date" date NOT NULL,
// 	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     CONSTRAINT payroll_closed_pkey PRIMARY KEY (id)
// );

// CREATE TABLE public.people (
//     id uuid NOT NULL DEFAULT uuid_generate_v4(),
//     name varchar(300) NOT NULL,
//     role_id uuid NOT NULL,
//     class_id uuid NOT NULL,
//     cpf varchar(11) NOT NULL,
//     bank_id uuid NOT NULL,
//     bank_agency varchar(20) NOT NULL,
//     bank_agency_account varchar(20) NOT NULL,
//     has_retention_bonus BOOLEAN NOT NULL DEFAULT false,
//     payroll_brl_pss float8 NOT NULL,
//     created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     updated_at timestamp NULL,
//     e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
//     CONSTRAINT people_cpf_check CHECK (length(cpf) = 11),
//     CONSTRAINT people_cpf_key UNIQUE (cpf),
//     CONSTRAINT people_pkey PRIMARY KEY (id)
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
        pr.id,
        pr.rf_receipt_id,
        pr.payroll_closed_item_id,
        pc.person_id,
        p.name as person_name,
        p.role_id,
        r.name,
        p.class_id,
        c.name,
        fr.value as fc_by_role,
        fc.value as fc_by_city,
    FROM paid_rf_receipts pr";

pub const JOINS_QUERY: &str = "
    JOIN payroll_closed pc ON pc.id = pr.payroll_closed_item_id
    JOIN people p ON p.id = pc.person_id
    JOIN roles r ON r.id = p.role_id
    JOIN classes c ON c.id = p.class_id
    JOIN fc_rf_by_roles fr ON fr.role_id = p.role_id AND fr.class_id = p.class_id
    JOIN fc_rf_by_city fc ON fc.role_id = p.role_id AND fc.class_id = p.class_id
    ";

pub const RETURN_QUERY: &str = "
    RETURNING m.id, m.person_id,  
        (SELECT name FROM people WHERE id = m.person_id) as person_name,
        m.payroll_item as payroll_item_id,
        (SELECT code FROM meta_payroll_items WHERE id = m.payroll_item) as payroll_item_code, 
        (SELECT short_name FROM meta_payroll_items WHERE id = m.payroll_item) as payroll_item_short_name, 
        (SELECT description FROM meta_payroll_items WHERE id = m.payroll_item) as payroll_item_description, 
        m.value, m.start_date, m.end_date, m.e_tag";
