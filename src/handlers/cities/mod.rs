use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

use crate::response::{get_error_status, handle_error, ApiResponse, Meta};

pub mod get_by_id;
pub mod list;
pub mod save;

pub use get_by_id::get_by_id;
pub use list::list;
pub use save::save;

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
