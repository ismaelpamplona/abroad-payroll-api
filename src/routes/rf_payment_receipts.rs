use crate::handlers::rf_payment_receipts;
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
            patch(rf_payment_receipts::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(rf_payment_receipts::list))
        .route("/:id", get(rf_payment_receipts::get_by_id))
        .route("/", post(rf_payment_receipts::save))
        .route("/:id", delete(rf_payment_receipts::delete))
}
