use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WatchAddress {
    pub id: i64,
    pub address: String,
    pub chain_id: i64,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: i64,
    pub chain_id: i64,
    pub tx_hash: String,
    pub block_number: i64,
    pub from_address: String,
    pub to_address: Option<String>,
    pub value_wei: String,
    pub created_at: DateTime<Utc>,
}
