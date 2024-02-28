use crate::handlers::payroll_simulation;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(payroll_simulation::calc))
        .route("/close", post(payroll_simulation::close))
}
