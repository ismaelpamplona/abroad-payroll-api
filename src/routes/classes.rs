use crate::handlers::classes;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(classes::list))
        .route("/:id", get(classes::get_by_id))
        .route("/", post(classes::save))
        .route("/:id", delete(classes::delete))
}
