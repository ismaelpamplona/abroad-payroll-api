use crate::handlers::roles_classes_indexes;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(roles_classes_indexes::list))
}
