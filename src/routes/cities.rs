use crate::handlers::cities;
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
            patch(cities::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(cities::list))
        .route("/:id", get(cities::get_by_id))
        .route("/", post(cities::save))
        .route("/:id", delete(cities::delete))
}
