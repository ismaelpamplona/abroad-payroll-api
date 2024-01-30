use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod list;

pub use list::list;

#[derive(Serialize, FromRow)]
pub struct CountryResponse {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
pub struct CityFilter {
    names: Option<String>,
}
