use crate::handlers::classes;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(classes::list))
        .route("/:id", get(classes::get_by_id))
        .route("/", post(classes::save))
}
