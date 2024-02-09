use crate::handlers::time_served_abroad;
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
            patch(time_served_abroad::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(time_served_abroad::list))
        .route("/:id", get(time_served_abroad::get_by_id))
        .route("/", post(time_served_abroad::save))
        .route("/:id", delete(time_served_abroad::delete))
}
