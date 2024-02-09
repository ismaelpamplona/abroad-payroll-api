use crate::handlers::cf_limit_value;
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
            patch(cf_limit_value::update).route_layer(from_fn(check_etag)),
        )
        .route("/", get(cf_limit_value::list))
        .route("/:id", get(cf_limit_value::get_by_id))
        .route("/", post(cf_limit_value::save))
        .route("/:id", delete(cf_limit_value::delete))
}
