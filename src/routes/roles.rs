use crate::handlers::roles;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(roles::list))
        .route("/:id", get(roles::get_by_id))
        .route("/", post(roles::save))
        .route("/:id", delete(roles::delete))
}
