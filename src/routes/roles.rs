use crate::handlers::roles;
use axum::middleware::from_fn;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::{Pool, Postgres};

pub fn routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(roles::list))
        .layer(Extension(pool))
}
