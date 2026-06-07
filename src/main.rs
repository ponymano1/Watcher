mod error;
mod scanner;
mod utils;
mod state;
mod api;
mod config;
mod db;

use api::routes::create_routes;
use state::AppState;
use tokio::net::TcpListener;
use config::Config;
use db::repository::ping_db;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let config = Config::from_env();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;


    ping_db(&pool).await?;

    let state = AppState {
        app_name: "hahaha-coming".to_string(),
    };

    let app = create_routes(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}
