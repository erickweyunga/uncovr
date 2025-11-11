//! Server module for Uncovr framework.
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
//! impl Endpoint for Hello {
//!     fn route(&self) -> Route {
//!         Route::get("/hello")
//!     }
//!
//!     fn meta(&self) -> Meta {
//!         Meta::new().summary("Say hello")
//!     }
//! }
//!
//! #[async_trait]
//! impl Handler for Hello {
//!     type Request = ();
//!     type Response = &'static str;
//!
//!     async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
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
pub mod endpoint;
pub mod params;
mod router;

pub use builder::{Server, ServerBuilder};
pub use endpoint::{Endpoint, HttpMethod, Meta, PathParam, QueryParam, ResponseCallback, Route};
pub use params::{Path, Query};
pub use router::RouterExt;
