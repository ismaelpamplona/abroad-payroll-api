use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod get_by_id;
pub mod list;
pub mod save;

pub use get_by_id::get_by_id;
pub use list::list;
pub use save::save;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CountryPayload {
    name: String,
}

#[derive(Serialize, FromRow)]
pub struct CountryResponse {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
pub struct CountryFilter {
    names: Option<String>,
}
