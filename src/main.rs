mod app;
mod handlers;
mod middlewares;
mod response;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run().await?;
    Ok(())
}
