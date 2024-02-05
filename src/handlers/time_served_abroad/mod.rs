use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
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
pub struct TimeServedAbroadPayload {
    city_id: Uuid,
    person_id: Uuid,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    law: String,
    law_date: NaiveDate,
}

#[derive(Serialize, FromRow)]
pub struct TimeServedAbroadResponse {
    id: Uuid,
    city_id: Uuid,
    city_name: String,
    person_id: Uuid,
    person_name: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    law: String,
    law_date: NaiveDate,
    e_tag: Uuid,
}

#[derive(Deserialize)]
pub struct TimeServedAbroadFilter {
    city_names: Option<String>,
    people_names: Option<String>,
}

pub const SELECT_QUERY: &str = "
    SELECT 
        tsa.id,
        tsa.city_id,
        c.name as city_name,
        tsa.person_id,
        p.name as person_name,
        tsa.start_date,
        tsa.end_date,
        tsa.law,
        tsa.law_date,
        tsa.e_tag
    FROM time_served_abroad tsa";

pub const JOIN_QUERY: &str = "
    JOIN cities c ON tsa.city_id = c.id
    JOIN people p ON tsa.person_id = p.id
";
