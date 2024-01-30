use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod list;

pub use list::list;

#[derive(Serialize, FromRow)]
pub struct RoleClassIndexResponse {
    rci_id: Uuid,
    role_id: Uuid,
    role_name: String,
    class_id: Uuid,
    class_name: String,
    fc_rb: f64,
    fc_irex: f64,
}

#[derive(Deserialize)]
pub struct RoleClassFilter {
    role_names: Option<String>,
    class_names: Option<String>,
}
