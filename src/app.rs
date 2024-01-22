use crate::handlers;

use crate::routes::test;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::{env, net::SocketAddr};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("running");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(handlers::get_root))
        .nest("/test", test::routes());

    let port: u16 = env::var("APP_PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
