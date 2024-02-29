use crate::handlers::paid_rf_receipts;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(paid_rf_receipts::list))
        .route("/:id", get(paid_rf_receipts::get_by_id))
}
