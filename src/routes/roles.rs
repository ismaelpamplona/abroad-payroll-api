use crate::handlers::roles;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(roles::list))
        .route("/:id", get(roles::get_by_id))
}
