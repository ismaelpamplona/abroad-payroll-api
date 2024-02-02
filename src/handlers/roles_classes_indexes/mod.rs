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
pub struct RoleClassIndexPayload {
    role_id: Uuid,
    class_id: Uuid,
    fc_rb: f64,
    fc_irex: f64,
}

#[derive(Serialize, FromRow)]
pub struct RoleClassIndexResponse {
    id: Uuid,
    role_id: Uuid,
    role_name: String,
    class_id: Uuid,
    class_name: String,
    fc_rb: f64,
    fc_irex: f64,
    e_tag: Uuid,
}

#[derive(Deserialize)]
pub struct RoleClassFilter {
    role_names: Option<String>,
    class_names: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        rci.id as id,
        rci.role_id,
        r.name as role_name,
        rci.class_id,
        c.name as class_name,
        rci.fc_rb,
        rci.fc_irex,
        rci.e_tag
    FROM roles_classes_indexes rci";

pub const JOIN_QUERY: &str = "
    JOIN roles r ON rci.role_id = r.id
    JOIN classes c ON rci.class_id = c.id
";
