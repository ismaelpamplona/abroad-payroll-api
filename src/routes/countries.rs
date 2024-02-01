use crate::handlers::countries;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(countries::list))
        .route("/:id", get(countries::get_by_id))
        .route("/", post(countries::save))
        .route("/:id", delete(countries::delete))
}
