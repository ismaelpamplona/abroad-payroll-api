use crate::response::{get_error_status, handle_error, ApiResponse, Meta};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
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
    pub e_tag: String,
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
    bank_numbers: Option<String>,
    bank_agencies: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        p.id as id,
        p.name as name,
        p.role_id,
        r.name as role_name,
        p.class_id,
        c.name as class_name,
        p.cpf,
        p.bank_id,
        b.name as bank_name,
        b.number as bank_number,
        p.bank_agency,
        p.bank_agency_account,
        p.created_at,
        p.updated_at,
        p.e_tag
    FROM people p";

pub const JOINS_QUERY: &str = "
    JOIN roles r ON p.role_id = r.id
    JOIN classes c ON p.class_id = c.id
    JOIN banks b ON p.bank_id = b.id";

pub const RETURN_QUERY: &str = "
    RETURNING people.id, people.name, people.role_id, 
        (SELECT name FROM roles WHERE id = people.role_id) as role_name, 
        people.class_id,
        (SELECT name FROM classes WHERE id = people.class_id) as class_name, 
        people.cpf, people.bank_id, 
        (SELECT name FROM banks WHERE id = people.bank_id) as bank_name, 
        (SELECT number FROM banks WHERE id = people.bank_id) as bank_number, 
        people.bank_agency, people.bank_agency_account, 
        people.created_at, people.updated_at, people.e_tag";
