//! Middleware utilities for Uncover.
//!
//! This module provides middleware types and utilities. Currently, logging
//! is handled automatically by the server builder based on configuration,
//! and CORS is also managed through the configuration system.
//!
//! For custom middleware, you can use Axum's middleware system directly
//! by wrapping the router with tower layers.
//!
//! # Example with Custom Middleware
//!
//! ```rust,no_run
//! use uncover::prelude::*;
//! use tower_http::timeout::TimeoutLayer;
//! use std::time::Duration;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let config = AppConfig::new("My API", "1.0.0");
//!
//! let server = uncover::server::Server::new()
//!     .with_config(config)
//!     .build();
//!
//! // The server can be extended with Axum middleware
//! // (Note: This is pseudocode for illustration)
//! // let app = server.layer(TimeoutLayer::new(Duration::from_secs(30)));
//! # }
//! ```

use crate::context::Context;

/// Type alias for middleware functions.
///
/// Currently for internal use. For custom middleware, use Axum's
/// tower middleware system instead.
pub type Middleware<Req, Res> = Box<dyn Fn(&Context<Req>, &mut Res) + Send + Sync>;
