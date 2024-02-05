use crate::handlers::cf_limit_value;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(cf_limit_value::list))
        .route("/:id", get(cf_limit_value::get_by_id))
        .route("/", post(cf_limit_value::save))
        .route("/:id", delete(cf_limit_value::delete))
}
