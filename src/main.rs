mod api;
mod chain;
mod config;
mod db;
mod error;
mod scanner;
mod state;
mod utils;

use std::sync::Arc;

use api::routes::create_router;
use chain::mock::MockChainClient;
use config::Config;
use db::repository::ping_db;
use dotenvy::dotenv;
use scanner::service::Scanner;
use sqlx::postgres::PgPoolOptions;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    ping_db(&pool).await?;

    let (tx_events, _rx) = tokio::sync::broadcast::channel(100);

    let state = AppState {
        pool: pool.clone(),
        tx_events: tx_events.clone(),
    };

    let scanner = Scanner {
        pool: pool.clone(),
        chain_id: config.chain_id,
        client: Arc::new(MockChainClient),
        scan_interval: config.scan_interval,
        confirmations: config.confirmations,
        tx_events: tx_events.clone(),
    };

    tokio::spawn(async move {
        scanner.run().await;
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&config.server_address).await?;

    tracing::info!("server listening on {}", config.server_address);

    axum::serve(listener, app).await?;

    Ok(())
}