mod api;
mod app_state;
mod domain;
mod provider;
mod storage;
mod telemetry;

use std::{env, sync::Arc};

use app_state::AppState;
use provider::{adapter::ProviderAdapter, fake_provider::FakeProvider};
use storage::{repository::ExecutionRepository, sqlite::init_sqlite};
use tokio::net::TcpListener;
use tracing::info;

use crate::telemetry::logging::init_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let database_path =
        env::var("DATABASE_URL").unwrap_or_else(|_| "data/rust_reliability_series.db".to_string());
    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);

    let sqlite = init_sqlite(&database_path)?;
    let repository = Arc::new(ExecutionRepository::new(sqlite));
    let provider_adapter = ProviderAdapter::new(FakeProvider::default());
    let state = AppState::new(repository, provider_adapter);
    let app = api::routes::router(state);

    let address = format!("{host}:{port}");
    info!(%address, %database_path, "starting rust reliability series api");

    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    info!("shutdown signal received");
}
