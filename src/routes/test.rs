use crate::handlers::test;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(test::list))
}
