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

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DependentPayload {
    name: String,
    person_id: Uuid,
    type_id: Uuid,
    birth_date: NaiveDate,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    ir: bool,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DependentResponse {
    id: Uuid,
    name: String,
    person_id: Uuid,
    person_name: String,
    type_id: Uuid,
    type_name: String,
    ir: bool,
    birth_date: NaiveDate,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct DependentFilter {
    names: Option<String>,
    people: Option<String>,
    ir: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT
        d.id,
        d.name,
        d.person_id,
        p.name as person_name,
        d.birth_date,
        d.start_date,
        d.end_date,
        d.type_id,
        t.name as type_name,
        d.ir,
        d.e_tag
    FROM dependents d";

pub const JOINS_QUERY: &str = "
    JOIN people p ON d.person_id = p.id 
    JOIN dependents_types t ON d.type_id = t.id 
";
