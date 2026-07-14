//! Application logging setup.

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Initializes structured logging to stderr.
///
/// `RUST_LOG` controls verbosity. The default is `warn`, which keeps normal CLI
/// output quiet. Repeated initialization is ignored so tests can install their
/// own subscriber.
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let formatting = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_writer(std::io::stderr);

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(formatting)
        .try_init();
}
