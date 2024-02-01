use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{FromRow, PgPool};
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

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DependentPayload {
    name: String,
    person_id: Uuid,
    type_id: Uuid,
    birth_date: NaiveDateTime,
    start_date: NaiveDateTime,
    end_date: Option<NaiveDateTime>,
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
    birth_date: NaiveDateTime,
    start_date: NaiveDateTime,
    end_date: Option<NaiveDateTime>,
    e_tag: Uuid,
}

#[derive(Deserialize)]
pub struct DependentFilter {
    names: Option<String>,
    people: Option<String>,
    ir: Option<String>,
}
