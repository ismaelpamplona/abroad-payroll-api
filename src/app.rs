use crate::handlers;
use crate::routes::{banks, cities, classes, countries, roles, roles_classes_indexes};
use axum::{routing::get, Extension, Router};
use sqlx::PgPool;
use std::{env, net::SocketAddr};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("running");

    let db_url = env::var("DB_URL").expect("DB_URL must be set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(handlers::get_root)) // Root route
        .nest("/classes", classes::routes())
        .nest("/roles", roles::routes())
        .nest("/roles-classes-indexes", roles_classes_indexes::routes())
        .nest("/banks", banks::routes())
        .nest("/countries", countries::routes())
        .nest("/cities", cities::routes())
        .layer(Extension(pool));

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
