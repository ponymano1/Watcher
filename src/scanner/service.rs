use std::{
    collections::HashSet,
    sync::Arc,
    time::Duration,
};

use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::scanner::events::NewTransactionEvent;
use crate::{
    chain::client::ChainClient,
    db::repository::{
        get_last_synced_block,
        insert_transaction,
        list_watched_address_strings,
        update_last_synced_block,
    },
    error::AppError,
};

pub struct Scanner<C> {
    pub pool: PgPool,
    pub chain_id: i64,
    pub client: Arc<C>,
    pub scan_interval: Duration,
    pub confirmations: u64,
    pub tx_events: broadcast::Sender<NewTransactionEvent>,
}

impl<C> Scanner<C>
where
    C: ChainClient,
{
    pub async fn scan_once(&self) -> Result<(), AppError> {
        let last_synced = get_last_synced_block(
            &self.pool,
            self.chain_id,
        )
        .await?;

        let latest = self.client.latest_block_number().await?;

        let safe_latest = latest.saturating_sub(self.confirmations);

        if safe_latest <= last_synced as u64 {
            tracing::info!(
                latest,
                safe_latest,
                last_synced,
                "scanner has no new safe blocks"
            );
            return Ok(());
        }

        let watched = list_watched_address_strings(
            &self.pool,
            self.chain_id,
        )
        .await?;

        let watched_set: HashSet<String> =
            watched.into_iter().map(|a| a.to_lowercase()).collect();

        for block_number in (last_synced as u64 + 1)..=safe_latest {
            let block = self
                .client
                .get_block_with_txs(block_number)
                .await?;

            tracing::info!(
                block_number = block.number,
                tx_count = block.transactions.len(),
                "scanning block"
            );

            for tx in block.transactions {
                let from = tx.from.to_lowercase();
                let to = tx.to.as_ref().map(|x| x.to_lowercase());

                let matched =
                    watched_set.contains(&from)
                    || to
                        .as_ref()
                        .map(|addr| watched_set.contains(addr))
                        .unwrap_or(false);

                if matched {
                    let inserted = insert_transaction(
                        &self.pool,
                        self.chain_id,
                        &tx,
                    )
                    .await?;
                    if inserted {
                        let _ = self.tx_events.send(NewTransactionEvent {
                            chain_id: self.chain_id,
                            tx_hash: tx.hash.clone(),
                            block_number: block_number as i64,
                            from_address: from,
                            to_address: tx.to.clone(),
                            value_wei: tx.value_wei.clone(),
                        });
                    }
                }
            }

            update_last_synced_block(
                &self.pool,
                self.chain_id,
                block_number as i64,
            )
            .await?;
        }

        Ok(())
    }
}

impl<C> Scanner<C>
where
    C: ChainClient + 'static,
{
    pub async fn run(self) {
        let mut interval = tokio::time::interval(self.scan_interval);

        loop {
            interval.tick().await;

            if let Err(err) = self.scan_once().await {
                tracing::error!("scanner error: {:?}", err);
            }
        }
    }
}