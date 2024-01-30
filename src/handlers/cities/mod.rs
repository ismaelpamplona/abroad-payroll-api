use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub mod get_by_id;
pub mod list;

pub use get_by_id::get_by_id;
pub use list::list;

#[derive(Serialize, FromRow)]
pub struct CityResponse {
    id: Uuid,
    name: String,
    country_id: Uuid,
    country: String,
    latitude: f64,
    longitude: f64,
    fc_rb: f64,
    fc_irex: f64,
}

#[derive(Deserialize)]
pub struct CityFilter {
    names: Option<String>,
    countries: Option<String>,
}
