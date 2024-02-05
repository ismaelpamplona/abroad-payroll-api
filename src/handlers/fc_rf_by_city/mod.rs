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
    e_tag: Uuid,
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
    JOIN countries co ON c.country_id = co.id
";

// id uuid NOT NULL DEFAULT uuid_generate_v4(),
// city_id uuid NOT NULL,
// value float8 NOT NULL,
// law varchar(200) NOT NULL,
// law_date date NOT NULL,
// created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
// updated_at timestamp NULL,
// e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
// CONSTRAINT fc_rf_by_city_city_key UNIQUE (city_id),
// CONSTRAINT fc_rf_by_city_pkey PRIMARY KEY (id)
