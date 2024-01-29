use crate::handlers::banks;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(banks::list))
}
