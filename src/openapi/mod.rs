//! OpenAPI documentation support for Uncover.
//!
//! This module provides automatic OpenAPI 3.0 documentation generation
//! for your API endpoints.
//!
//! # Features
//!
//! - Automatic schema generation from types
//! - Request/response documentation
//! - Interactive API explorer (Scalar UI)
//! - Type-safe documentation
//!
//! # Quick Start
//!
//! ```no_run
//! use uncover::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Default, Deserialize, JsonSchema)]
//! struct CreateUser {
//!     name: String,
//!     email: String,
//! }
//!
//! #[derive(Serialize, JsonSchema)]
//! struct User {
//!     id: u64,
//!     name: String,
//!     email: String,
//! }
//!
//! #[derive(Clone)]
//! pub struct CreateUserEndpoint;
//!
//! impl Metadata for CreateUserEndpoint {
//!     fn metadata(&self) -> EndpointMetadata {
//!         EndpointMetadata::new("/users", "post")
//!             .summary("Create a new user")
//!     }
//! }
//!
//! #[async_trait]
//! impl API for CreateUserEndpoint {
//!     type Req = CreateUser;
//!     type Res = Json<User>;
//!
//!     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
//!         Json(User {
//!             id: 1,
//!             name: ctx.req.name,
//!             email: ctx.req.email,
//!         })
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = AppConfig::new("My API", "1.0.0")
//!         .description("A simple API built with Uncover")
//!         .docs(true);
//!
//!     uncover::server::Server::new()
//!         .with_config(config)
//!         .register(CreateUserEndpoint)
//!         .serve()
//!         .await
//!         .expect("Server failed");
//! }
//! ```
//!
//! The OpenAPI documentation will be available at `/docs` and the JSON schema at `/openapi.json`.

mod config;
mod docs;

pub use aide::openapi::OpenApi;
pub use config::OpenApiConfig;
pub use docs::{serve_docs, serve_scalar_ui};
pub use schemars::JsonSchema;
