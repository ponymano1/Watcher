mod error;
mod scanner;
mod utils;

use tokio::select;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting scanner...");

    let event_bus = scanner::mock::create_mock_event_bus();
    let mut rx = event_bus.subscribe();
    let cancel = CancellationToken::new();

    let cancel_scanner = cancel.clone();
    let scanner_handle = tokio::spawn(async move {
        scanner::mock::run_scanner_mock(event_bus, cancel_scanner).await;
    });

    let cancel_receiver = cancel.clone();
    let receiver_handle = tokio::spawn(async move {
        loop {
            select! {
                _ = cancel_receiver.cancelled() => {
                    println!("Receiver shutting down");
                    break;
                }
                result = rx.recv() => {
                    match result {
                        Ok(event) => {
                            println!("Event: {:?}", event);
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            println!("Event channel closed");
                            break;
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            println!("Receiver lagged by {n} events");
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                }
            }
        }
    });
    tokio::time::sleep(Duration::from_secs(10)).await;
    cancel.cancel();
    let _ = tokio::join!(scanner_handle, receiver_handle);
    println!("Scanner stopped");
}
