# Bitcoin-cli-rs

A modular Rust command-line client for interacting with Bitcoin Core through its JSON-RPC API.

> **Status:** Milestones 1 and 2 are complete. Configuration loading, the reusable JSON-RPC transport, Clap command routing, `blockchain-info`, and the wallet commands are implemented. Generic RPC execution is currently an explicit placeholder.

## Overview

`bitcoin-cli-rs` will connect to a local Bitcoin Core node running on Regtest through [Polar](https://lightningpolar.com/). It will provide friendly commands for common blockchain and wallet operations while retaining access to arbitrary Bitcoin Core RPC methods.

The project is being built for the Rust for Bitcoin Program 2.0 technical assessment, but its structure and naming are intended to support continued development after the assessment.

## Commands

| Command | Purpose | Status |
| --- | --- | --- |
| `blockchain-info` | Display the chain, block and header counts, difficulty, and verification progress. | Implemented |
| `wallet-info` | Display the wallet name, trusted, unconfirmed, and immature balances, and transaction count. | Implemented |
| `balance` | Print the wallet's trusted balance. | Implemented |
| `new-address` | Generate and print a new Bech32 receiving address. | Implemented |
| `rpc <method> [params...]` | Execute an arbitrary Bitcoin Core JSON-RPC method. | Placeholder |

Current working examples:

```bash
cargo run -- blockchain-info
cargo run -- wallet-info
cargo run -- balance
cargo run -- new-address
```

Planned examples for the next milestone:

```bash
cargo run -- rpc getblockcount
cargo run -- rpc getblockhash 20
```

## Architecture

This repository is a single Cargo package organized into focused Rust modules:

```text
bitcoin-cli-rs/
├── src/
│   ├── main.rs               # Executable entry point and logger setup
│   ├── cli.rs                # Clap arguments and subcommands
│   ├── rpc.rs                # JSON-RPC transport, methods, and models
│   ├── config.rs             # TOML, environment, and CLI configuration
│   ├── error.rs              # Typed RPC errors
│   ├── logger.rs             # Structured tracing setup
│   └── commands/
│       ├── mod.rs            # Command dispatch and shared command helpers
│       ├── blockchain.rs     # Blockchain command output
│       ├── wallet.rs         # Wallet information and balance output
│       └── address.rs        # New-address output
├── Cargo.toml                # Package metadata and dependencies
├── config.example.toml       # TOML configuration template
└── .env.example              # Environment-variable template
```

The intended request flow is:

```text
Terminal command
    -> bitcoin-cli-rs
    -> rpc module
    -> Bitcoin Core JSON-RPC
    -> Polar Regtest node
```

## Prerequisites

- Rust 1.85 or newer.
- Cargo.
- [Docker](https://www.docker.com/products/docker-desktop/) running locally.
- [Polar](https://lightningpolar.com/) with a started Bitcoin Core Regtest node.

Polar manages the local Bitcoin Core container, so a separate Bitcoin Core desktop installation is not required for the intended setup.

### Create the Regtest wallet

Bitcoin Core provides wallet functionality, but a newly created Polar node does not automatically create a wallet. After starting the network, open the Bitcoin Core node terminal in Polar and create the wallet used by this project:

```bash
bitcoin-cli createwallet "bitcoin-cli-rs-wallet"
```

Confirm that the wallet exists and is loaded:

```bash
bitcoin-cli -rpcwallet=bitcoin-cli-rs-wallet getwalletinfo
```

Blockchain-only commands such as `blockchain-info` do not require a wallet. The `wallet-info`, `balance`, and `new-address` commands require the wallet configured below to exist and be loaded.

### macOS Gatekeeper troubleshooting

Polar's macOS build may be blocked because it is not notarized by Apple. First try the standard macOS flow under **System Settings -> Privacy & Security -> Open Anyway**.

On an Apple Silicon Mac, download the ARM64 DMG from the official [Polar releases](https://github.com/jamaljsr/polar/releases) page. Before bypassing quarantine, verify that the installer checksum matches the digest published with that release. For Polar v4.0.0:

```bash
shasum -a 256 ~/Downloads/polar-mac-arm64-v4.0.0.dmg
```

Expected SHA-256 for the official v4.0.0 ARM64 asset:

```text
bfc315f71f710666f7efdf0c8f9be92d32ab63e957db07f3bbfd1462c77b5295
```

If the checksum matches but macOS does not provide an **Open Anyway** option, clearing Polar's extended attributes allowed the application to open in the tested environment:

```bash
xattr -cr /Applications/Polar.app
open /Applications/Polar.app
```

`xattr -cr` recursively removes extended attributes, including the quarantine attribute used by Gatekeeper. Only run it for an application downloaded from a source you trust and whose checksum you have verified; do not use it as a general Gatekeeper bypass.

## Configuration

The application supports a local TOML file with environment-variable overrides.

Copy the provided templates:

```bash
cp config.example.toml config.toml
cp .env.example .env
```

Example non-sensitive TOML configuration:

```toml
rpc_url = "http://127.0.0.1:18443"
wallet = "bitcoin-cli-rs-wallet"
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

The application validates that credentials are present, the timeout is greater than zero, and the RPC URL uses HTTP or HTTPS before attempting a request. Passwords are not included in command output or debug representations.

## Logging

The executable initializes `tracing-subscriber` once from `main.rs`. Application and RPC modules emit structured events using `tracing`. Logs are written to stderr so stdout remains suitable for command results and shell scripts.

Logging defaults to `warn`. Set `RUST_LOG` to increase verbosity:

```bash
RUST_LOG=bitcoin_cli_rs=info cargo run -- wallet-info
RUST_LOG=bitcoin_cli_rs=debug cargo run -- blockchain-info
```

RPC credentials and RPC parameter values are never logged.

`main.rs` is the only executable entry point. Every other file under `src/` is a Rust module compiled into the same executable.

## Build the application

From the repository root:

```bash
cargo build
```

Check the package:

```bash
cargo check
```

Root-level `cargo run -- ...` commands target the `bitcoin-cli-rs` executable.

## Development checks

Before submitting changes, run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
```

## Example output

With the Polar Regtest network running:

```text
$ cargo run -- blockchain-info
Chain:                 regtest
Blocks:                1
Headers:               1
Best block hash:       337e012a20f4e1d013bd5fc25d1e63750a92fc71c4753a91da3800f54580e18a
Difficulty:            0.00000000046565423739069247
Verification progress: 100.00%
Initial block download: false
Pruned:                false
```

Block height and hashes will differ as blocks are generated in each user's Regtest network.

Wallet command examples:

```text
$ cargo run -- wallet-info
Wallet:              bitcoin-cli-rs-wallet
Trusted balance:     0.0 BTC
Unconfirmed balance: 0.0 BTC
Immature balance:    0.0 BTC
Transactions:        0

$ cargo run -- balance
0.0 BTC

$ cargo run -- new-address
bcrt1qja0mtkccr8ynwdrkwxk7gpzgljrgmxwyy6a73w
```

Wallet balances and newly generated Regtest addresses will differ between environments. Regtest addresses and coins have no mainnet value.

## Error-handling approach

- The RPC module exposes structured errors using `thiserror`.
- The CLI command layer adds user-facing context using `anyhow`.
- Connection failures, authentication failures, invalid methods or parameters, and missing wallets must produce clear messages without panicking.
- If the configured wallet does not exist or is not loaded, wallet commands will identify the wallet by name and explain how to create or load it. They will exit with a non-zero status instead of panicking.

## Security

- Do not commit `.env` or `config.toml`.
- Do not include Polar RPC credentials in screenshots or example output.
- Commit only `.env.example` and `config.example.toml`, using placeholder values.

## Implementation roadmap

- [x] Create the Cargo package and module structure.
- [x] Separate CLI, RPC, configuration, errors, and command concerns.
- [x] Add safe configuration templates.
- [x] Implement configuration loading and precedence.
- [x] Implement the reusable JSON-RPC client.
- [x] Implement and verify `blockchain-info` against Polar.
- [x] Implement and verify the wallet-related named commands.
- [ ] Implement dynamic generic RPC parameters.
- [x] Add focused unit tests.
- [ ] Add automated integration tests.
- [ ] Verify all commands against a Polar Regtest node.
- [x] Add real, safely redacted terminal output for `blockchain-info`.

## Assumptions

- Bitcoin Core is accessed locally through Polar and runs in Regtest mode.
- Wallet commands require a wallet that exists and is loaded.
- The `balance` command reports the wallet's trusted balance.
- Generic RPC arguments will be parsed as JSON values when possible and otherwise treated as strings.
