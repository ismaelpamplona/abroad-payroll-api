use crate::handlers::cf_limit_exchange_rate;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(cf_limit_exchange_rate::list))
        .route("/:id", get(cf_limit_exchange_rate::get_by_id))
        .route("/", post(cf_limit_exchange_rate::save))
        .route("/:id", delete(cf_limit_exchange_rate::delete))
}
