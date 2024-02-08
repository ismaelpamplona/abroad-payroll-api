use crate::handlers::banks;
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
            patch(banks::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(banks::list))
        .route("/:id", get(banks::get_by_id))
        .route("/", post(banks::save))
        .route("/:id", delete(banks::delete))
}
