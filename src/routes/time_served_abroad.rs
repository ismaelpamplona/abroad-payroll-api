use crate::handlers::time_served_abroad;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(time_served_abroad::list))
        .route("/:id", get(time_served_abroad::get_by_id))
        .route("/", post(time_served_abroad::save))
        .route("/:id", delete(time_served_abroad::delete))
}
