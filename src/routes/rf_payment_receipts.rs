use crate::handlers::rf_payment_receipts;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(rf_payment_receipts::list))
        .route("/:id", get(rf_payment_receipts::get_by_id))
        .route("/", post(rf_payment_receipts::save))
        .route("/:id", delete(rf_payment_receipts::delete))
}
