use crate::handlers::meta_payroll_items;
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
            patch(meta_payroll_items::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(meta_payroll_items::list))
        .route("/:id", get(meta_payroll_items::get_by_id))
        .route("/", post(meta_payroll_items::save))
        .route("/:id", delete(meta_payroll_items::delete))
}
