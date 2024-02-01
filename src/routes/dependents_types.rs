use crate::handlers::dependents_types;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(dependents_types::list))
        .route("/:id", get(dependents_types::get_by_id))
        .route("/", post(dependents_types::save))
        .route("/:id", delete(dependents_types::delete))
}
