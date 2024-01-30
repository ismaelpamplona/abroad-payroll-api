use crate::handlers::classes;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(classes::list))
        .route("/:id", get(classes::get_by_id))
}
