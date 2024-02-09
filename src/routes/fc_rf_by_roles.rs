use crate::handlers::fc_rf_by_roles;
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
            patch(fc_rf_by_roles::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(fc_rf_by_roles::list))
        .route("/:id", get(fc_rf_by_roles::get_by_id))
        .route("/", post(fc_rf_by_roles::save))
        .route("/:id", delete(fc_rf_by_roles::delete))
}
