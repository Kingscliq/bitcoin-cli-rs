use reqwest::StatusCode;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("failed to build the HTTP client: {0}")]
    BuildClient(reqwest::Error),

    #[error("could not connect to Bitcoin Core: {0}")]
    Transport(reqwest::Error),

    #[error("Bitcoin Core rejected the RPC credentials")]
    Authentication,

    #[error("Bitcoin Core returned HTTP status {0}")]
    HttpStatus(StatusCode),

    #[error("Bitcoin Core returned an invalid JSON-RPC response: {0}")]
    InvalidResponse(reqwest::Error),

    #[error("Bitcoin Core RPC error {code}: {message}")]
    BitcoinCore {
        code: i64,
        message: String,
        data: Option<Value>,
    },

    #[error("wallet `{wallet}` does not exist or is not loaded: {message}")]
    WalletUnavailable { wallet: String, message: String },

    #[error("Bitcoin Core returned neither a result nor an error")]
    MissingResult,

    #[error("the configured RPC URL cannot be used as a base URL")]
    InvalidBaseUrl,
}
