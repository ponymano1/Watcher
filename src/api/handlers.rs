use axum::{
    Json,
    extract::{Path, Query, State},
};

use serde::Deserialize;

use crate::{
    api::dto::{CreateWatchAddressRequest, WatchAddressResponse},
    db::{
        models::{Transaction, WatchAddress},
        repository::{insert_watch_address, list_transactions_by_address, list_watch_addresses},
    },
    error::AppError,
    state::AppState,
    utils::address::normalize_address,
    utils::address::validate_address,
};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn create_watch_address(
    State(state): State<AppState>,
    Json(req): Json<CreateWatchAddressRequest>,
) -> Result<Json<WatchAddress>, AppError> {
    let address = validate_address(&req.address)?;

    let saved = insert_watch_address(
        &state.pool,
        &address,
        req.chain_id as i64,
        req.label.as_deref(),
    )
    .await?;

    Ok(Json(saved))
}

pub async fn get_watch_addresses(
    State(state): State<AppState>,
) -> Result<Json<Vec<WatchAddress>>, AppError> {
    let addresses = list_watch_addresses(&state.pool).await?;
    Ok(Json(addresses))
}

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    chain_id: i64,
}

pub async fn get_transaction_by_address(
    State(state): State<AppState>,
    Path(address): Path<String>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<Vec<Transaction>>, AppError> {
    let address = validate_address(&address)?;
    let transactions = list_transactions_by_address(&state.pool, query.chain_id, &address).await?;
    Ok(Json(transactions))
}
