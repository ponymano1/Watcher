use crate::{
    chain::client::{ChainBlock, ChainClient, ChainTransaction},
    error::AppError,
};

pub struct MockChainClient;

#[async_trait::async_trait]
impl ChainClient for MockChainClient {
    async fn latest_block_number(&self) -> Result<u64, AppError> {
        Ok(5)
    }

    async fn get_block_with_txs(&self, block_number: u64) -> Result<ChainBlock, AppError> {
        Ok(ChainBlock {
            number: block_number,
            transactions: vec![
                ChainTransaction {
                    hash: format!("0xmocktx{}", block_number),
                    block_number,
                    from: "0x1111111111111111111111111111111111111111".to_string(),
                    to: Some("0x2222222222222222222222222222222222222222".to_string()),
                    value_wei: "1000000000000000000".to_string(),
                },
                ChainTransaction {
                    hash: format!("0xother{}", block_number),
                    block_number,
                    from: "0x3333333333333333333333333333333333333333".to_string(),
                    to: Some("0x4444444444444444444444444444444444444444".to_string()),
                    value_wei: "123".to_string(),
                },
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_chain_client() {
        let client = MockChainClient;

        let latest = client.latest_block_number().await.unwrap();
        assert_eq!(latest, 5);

        let block = client.get_block_with_txs(1).await.unwrap();
        assert_eq!(block.number, 1);
        assert_eq!(block.transactions.len(), 2);
    }
}
