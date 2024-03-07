use crate::handlers::progressive_income_tax_table;
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
            patch(progressive_income_tax_table::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(progressive_income_tax_table::list))
        .route("/:id", get(progressive_income_tax_table::get_by_id))
        .route("/", post(progressive_income_tax_table::save))
        .route("/:id", delete(progressive_income_tax_table::delete))
}
