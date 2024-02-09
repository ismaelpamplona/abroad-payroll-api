use crate::handlers::dependents_types;
use crate::middlewares::check_etag::check_etag;

use axum::{
    middleware::from_fn,
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/:id",
            patch(dependents_types::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(dependents_types::list))
        .route("/:id", get(dependents_types::get_by_id))
        .route("/", post(dependents_types::save))
        .route("/:id", delete(dependents_types::delete))
}
