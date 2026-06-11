use axum::{
    Router,
    routing::{get, post},
};

use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    api::handlers::{
        create_watch_address, 
        get_transaction_by_address, 
        get_watch_addresses, 
        health,
        ws_transactions,
    },
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route(
            "/watch-addresses",
            post(create_watch_address).get(get_watch_addresses),
        )
        .route("/transactions/{address}", get(get_transaction_by_address))
        .route("/ws/transactions", get(ws_transactions))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
