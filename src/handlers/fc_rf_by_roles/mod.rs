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

#[derive(Deserialize, Serialize, FromRow)]
pub struct FcByRolesPayload {
    role_id: Uuid,
    class_id: Uuid,
    value: f64,
    law: String,
    law_date: NaiveDate,
}

#[derive(Serialize, FromRow, Debug)]
pub struct FcByRolesResponse {
    id: Uuid,
    role_id: Uuid,
    role_name: String,
    class_id: Uuid,
    class_name: String,
    value: f64,
    law: String,
    law_date: NaiveDate,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct FcByRolesFilter {
    role_names: Option<String>,
    class_names: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        f.id,
        f.role_id,
        r.name as role_name,
        f.class_id,
        c.name as class_name, 
        f.value,       
        f.law,
        f.law_date,
        f.e_tag
    FROM fc_rf_by_roles f";

pub const JOIN_QUERY: &str = "
    JOIN roles r ON f.role_id = r.id
    JOIN classes c ON f.class_id = c.id
";

pub const RETURN_QUERY: &str = "
    RETURNING f.id, f.role_id, 
        (SELECT name FROM roles WHERE id = f.role_id) as role_name, 
        f.class_id,
        (SELECT name FROM classes WHERE id = f.class_id) as class_name, 
        f.value, f.law, f.law_date, f.e_tag";
