use crate::db::models::{Transaction, WatchAddress};
use sqlx::PgPool;
use crate::chain::client::ChainTransaction;
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

pub async fn list_watched_address_strings(
    pool: &PgPool,
    chain_id: i64,
) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT address
        FROM watch_addresses
        WHERE chain_id = $1
        "#,
    )
    .bind(chain_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.0).collect())
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

pub async fn get_last_synced_block(
    pool: &PgPool,
    chain_id: i64,
) -> Result<i64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
   r#"
        SELECT last_synced_block
        FROM sync_state
        WHERE chain_id = $1
        "#,
    )
    .bind(chain_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0).unwrap_or(0))
}

pub async fn update_last_synced_block(
    pool: &PgPool,
    chain_id: i64,
    block_number: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sync_state (chain_id, last_synced_block, updated_at)
        VALUES ($1, $2, now())
        ON CONFLICT (chain_id)
        DO UPDATE SET 
            last_synced_block = EXCLUDED.last_synced_block,
            updated_at = now()
        "#,
    )
    .bind(chain_id)
    .bind(block_number)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_transaction(
    pool: &PgPool,
    chain_id: i64,
    tx: &ChainTransaction,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO transactions (
            chain_id,
            tx_hash, 
            block_number, 
            from_address, 
            to_address, 
            value_wei
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (chain_id, tx_hash)
        DO NOTHING
        "#,
    )
    .bind(chain_id)
    .bind(&tx.hash)
    .bind(tx.block_number as i64)
    .bind(&tx.from)
    .bind(tx.to.as_deref())
    .bind(&tx.value_wei)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}