use crate::handlers::fc_rf_by_roles;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(fc_rf_by_roles::list))
        .route("/:id", get(fc_rf_by_roles::get_by_id))
        .route("/", post(fc_rf_by_roles::save))
        .route("/:id", delete(fc_rf_by_roles::delete))
}
