//! Application configuration module
//!
//! Provides configuration structures for Uncovr applications.
//!
//! # Example
//!
//! ```rust,ignore
//! use uncovr::config::{App, Logging};
//! use uncovr::middleware::Cors;
//!
//! let app = App::new("My API", "1.0.0", "0.0.0.0:8080")
//!     .description("A REST API");
//!
//! Server::new()
//!     .with_config(app)
//!     .with_logging(Logging::development())
//!     .layer(Cors::permissive())
//!     .serve()
//!     .await
//! ```

mod app;
mod logging;

pub use app::{App, Server};
pub use logging::{LogFormat, LogLevel, Logging};
