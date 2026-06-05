use axum::{extract::State, Json};

use crate::{
    api::dto::{CreateWatchAddressRequest, WatchAddressResponse},
    state::AppState,
    utils::address::normalize_address,
};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn create_watch_address(
    State(state): State<AppState>,
    Json(req): Json<CreateWatchAddressRequest>
) -> Json<WatchAddressResponse> {
    tracing::info!("app={} create watch address", state.app_name);
    let normalized_address = normalize_address(&req.address);

    let resp = WatchAddressResponse {
        id: "1".to_string(),
        address: normalized_address,
        chain_id: req.chain_id,
        label: req.label,
    };

    Json(resp)
}