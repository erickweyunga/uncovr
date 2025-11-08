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
//!     fn ep(&self) -> Route {
//!         Route::GET("/hello")
//!     }
//!
//!     fn docs(&self) -> Option<Docs> {
//!         Some(Docs::new().summary("Say hello"))
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
pub mod endpoint;
pub mod params;
mod router;

pub use builder::{Server, ServerBuilder};
pub use endpoint::{Docs, Endpoint, HttpMethod, PathParam, QueryParam, ResponseCallback, Route};
pub use params::{PathParams, QueryParams};
pub use router::RouterExt;
