//! Logging initialization and configuration.
//!
//! This module provides initialization for structured logging using the `tracing` crate.
//! Logging is configured to use environment-based filtering and output to stderr.

use std::sync::OnceLock;

/// Initializes the tracing subscriber for structured logging.
///
/// This function is idempotent and safe to call multiple times. The first call initializes
/// the logger, subsequent calls are no-ops.
///
/// Logging configuration:
/// * Filter level is controlled by the `RUST_LOG` environment variable (defaults to `info`)
/// * Output goes to stderr
/// * Target information is disabled
/// * Timestamps are disabled
pub(super) fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    static INIT: OnceLock<()> = OnceLock::new();
    INIT.get_or_init(|| {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .with_target(false)
            .without_time()
            .try_init();
    });
}
