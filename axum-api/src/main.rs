use anyhow::Context;
use axum_api::{
    http,
    utils::{db::Db, init, telemetry},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing();

    tracing::debug!("Initializing configuration");
    init::config()?;

    tracing::debug!("Initializing db pool");
    let db = Db::new().await.context("Failed to initialize db pool")?;
    tracing::debug!("Running migrations");
    db.migrate().await.context("Failed to run migrations")?;

    tracing::info!("Starting AXUM API server on port 5000");
    http::serve(db.pool).await;

    Ok(())
}
