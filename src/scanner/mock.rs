use crate::error::AppError;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::select;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct MockNewTransactionEvent {
    pub hash: String,
    pub address: String,
}

pub fn create_mock_event_bus() -> broadcast::Sender<MockNewTransactionEvent> {
    let (tx, _) = broadcast::channel(100);
    tx
}

pub async fn fetch_latest_block_mock() -> u64 {
    tokio::time::sleep(Duration::from_millis(300)).await;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    timestamp % 500
}

pub async fn call_rpc_mock_with_timeout() -> Result<u64, AppError> {
    let result = tokio::time::timeout(Duration::from_secs(3), fetch_latest_block_mock())
        .await
        .map_err(|_| AppError::RPC("timeout".to_string()))?;
    Ok(result)
}

pub async fn run_scanner_mock(
    tx_events: broadcast::Sender<MockNewTransactionEvent>,
    cancel: CancellationToken,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut counter = 0u64;
    loop {
        interval.tick().await;
        select! {
            _ = cancel.cancelled() => {
                println!("Scanner cancelled");
                return;
            }
            _ = interval.tick() => {
                match call_rpc_mock_with_timeout().await {
                    Ok(latest_block) => {
                        println!("Latest block number: {}", latest_block);
                        counter += 1;
                        let event = MockNewTransactionEvent{
                            hash: format!("0xmocktx{}", counter),
                            address: "0x1111111111111111111111111111111111111111".to_string(),
                        };

                        let receiver_count = tx_events.send(event).unwrap_or(0);

                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_mock_event_bus() {
        let event_bus = create_mock_event_bus();
        assert_eq!(event_bus.receiver_count(), 0);
    }

    #[tokio::test]
    async fn test_fetch_latest_block_mock() {
        let latest_block = fetch_latest_block_mock().await;
        assert!(latest_block > 0);
    }

    #[tokio::test]
    async fn test_call_rpc_mock_with_timeout() {
        let result = call_rpc_mock_with_timeout().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_run_scanner_mock() {
        let event_bus = create_mock_event_bus();
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        let mut rx = event_bus.subscribe();
        tokio::spawn(async move {
            run_scanner_mock(event_bus, cancel_clone).await;
        });

        let mut events = Vec::new();
        for _ in 0..5 {
            if let Ok(event) = rx.recv().await {
                events.push(event);
            }
        }
        cancel.cancel();
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(!events.is_empty());
        println!("Events: {:?}", events);
    }
}
