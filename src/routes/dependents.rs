use crate::handlers::dependents;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(dependents::list))
        .route("/:id", get(dependents::get_by_id))
        .route("/", post(dependents::save))
        .route("/:id", delete(dependents::delete))
}
