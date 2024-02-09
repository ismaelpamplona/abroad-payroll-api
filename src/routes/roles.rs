use crate::handlers::roles;
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
            patch(roles::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(roles::list))
        .route("/:id", get(roles::get_by_id))
        .route("/", post(roles::save))
        .route("/:id", delete(roles::delete))
}
