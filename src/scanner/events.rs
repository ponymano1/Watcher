use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct NewTransactionEvent {
    pub chain_id: i64,
    pub tx_hash: String,
    pub block_number: i64,
    pub from_address: String,
    pub to_address: Option<String>,
    pub value_wei: String,
}