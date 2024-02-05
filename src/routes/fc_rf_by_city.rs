use crate::handlers::fc_rf_by_city;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(fc_rf_by_city::list))
        .route("/:id", get(fc_rf_by_city::get_by_id))
        .route("/", post(fc_rf_by_city::save))
        .route("/:id", delete(fc_rf_by_city::delete))
}
