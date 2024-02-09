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
pub struct FcByCityPayload {
    city_id: Uuid,
    value: f64,
    law: String,
    law_date: NaiveDate,
}

#[derive(Serialize, FromRow, Debug)]
pub struct FcByCityResponse {
    id: Uuid,
    city_id: Uuid,
    city_name: String,
    country_id: Uuid,
    country_name: String,
    value: f64,
    law: String,
    law_date: NaiveDate,
    e_tag: String,
}

#[derive(Deserialize)]
pub struct FcByCityFilter {
    city_names: Option<String>,
    country_names: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        f.id,
        f.city_id,
        c.name as city_name,
        c.country_id,
        co.name as country_name,
        f.value,       
        f.law,
        f.law_date,
        f.e_tag
    FROM fc_rf_by_city f";

pub const JOIN_QUERY: &str = "
    JOIN cities c ON f.city_id = c.id
    JOIN countries co ON c.country_id = co.id";

pub const RETURN_QUERY: &str = "
    RETURNING f.id, f.city_id, 
        (SELECT name FROM cities c WHERE c.id = f.city_id) as city_name, 
        (SELECT country_id FROM cities c WHERE c.id = f.city_id) as country_id,
        (SELECT name FROM countries co WHERE co.id = (SELECT country_id FROM cities c WHERE c.id = f.city_id)) as country_name, 
        f.value, f.law, f.law_date, f.e_tag";
