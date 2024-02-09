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

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CityPayload {
    name: String,
    country_id: Uuid,
    latitude: f64,
    longitude: f64,
    fc_rb: f64,
    fc_irex: f64,
}

#[derive(Serialize, FromRow)]
pub struct CityResponse {
    id: Uuid,
    name: String,
    country_id: Uuid,
    country_name: String,
    latitude: f64,
    longitude: f64,
    fc_rb: f64,
    fc_irex: f64,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct CityFilter {
    names: Option<String>,
    country_names: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        c.id as id,
        c.name as name,
        c.country_id,
        co.name as country_name,
        c.latitude,
        c.longitude,
        c.fc_rb,
        c.fc_irex,
        c.e_tag
    FROM cities c";

pub const JOINS_QUERY: &str = "JOIN countries co ON c.country_id = co.id";

pub const RETURN_QUERY: &str = "
    RETURNING cities.id, cities.name, cities.country_id, 
        (SELECT name FROM countries WHERE id = cities.country_id) as country_name, 
        cities.latitude, cities.longitude, cities.fc_rb, cities.fc_irex, cities.e_tag";
