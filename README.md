# Bitcoin-cli-rs

A modular Rust command-line client for interacting with Bitcoin Core through its JSON-RPC API.

> **Status:** Early development. The Cargo workspace is configured, but the Bitcoin Core commands described below are not implemented yet.

## Overview

`bitcoin-cli-rs` will connect to a local Bitcoin Core node running on Regtest through [Polar](https://lightningpolar.com/). It will provide friendly commands for common blockchain and wallet operations while retaining access to arbitrary Bitcoin Core RPC methods.

The project is being built for the Rust for Bitcoin Program 2.0 technical assessment, but its structure and naming are intended to support continued development after the assessment.

## Planned commands

| Command | Purpose |
| --- | --- |
| `blockchain-info` | Display the chain, block and header counts, difficulty, and verification progress. |
| `wallet-info` | Display the wallet name, trusted and unconfirmed balances, and transaction count. |
| `balance` | Print the wallet's trusted balance. |
| `new-address` | Generate and print a new receiving address. |
| `rpc <method> [params...]` | Execute an arbitrary Bitcoin Core JSON-RPC method. |

Planned examples:

```bash
cargo run -- blockchain-info
cargo run -- wallet-info
cargo run -- balance
cargo run -- new-address
cargo run -- rpc getblockcount
cargo run -- rpc getblockhash 20
```

## Architecture

This repository is a Cargo workspace with a clear boundary between the executable application and the reusable Bitcoin Core integration:

```text
bitcoin-cli-rs/
├── bin/
│   └── bitcoin-rpc-cli/      # CLI parsing, configuration, commands, and output
├── crates/
│   └── bitcoin-core-rpc/     # JSON-RPC transport, models, and typed errors
├── Cargo.toml                # Workspace configuration and shared dependencies
├── config.example.toml       # TOML configuration template
└── .env.example              # Environment-variable template
```

The intended request flow is:

```text
Terminal command
    -> bitcoin-rpc-cli
    -> bitcoin-core-rpc
    -> Bitcoin Core JSON-RPC
    -> Polar Regtest node
```

## Prerequisites

- Rust 1.85 or newer.
- Cargo.
- [Docker](https://www.docker.com/products/docker-desktop/) running locally.
- [Polar](https://lightningpolar.com/) with a started Bitcoin Core Regtest node.

Polar manages the local Bitcoin Core container, so a separate Bitcoin Core desktop installation is not required for the intended setup.

## Configuration

The application is intended to support a local TOML file with environment-variable overrides.

Copy the provided templates:

```bash
cp config.example.toml config.toml
cp .env.example .env
```

Example non-sensitive TOML configuration:

```toml
rpc_url = "http://127.0.0.1:18443"
wallet = "assessment-wallet"
timeout_seconds = 30
```

Example environment variables:

```env
BITCOIN_RPC_USER=your_rpc_username
BITCOIN_RPC_PASSWORD=your_rpc_password
```

The intended precedence is:

```text
CLI flags -> environment variables -> config.toml -> built-in defaults
```

Obtain the RPC URL, username, and password from the connection details for the Bitcoin Core node inside Polar. Never commit real RPC credentials. Both `.env` and `config.toml` are ignored by Git.

## Build the workspace

From the repository root:

```bash
cargo build
```

Check every workspace member:

```bash
cargo check --workspace
```

The CLI is the workspace's default member, so root-level `cargo run -- ...` commands will target `bitcoin-rpc-cli` without requiring `-p bitcoin-rpc-cli`.

## Development checks

Before submitting changes, run:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build
```

## Error-handling approach

- The reusable `bitcoin-core-rpc` library will expose structured errors using `thiserror`.
- The `bitcoin-rpc-cli` application will add user-facing context using `anyhow`.
- Connection failures, authentication failures, invalid methods or parameters, and missing wallets must produce clear messages without panicking.

## Security

- Do not commit `.env` or `config.toml`.
- Do not include Polar RPC credentials in screenshots or example output.
- Commit only `.env.example` and `config.example.toml`, using placeholder values.

## Implementation roadmap

- [x] Create the Cargo workspace.
- [x] Separate the CLI binary and RPC client library.
- [x] Add safe configuration templates.
- [ ] Implement configuration loading and precedence.
- [ ] Implement the reusable JSON-RPC client.
- [ ] Implement the required named commands.
- [ ] Implement dynamic generic RPC parameters.
- [ ] Add focused unit and integration tests.
- [ ] Verify all commands against a Polar Regtest node.
- [ ] Add real, safely redacted terminal output.

## Assumptions

- Bitcoin Core is accessed locally through Polar and runs in Regtest mode.
- Wallet commands require a wallet that exists and is loaded.
- The `balance` command reports the wallet's trusted balance.
- Generic RPC arguments will be parsed as JSON values when possible and otherwise treated as strings.
