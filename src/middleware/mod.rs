//! Built-in middleware collection for common use cases
//!
//! This module provides pre-built middleware for common scenarios like
//! request IDs, rate limiting, and authentication helpers.
//!
//! # Example
//!
//! ```rust,no_run
//! use uncovr::prelude::*;
//! use uncovr::middleware::{RequestId, RateLimit};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     Server::new()
//!         .with_config(AppConfig::new("My API", "1.0.0"))
//!         .layer(RequestId::new())
//!         .layer(RateLimit::new(100, Duration::from_secs(60)))
//!         .register(MyEndpoint)
//!         .serve()
//!         .await
//!         .unwrap();
//! }
//! ```

mod auth;
mod cors;
mod rate_limit;
mod request_id;

pub use auth::BearerAuth;
pub use cors::Cors;
pub use rate_limit::RateLimit;
pub use request_id::RequestId;
