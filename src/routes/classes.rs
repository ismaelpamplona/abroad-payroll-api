use crate::handlers::classes;
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
            patch(classes::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(classes::list))
        .route("/:id", get(classes::get_by_id))
        .route("/", post(classes::save))
        .route("/:id", delete(classes::delete))
}
