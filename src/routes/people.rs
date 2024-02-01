use crate::handlers::people;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(people::list))
        .route("/:id", get(people::get_by_id))
        .route("/", post(people::save))
        .route("/:id", delete(people::delete))
}
