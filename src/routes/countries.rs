use crate::handlers::countries;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(countries::list))
        .route("/:id", get(countries::get_by_id))
}
