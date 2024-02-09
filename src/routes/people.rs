use crate::handlers::people;
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
            patch(people::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(people::list))
        .route("/:id", get(people::get_by_id))
        .route("/", post(people::save))
        .route("/:id", delete(people::delete))
}
