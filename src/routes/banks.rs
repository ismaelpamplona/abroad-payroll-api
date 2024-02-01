use crate::handlers::banks;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(banks::list))
        .route("/:id", get(banks::get_by_id))
        .route("/", post(banks::save))
        .route("/:id", delete(banks::delete))
}
