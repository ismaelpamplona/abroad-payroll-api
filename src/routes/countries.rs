use crate::handlers::countries;
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
            patch(countries::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(countries::list))
        .route("/:id", get(countries::get_by_id))
        .route("/", post(countries::save))
        .route("/:id", delete(countries::delete))
}
