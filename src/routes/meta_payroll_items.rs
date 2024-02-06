use crate::handlers::meta_payroll_items;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(meta_payroll_items::list))
        .route("/:id", get(meta_payroll_items::get_by_id))
        .route("/", post(meta_payroll_items::save))
        .route("/:id", delete(meta_payroll_items::delete))
}
