mod error;
mod scanner;
mod utils;
mod state;
mod api;

use api::routes::create_routes;
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        app_name: "web3-rust-watcher-hahaha".to_string(),
    };

    let app = create_routes(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
