//! # Uncover
//!
//! A modular Rust API framework built on top of Axum, designed for building type-safe,
//! production-ready HTTP APIs with minimal boilerplate.
//!
//! Uncover provides automatic OpenAPI documentation generation, built-in logging,
//! CORS support, and a clean configuration system, while maintaining full compatibility
//! with the Axum ecosystem.
//!
//! ## Quick Start
//!
//! ```no_run
//! use uncover::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! // Define request and response types
//! #[derive(Default, Deserialize, JsonSchema)]
//! pub struct CreateUser {
//!     name: String,
//!     email: String,
//! }
//!
//! #[derive(Serialize, JsonSchema)]
//! pub struct User {
//!     id: u64,
//!     name: String,
//!     email: String,
//! }
//!
//! // Define your endpoint
//! #[derive(Clone)]
//! pub struct CreateUserEndpoint;
//!
//! impl Metadata for CreateUserEndpoint {
//!     fn metadata(&self) -> Endpoint {
//!         Endpoint::new("/users", "post")
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
//!         .logging(LoggingConfig::development());
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
//! ## Features
//!
//! - **Auto-generated OpenAPI Documentation** - Automatic API docs with Scalar UI
//! - **Built-in Logging** - Structured logging powered by tracing
//! - **CORS Support** - Configurable CORS with environment-based presets
//! - **Type-Safe** - Full compile-time type checking for requests and responses
//! - **Configuration Management** - Centralized config via meta.rs pattern
//! - **Minimal Boilerplate** - Focus on business logic, not plumbing
//! - **Axum Compatible** - Use any Axum extractors, middleware, and responses
//! - **Async/Await** - Built for modern async Rust
//!
//! ## Configuration
//!
//! Create a `meta.rs` file to configure your application:
//!
//! ```rust
//! use uncover::prelude::*;
//!
//! pub fn config() -> AppConfig {
//!     AppConfig::new("My API", "1.0.0")
//!         .description("My awesome API")
//!         .environment(Environment::Development)
//!         .logging(LoggingConfig::development())
//!         .cors(CorsConfig::development())
//!         .docs(true)
//! }
//! ```
//!
//! ## Endpoint Definition
//!
//! Endpoints implement two traits: `Metadata` for routing info and `API` for handler logic:
//!
//! ```rust
//! use uncover::prelude::*;
//!
//! #[derive(Clone)]
//! pub struct MyEndpoint;
//!
//! impl Metadata for MyEndpoint {
//!     fn metadata(&self) -> Endpoint {
//!         Endpoint::new("/path", "get")
//!             .summary("Endpoint description")
//!     }
//! }
//!
//! #[async_trait]
//! impl API for MyEndpoint {
//!     type Req = RequestType;
//!     type Res = Json<ResponseType>;
//!
//!     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
//!         // Your logic here
//!         Json(response_data)
//!     }
//! }
//! ```
//!
//! ## Modules
//!
//! - [`prelude`] - Commonly used types and traits (import with `use uncover::prelude::*`)
//! - [`server`] - Server builder and routing functionality
//! - [`api`] - Core API traits and types
//! - [`config`] - Configuration types for logging, CORS, and application settings
//! - [`logging`] - Logging initialization and utilities
//! - [`context`] - Request context types
//! - [`openapi`] - OpenAPI documentation generation
//! - [`middleware`] - Middleware utilities
//! - [`http`] - HTTP types re-exported from Axum
//! - [`extract`] - Extractors re-exported from Axum
//! - [`response`] - Response types re-exported from Axum
//! - [`routing`] - Routing utilities re-exported from Axum
//!
//! ## Logging
//!
//! Uncover includes built-in structured logging:
//!
//! ```rust
//! use uncover::config::{LoggingConfig, LogLevel, LogFormat};
//!
//! // Development: verbose, pretty output
//! let dev_logging = LoggingConfig::development();
//!
//! // Production: info level, JSON format
//! let prod_logging = LoggingConfig::production();
//!
//! // Custom configuration
//! let custom = LoggingConfig::default()
//!     .level(LogLevel::Debug)
//!     .format(LogFormat::Pretty)
//!     .log_requests(true);
//! ```
//!
//! ## CORS
//!
//! Configure CORS based on your environment:
//!
//! ```rust
//! use uncover::config::CorsConfig;
//!
//! // Development: allow all origins
//! let dev_cors = CorsConfig::development();
//!
//! // Production: specific origins only
//! let prod_cors = CorsConfig::production(vec![
//!     "https://yourdomain.com".to_string()
//! ]);
//! ```

pub mod api;
pub mod config;
pub mod context;
pub mod logging;
pub mod middleware;
pub mod openapi;
pub mod prelude;
pub mod server;

// Re-export commonly used Axum modules at the root level
pub mod http {
    //! HTTP types re-exported from Axum.
    //!
    //! Includes status codes, headers, methods, and other HTTP primitives.
    pub use axum::http::*;
}

pub mod extract {
    //! Request extractors re-exported from Axum.
    //!
    //! Extractors allow you to declaratively parse different parts of a request.
    pub use axum::extract::*;
}

pub mod response {
    //! Response types re-exported from Axum.
    //!
    //! Types for building HTTP responses.
    pub use axum::response::*;
}

pub mod routing {
    //! Routing utilities re-exported from Axum.
    //!
    //! Types and functions for advanced routing scenarios.
    pub use axum::routing::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn context_creation() {
        let ctx = Context {
            req: "request".to_string(),
            headers: HeaderMap::new(),
        };
        assert_eq!(ctx.req, "request");
    }
}
