use crate::handlers::cities;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(cities::list))
        .route("/:id", get(cities::get_by_id))
        .route("/", post(cities::save))
}
