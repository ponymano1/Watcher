use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct ChainTransaction {
    pub hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: Option<String>,
    pub value_wei: String,
}

#[derive(Debug, Clone)]
pub struct ChainBlock {
    pub number: u64,
    pub transactions: Vec<ChainTransaction>,
}

#[async_trait::async_trait]
pub trait ChainClient: Send + Sync {
    async fn latest_block_number(&self) -> Result<u64, AppError>;

    async fn get_block_with_txs(&self, block_number: u64) -> Result<ChainBlock, AppError>;
}
