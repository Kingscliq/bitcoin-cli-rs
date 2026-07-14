use std::{fs, path::Path, time::Duration};

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use url::Url;

use crate::{cli::Cli, rpc::RpcConnection};

const DEFAULT_RPC_URL: &str = "http://127.0.0.1:18443";
const DEFAULT_WALLET: &str = "bitcoin-cli-rs-wallet";
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

#[derive(Clone)]
pub struct AppConfig {
    rpc_url: Url,
    rpc_user: String,
    rpc_password: String,
    pub wallet: String,
    timeout: Duration,
}

impl AppConfig {
    pub fn load(cli: &Cli) -> Result<Self> {
        let file = load_config_file(&cli.config)?;

        let rpc_url = cli
            .rpc_url
            .clone()
            .or(file.rpc_url)
            .unwrap_or_else(|| DEFAULT_RPC_URL.to_owned());
        let rpc_user = required_value(cli.rpc_user.clone().or(file.rpc_user), "RPC username")?;
        let rpc_password = required_value(
            cli.rpc_password.clone().or(file.rpc_password),
            "RPC password",
        )?;
        let wallet = cli
            .wallet
            .clone()
            .or(file.wallet)
            .unwrap_or_else(|| DEFAULT_WALLET.to_owned());
        let timeout_seconds = cli
            .timeout_seconds
            .or(file.timeout_seconds)
            .unwrap_or(DEFAULT_TIMEOUT_SECONDS);

        if timeout_seconds == 0 {
            bail!("RPC timeout must be greater than zero seconds");
        }

        let rpc_url = Url::parse(&rpc_url)
            .with_context(|| format!("RPC URL `{rpc_url}` is not a valid URL"))?;

        if !matches!(rpc_url.scheme(), "http" | "https") {
            bail!("RPC URL must use http or https");
        }

        Ok(Self {
            rpc_url,
            rpc_user,
            rpc_password,
            wallet,
            timeout: Duration::from_secs(timeout_seconds),
        })
    }

    pub fn rpc_connection(&self) -> RpcConnection {
        RpcConnection {
            url: self.rpc_url.clone(),
            username: self.rpc_user.clone(),
            password: self.rpc_password.clone(),
            timeout: self.timeout,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    rpc_url: Option<String>,
    rpc_user: Option<String>,
    rpc_password: Option<String>,
    wallet: Option<String>,
    timeout_seconds: Option<u64>,
}

fn load_config_file(path: &Path) -> Result<FileConfig> {
    if !path.exists() {
        return Ok(FileConfig::default());
    }

    let contents = fs::read_to_string(path)
        .with_context(|| format!("could not read configuration file `{}`", path.display()))?;

    toml::from_str(&contents)
        .with_context(|| format!("could not parse configuration file `{}`", path.display()))
}

fn required_value(value: Option<String>, label: &str) -> Result<String> {
    match value {
        Some(value) if !value.trim().is_empty() => Ok(value),
        _ => bail!(
            "{label} is required; provide it through a CLI flag, environment variable, or config.toml"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn rejects_zero_timeout() {
        let cli = Cli::try_parse_from([
            "bitcoin-cli-rs",
            "--config",
            "test-data/missing-config.toml",
            "--rpc-user",
            "user",
            "--rpc-password",
            "pass",
            "--timeout-seconds",
            "0",
            "blockchain-info",
        ])
        .expect("valid CLI arguments");

        let error = match AppConfig::load(&cli) {
            Ok(_) => panic!("zero timeout should fail"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("greater than zero"));
    }

    #[test]
    fn rejects_non_http_rpc_url() {
        let cli = Cli::try_parse_from([
            "bitcoin-cli-rs",
            "--config",
            "test-data/missing-config.toml",
            "--rpc-url",
            "file:///tmp/bitcoin.sock",
            "--rpc-user",
            "user",
            "--rpc-password",
            "pass",
            "blockchain-info",
        ])
        .expect("valid CLI arguments");

        let error = match AppConfig::load(&cli) {
            Ok(_) => panic!("file URL should fail"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("http or https"));
    }
}
