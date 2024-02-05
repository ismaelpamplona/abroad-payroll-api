use crate::handlers;
use crate::routes;
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
        .nest("/classes", routes::classes::routes())
        .nest("/roles", routes::roles::routes())
        .nest(
            "/roles-classes-indexes",
            routes::roles_classes_indexes::routes(),
        )
        .nest("/banks", routes::banks::routes())
        .nest("/countries", routes::countries::routes())
        .nest("/cities", routes::cities::routes())
        .nest("/people", routes::people::routes())
        .nest("/dependents-types", routes::dependents_types::routes())
        .nest("/dependents", routes::dependents::routes())
        .nest("/time-served-abroad", routes::time_served_abroad::routes())
        .nest("/fc-rf-by-roles", routes::fc_rf_by_roles::routes())
        .nest("/fc-rf-by-city", routes::fc_rf_by_city::routes())
        .nest(
            "/cf-limit-exchange-rate",
            routes::cf_limit_exchange_rate::routes(),
        )
        .nest("/cf-limit", routes::cf_limit_value::routes())
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
