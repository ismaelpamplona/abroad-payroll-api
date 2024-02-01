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

#[derive(Deserialize, Serialize, FromRow)]
pub struct PersonResponse {
    pub id: Uuid,
    pub name: String,
    pub role_id: Uuid,
    pub role_name: String,
    pub class_id: Uuid,
    pub class_name: String,
    pub cpf: String,
    pub bank_id: Uuid,
    pub bank_name: String,
    pub bank_number: String,
    pub bank_agency: String,
    pub bank_agency_account: String,
    pub e_tag: Uuid,
}

#[derive(Deserialize)]
pub struct PersonPayload {
    pub name: String,
    pub role_id: Uuid,
    pub class_id: Uuid,
    pub cpf: String,
    pub bank_id: Uuid,
    pub bank_agency: String,
    pub bank_agency_account: String,
}

#[derive(Deserialize)]
pub struct PeopleFilter {
    names: Option<String>,
    role_names: Option<String>,
    class_names: Option<String>,
    cpfs: Option<String>,
    bank_names: Option<String>,
    agency_numbers: Option<String>,
}
