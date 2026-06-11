use sqlx::PgPool;
use tokio::sync::broadcast;
use crate::scanner::events::NewTransactionEvent;
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tx_events: broadcast::Sender<NewTransactionEvent>,
}
