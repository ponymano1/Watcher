use crate::db::models::{Transaction, WatchAddress};
use sqlx::PgPool;

pub async fn ping_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

pub async fn insert_watch_address(
    pool: &PgPool,
    address: &str,
    chain_id: i64,
    label: Option<&str>,
) -> Result<WatchAddress, sqlx::Error> {
    sqlx::query_as::<_, WatchAddress>(
        r#"
        INSERT INTO watch_addresses (address, chain_id, label)
        VALUES ($1, $2, $3)
        ON CONFLICT (address, chain_id)
        DO UPDATE SET label = EXCLUDED.label
        RETURNING id, address, chain_id, label, created_at
        "#,
    )
    .bind(address)
    .bind(chain_id)
    .bind(label)
    .fetch_one(pool)
    .await
}

pub async fn list_watch_addresses(pool: &PgPool) -> Result<Vec<WatchAddress>, sqlx::Error> {
    sqlx::query_as::<_, WatchAddress>(
        r#"
        SELECT id, address, chain_id, label, created_at
        FROM watch_addresses
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn list_transactions_by_address(
    pool: &PgPool,
    chain_id: i64,
    address: &str,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, chain_id, tx_hash, block_number, from_address, to_address, value_wei, created_at
        FROM transactions
        WHERE chain_id = $1 
            AND (from_address = $2 OR to_address = $2)
        ORDER BY block_number DESC
        LIMIT 100
        "#,
    )
    .bind(chain_id)
    .bind(address)
    .fetch_all(pool)
    .await
}
