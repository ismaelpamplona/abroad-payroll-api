use crate::handlers::roles_classes_indexes;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(roles_classes_indexes::list))
        .route("/:id", get(roles_classes_indexes::get_by_id))
        .route("/", post(roles_classes_indexes::save))
}
