use crate::handlers::dependents;
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
            patch(dependents::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(dependents::list))
        .route("/:id", get(dependents::get_by_id))
        .route("/", post(dependents::save))
        .route("/:id", delete(dependents::delete))
}
