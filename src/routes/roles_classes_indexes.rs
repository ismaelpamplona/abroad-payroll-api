use crate::handlers::roles_classes_indexes;
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
            patch(roles_classes_indexes::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(roles_classes_indexes::list))
        .route("/:id", get(roles_classes_indexes::get_by_id))
        .route("/", post(roles_classes_indexes::save))
        .route("/:id", delete(roles_classes_indexes::delete))
}
