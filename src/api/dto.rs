use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateWatchAddressRequest {
    pub address: String,
    pub chain_id: u64,
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WatchAddressResponse {
    pub id: String,
    pub address: String,
    pub chain_id: u64,
    pub label: Option<String>,
}