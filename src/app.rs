use crate::handlers;

use crate::routes::roles;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::{env, net::SocketAddr};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("running");

    let db_url = env::var("DB_URL").expect("DB_URL must be set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(handlers::get_root))
        .nest("/roles", roles::routes(pool));

    let port: u16 = env::var("APP_PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
