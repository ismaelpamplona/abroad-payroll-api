use crate::handlers::cities;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(cities::list))
}
