use axum::{
    routing::{get, post},
    Router,
};

use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::{
    api::handlers::{create_watch_address, health},
    state::AppState,
};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/watch-address", post(create_watch_address))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().permissive())
        .with_state(state)
}
