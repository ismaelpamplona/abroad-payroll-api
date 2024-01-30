use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod get_by_id;
pub mod list;

pub use get_by_id::get_by_id;
pub use list::list;

#[derive(Serialize, FromRow)]
pub struct RoleResponse {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
pub struct RoleFilter {
    names: Option<String>,
}
