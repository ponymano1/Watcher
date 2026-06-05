use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("nvalid address")]
    InvalidAddress,

    #[error("invalid chain ID")]
    InvalidChainId,

    #[error("invalid block number")]
    InvalidBlockNumber,

    #[error("RPC error: {0}")]
    RPC(String),

    #[error("not found")]
    NotFound,
}