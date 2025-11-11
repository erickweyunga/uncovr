//! Logging initialization and utilities for Uncovr framework.
//!
//! This module provides logging setup based on configuration, with support for
//! both pretty-printed development logs and structured JSON logs for production.
//!
//! # Examples
//!
//! ```rust
//! use uncovr::logging;
//! use uncovr::config::Logging;
//!
//! // Initialize logging (usually done automatically by the server)
//! let config = Logging::development();
//! logging::init(&config);
//!
//! // Now you can use tracing macros
//! tracing::info!("Server started");
//! tracing::debug!(user_id = 42, "Processing request");
//! ```

use crate::config::{LogFormat, Logging};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
/// use uncovr::{logging, config::{Logging, LogLevel, LogFormat}};
///
/// // Development logging
/// let dev_config = Logging::development();
/// logging::init(&dev_config);
///
/// // Production logging
/// let prod_config = Logging::production();
/// logging::init(&prod_config);
///
/// // Custom configuration
/// let custom = Logging::default()
///     .level(LogLevel::Info)
///     .format(LogFormat::Json);
/// logging::init(&custom);
/// ```
pub fn init(config: &Logging) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level.as_filter()));

    match config.format {
        LogFormat::Pretty => {
            let _ = tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_target(false)
                        .with_thread_ids(false)
                        .with_file(false)
                        .with_line_number(false)
                        .compact(),
                )
                .try_init();
        }
        LogFormat::Json => {
            let _ = tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().with_file(true).with_line_number(true).json())
                .try_init();
        }
    }
}

/// Creates a tracing span for HTTP requests.
///
/// This macro simplifies creating tracing spans for HTTP requests with
/// method and path information.
///
/// # Examples
///
/// ```rust
/// use uncovr::request_span;
///
/// let span = request_span!("GET", "/api/users");
/// let _enter = span.enter();
/// // Your request handling code here
/// ```
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
