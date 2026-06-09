use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database error")]
    Database(#[from] sqlx::Error),

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

#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidAddress => StatusCode::BAD_REQUEST,
            AppError::InvalidChainId => StatusCode::BAD_REQUEST,
            AppError::InvalidBlockNumber => StatusCode::BAD_REQUEST,
            AppError::RPC(_) => StatusCode::BAD_GATEWAY,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };
        let body = ErrorBody {
            message: self.to_string(),
        };
        (status, Json(body)).into_response()
    }
}
