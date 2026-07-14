use std::time::Duration;

use reqwest::{StatusCode, blocking::Client as HttpClient};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use tracing::{debug, warn};
use url::Url;

use crate::error::RpcError;

#[derive(Clone)]
pub struct RpcConnection {
    pub url: Url,
    pub username: String,
    pub password: String,
    pub timeout: Duration,
}

#[derive(Clone)]
pub struct BitcoinRpcClient {
    connection: RpcConnection,
    http: HttpClient,
}

impl BitcoinRpcClient {
    pub fn new(connection: RpcConnection) -> Result<Self, RpcError> {
        let http = HttpClient::builder()
            .timeout(connection.timeout)
            .build()
            .map_err(RpcError::BuildClient)?;

        Ok(Self { connection, http })
    }

    pub fn call<T>(&self, method: &str, params: Value) -> Result<T, RpcError>
    where
        T: DeserializeOwned,
    {
        self.call_at(self.connection.url.clone(), method, params)
    }

    pub fn call_wallet<T>(&self, wallet: &str, method: &str, params: Value) -> Result<T, RpcError>
    where
        T: DeserializeOwned,
    {
        let endpoint = wallet_endpoint(&self.connection.url, wallet)?;
        self.call_at(endpoint, method, params)
            .map_err(|error| map_wallet_error(wallet, error))
    }

    pub fn get_blockchain_info(&self) -> Result<BlockchainInfo, RpcError> {
        self.call("getblockchaininfo", json!([]))
    }

    pub fn get_wallet_info(&self, wallet: &str) -> Result<WalletInfo, RpcError> {
        let metadata: WalletMetadata = self.call_wallet(wallet, "getwalletinfo", json!([]))?;
        let balances: WalletBalances = self.call_wallet(wallet, "getbalances", json!([]))?;

        Ok(WalletInfo {
            walletname: metadata.walletname,
            trusted_balance: balances.mine.trusted,
            unconfirmed_balance: balances.mine.untrusted_pending,
            immature_balance: balances.mine.immature,
            txcount: metadata.txcount,
        })
    }

    pub fn get_balance(&self, wallet: &str) -> Result<serde_json::Number, RpcError> {
        self.call_wallet(wallet, "getbalance", json!([]))
    }

    pub fn get_new_address(&self, wallet: &str) -> Result<String, RpcError> {
        self.call_wallet(wallet, "getnewaddress", json!(["", "bech32"]))
    }

    fn call_at<T>(&self, endpoint: Url, method: &str, params: Value) -> Result<T, RpcError>
    where
        T: DeserializeOwned,
    {
        debug!(rpc_method = method, rpc_endpoint = %endpoint, "sending Bitcoin Core RPC request");

        let request = JsonRpcRequest {
            jsonrpc: "1.0",
            id: "bitcoin-cli-rs",
            method,
            params,
        };

        let response = self
            .http
            .post(endpoint)
            .basic_auth(&self.connection.username, Some(&self.connection.password))
            .json(&request)
            .send()
            .map_err(RpcError::Transport)?;

        let status = response.status();
        if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
            warn!(rpc_method = method, %status, "Bitcoin Core rejected RPC authentication");
            return Err(RpcError::Authentication);
        }

        if !status.is_success() {
            debug!(rpc_method = method, %status, "Bitcoin Core returned a non-success HTTP status");
        }

        match response.json::<JsonRpcResponse<T>>() {
            Ok(envelope) => envelope.into_result(),
            Err(_) if !status.is_success() => Err(RpcError::HttpStatus(status)),
            Err(error) => Err(RpcError::InvalidResponse(error)),
        }
    }
}

fn map_wallet_error(wallet: &str, error: RpcError) -> RpcError {
    match error {
        RpcError::BitcoinCore {
            code: -18, message, ..
        } => RpcError::WalletUnavailable {
            wallet: wallet.to_owned(),
            message,
        },
        error => error,
    }
}

fn wallet_endpoint(base_url: &Url, wallet: &str) -> Result<Url, RpcError> {
    let mut url = base_url.clone();
    url.path_segments_mut()
        .map_err(|_| RpcError::InvalidBaseUrl)?
        .pop_if_empty()
        .push("wallet")
        .push(wallet);
    Ok(url)
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'a str,
    params: Value,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
}

impl<T> JsonRpcResponse<T> {
    fn into_result(self) -> Result<T, RpcError> {
        if let Some(error) = self.error {
            return Err(RpcError::BitcoinCore {
                code: error.code,
                message: error.message,
                data: error.data,
            });
        }

        self.result.ok_or(RpcError::MissingResult)
    }
}

#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i64,
    message: String,
    #[serde(default)]
    data: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub pruned: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WalletInfo {
    pub walletname: String,
    pub trusted_balance: serde_json::Number,
    pub unconfirmed_balance: serde_json::Number,
    pub immature_balance: serde_json::Number,
    pub txcount: u64,
}

#[derive(Debug, Deserialize)]
struct WalletMetadata {
    walletname: String,
    txcount: u64,
}

#[derive(Debug, Deserialize)]
struct WalletBalances {
    mine: MineBalances,
}

#[derive(Debug, Deserialize)]
struct MineBalances {
    trusted: serde_json::Number,
    untrusted_pending: serde_json::Number,
    immature: serde_json::Number,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_percent_encoded_wallet_endpoint() {
        let base = Url::parse("http://127.0.0.1:18443").expect("valid test URL");
        let endpoint = wallet_endpoint(&base, "team wallet").expect("valid wallet endpoint");

        assert_eq!(
            endpoint.as_str(),
            "http://127.0.0.1:18443/wallet/team%20wallet"
        );
    }

    #[test]
    fn converts_json_rpc_error_to_typed_error() {
        let response = JsonRpcResponse::<Value> {
            result: None,
            error: Some(JsonRpcError {
                code: -18,
                message: "Requested wallet does not exist or is not loaded".to_owned(),
                data: None,
            }),
        };

        let error = response.into_result().expect_err("response should fail");
        assert!(matches!(error, RpcError::BitcoinCore { code: -18, .. }));
    }

    #[test]
    fn maps_wallet_not_found_to_wallet_unavailable() {
        let error = RpcError::BitcoinCore {
            code: -18,
            message: "Requested wallet does not exist or is not loaded".to_owned(),
            data: None,
        };

        let error = map_wallet_error("missing-wallet", error);

        assert!(matches!(
            error,
            RpcError::WalletUnavailable { wallet, .. } if wallet == "missing-wallet"
        ));
    }

    #[test]
    fn deserializes_bitcoin_core_v30_wallet_responses() {
        let metadata: WalletMetadata = serde_json::from_value(json!({
            "walletname": "bitcoin-cli-rs-wallet",
            "txcount": 2
        }))
        .expect("valid wallet metadata");
        let balances: WalletBalances = serde_json::from_value(json!({
            "mine": {
                "trusted": 1.25,
                "untrusted_pending": 0.1,
                "immature": 50.0
            }
        }))
        .expect("valid wallet balances");

        assert_eq!(metadata.walletname, "bitcoin-cli-rs-wallet");
        assert_eq!(metadata.txcount, 2);
        assert_eq!(balances.mine.trusted.to_string(), "1.25");
        assert_eq!(balances.mine.untrusted_pending.to_string(), "0.1");
        assert_eq!(balances.mine.immature.to_string(), "50.0");
    }
}
