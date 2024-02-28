use crate::handlers::manual_entries;
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
            patch(manual_entries::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(manual_entries::list))
        .route("/:id", get(manual_entries::get_by_id))
        .route("/", post(manual_entries::save))
        .route("/:id", delete(manual_entries::delete))
}
