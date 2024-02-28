use crate::handlers::paid_rf_receipts;
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
            patch(paid_rf_receipts::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(paid_rf_receipts::list))
        .route("/:id", get(paid_rf_receipts::get_by_id))
        .route("/", post(paid_rf_receipts::save))
        .route("/:id", delete(paid_rf_receipts::delete))
}
