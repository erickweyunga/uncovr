//! Logging initialization and utilities for Uncover framework.
//!
//! This module provides logging setup based on configuration, with support for
//! both pretty-printed development logs and structured JSON logs for production.
//!
//! # Examples
//!
//! ```rust
//! use uncover::logging;
//! use uncover::config::LoggingConfig;
//!
//! // Initialize logging (usually done automatically by the server)
//! let config = LoggingConfig::development();
//! logging::init(&config);
//!
//! // Now you can use tracing macros
//! tracing::info!("Server started");
//! tracing::debug!(user_id = 42, "Processing request");
//! ```

use crate::config::{LogFormat, LoggingConfig};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize the logging system based on configuration.
///
/// This function sets up the tracing subscriber with the specified log level
/// and format. It's automatically called by the server builder when using
/// `with_config()`, so you typically don't need to call it manually.
///
/// # Configuration
///
/// - **Pretty format**: Compact, single-line logs without file/line numbers (development)
/// - **JSON format**: Structured logs with file/line numbers (production)
///
/// The log level can be overridden using the `RUST_LOG` environment variable:
/// ```bash
/// RUST_LOG=debug cargo run
/// RUST_LOG=my_api=trace,tower_http=info cargo run
/// ```
///
/// # Examples
///
/// ```rust
/// use uncover::{logging, config::{LoggingConfig, LogLevel, LogFormat}};
///
/// // Development logging
/// let dev_config = LoggingConfig::development();
/// logging::init(&dev_config);
///
/// // Production logging
/// let prod_config = LoggingConfig::production();
/// logging::init(&prod_config);
///
/// // Custom configuration
/// let custom = LoggingConfig::default()
///     .level(LogLevel::Info)
///     .format(LogFormat::Json);
/// logging::init(&custom);
/// ```
pub fn init(config: &LoggingConfig) {
    if !config.enabled {
        return;
    }

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.as_filter()));

    match config.format {
        LogFormat::Pretty => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_target(false)
                        .with_thread_ids(false)
                        .with_file(false)
                        .with_line_number(false)
                        .compact(),
                )
                .init();
        }
        LogFormat::Json => {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().with_file(true).with_line_number(true).json())
                .init();
        }
    }
}

#[macro_export]
macro_rules! request_span {
    ($method:expr, $path:expr) => {
        tracing::info_span!(
            "request",
            method = %$method,
            path = %$path,
        )
    };
}
