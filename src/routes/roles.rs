use crate::handlers::roles;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;

pub fn routes() -> Router {
    Router::new().route("/", get(roles::list))
}
