use crate::routes;
use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    middleware::from_fn,
    middleware::Next,
    response::Response,
    Router,
};
use sqlx::PgPool;
use std::{collections::HashMap, env::var, net::SocketAddr};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let map = map_path_to_tables();

    let db_url = var("DB_URL").expect("DB_URL must be set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    let mut app = Router::new();

    for (path, (_, route)) in map.into_iter() {
        app = app.nest(format!("/{}", path).as_str(), route);
    }

    let app = app.layer(from_fn(get_path)).layer(Extension(pool));

    let port: u16 = var("APP_PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn map_path_to_tables() -> HashMap<&'static str, (&'static str, Router)> {
    #[rustfmt::skip]
    let vec = vec![
        ("banks", ("banks", routes::banks::routes())),
        ("cf-limit-exchange-rate", ("cf_limit_exchange_rate", routes::cf_limit_exchange_rate::routes())),
        ("cf-limit", ("cf_limit_value", routes::cf_limit_value::routes())),
        ("cities", ("cities", routes::cities::routes())),
        ("classes", ("classes", routes::classes::routes())),
        ("countries", ("countries", routes::countries::routes())),
        ("dependents-types", ("dependents_types", routes::dependents_types::routes())),
        ("dependents", ("dependents", routes::dependents::routes())),
        ("fc-rf-by-city", ("fc_rf_by_city", routes::fc_rf_by_city::routes())),
        ("fc-rf-by-roles", ("fc_rf_by_roles", routes::fc_rf_by_roles::routes())),
        ("payroll-items", ("meta_payroll_items", routes::meta_payroll_items::routes())),
        ("people", ("people", routes::people::routes())),
        ("rf-payment-receipts", ("rf_payment_receipts", routes::rf_payment_receipts::routes())),
        ("roles-classes-indexes", ("roles_classes_indexes", routes::roles_classes_indexes::routes())),
        ("roles", ("roles", routes::roles::routes())),
        ("time-served-abroad", ("time_served_abroad", routes::time_served_abroad::routes())),
        ("simulation", ("payroll_simulation", routes::payroll_simulation::routes())),
    ];
    vec.into_iter().collect()
}

pub async fn get_path(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_owned();
    let parts: Vec<&str> = path.split('/').collect();
    let map = map_path_to_tables();
    let mut table = String::from("");
    if let Some((tbl, _)) = map.get(parts[1]) {
        table = tbl.to_string();
    }
    let modified_request = {
        let mut request = request;
        request.extensions_mut().insert(table.clone());
        request
    };
    Ok(next.run(modified_request).await)
}
