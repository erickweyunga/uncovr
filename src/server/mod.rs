//! Server module for Uncover framework.
//!
//! Provides a declarative API for building HTTP servers with Axum,
//! integrating seamlessly with `#[resolve]` endpoints.
//!
//! # Example
//!
//! ```no_run
//! use uncover::prelude::*;
//! use uncover::server::Server;
//!
//! #[resolve(path = "/hello")]
//! pub struct Hello;
//!
//! impl Hello {
//!     async fn handler(&self) -> &'static str {
//!         "Hello, World!"
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     Server::new()
//!         .register(Hello, |endpoint| endpoint.handler())
//!         .bind("0.0.0.0:3000")
//!         .serve()
//!         .await
//!         .unwrap();
//! }
//! ```

mod builder;
mod router;

pub use builder::{Endpoint, Metadata, Server, ServerBuilder};
pub use router::RouterExt;
