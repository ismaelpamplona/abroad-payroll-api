use crate::handlers::cf_limit_exchange_rate;
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
            patch(cf_limit_exchange_rate::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(cf_limit_exchange_rate::list))
        .route("/:id", get(cf_limit_exchange_rate::get_by_id))
        .route("/", post(cf_limit_exchange_rate::save))
        .route("/:id", delete(cf_limit_exchange_rate::delete))
}
