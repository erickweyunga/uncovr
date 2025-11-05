//! Server module for Uncover framework.
//!
//! Provides a modular API for building HTTP servers with type-safe endpoints.
//!
//! # Example
//!
//! ```no_run
//! use uncovr::prelude::*;
//! use uncovr::server::Server;
//!
//! #[derive(Clone)]
//! pub struct Hello;
//!
//! impl Metadata for Hello {
//!     fn metadata(&self) -> Endpoint {
//!         Endpoint::new("/hello", "get")
//!     }
//! }
//!
//! #[async_trait]
//! impl API for Hello {
//!     type Req = ();
//!     type Res = &'static str;
//!
//!     async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
//!         "Hello, World!"
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = AppConfig::new("My API", "1.0.0");
//!
//!     Server::new()
//!         .with_config(config)
//!         .register(Hello)
//!         .serve()
//!         .await
//!         .unwrap();
//! }
//! ```

mod builder;
mod router;

pub use builder::{Endpoint, Metadata, Server, ServerBuilder};
pub use router::RouterExt;
