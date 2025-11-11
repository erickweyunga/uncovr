//! Application configuration module
//!
//! Provides simplified configuration structures for Uncovr applications.
//!
//! # Migration from v0.2.x
//!
//! In v0.3.0, configuration has been simplified:
//! - CORS moved to `uncovr::middleware::Cors`
//! - Environment enum removed (configure explicitly)
//! - Logging is now optional (use `.with_logging()` on ServerBuilder)
//!
//! ## Before (v0.2.x)
//!
//! ```rust,ignore
//! use uncovr::config::{AppConfig, LoggingConfig, CorsConfig, Environment};
//!
//! let config = AppConfig::new("My API", "1.0.0")
//!     .environment(Environment::Development)
//!     .logging(LoggingConfig::development())
//!     .cors(CorsConfig::development());
//!
//! Server::new()
//!     .with_config(config)
//!     .serve()
//!     .await
//! ```
//!
//! ## After (v0.3.0)
//!
//! ```rust,ignore
//! use uncovr::config::{AppConfig, LoggingConfig};
//! use uncovr::middleware::Cors;
//!
//! let config = AppConfig::new("My API", "1.0.0");
//!
//! Server::new()
//!     .with_config(config)
//!     .with_logging(LoggingConfig::development())
//!     .layer(Cors::permissive())
//!     .serve()
//!     .await
//! ```

mod app;
mod logging;

pub use app::{ApiServer, AppConfig};
pub use logging::{LogFormat, LogLevel, LoggingConfig};
