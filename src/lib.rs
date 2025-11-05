//! # Uncover
//!
//! A modular microbackend framework for building type-safe, production-ready REST APIs
//! with minimal boilerplate and automatic documentation.
//!
//! Uncover enables you to build composable, self-contained API modules (microbackends)
//! with automatic OpenAPI documentation, built-in logging, CORS support, and a clean
//! configuration system. Each endpoint is a standalone module that can be developed,
//! tested, and deployed independently.
//!
//! ## Quick Start
//!
//! ```no_run
//! use uncovr::prelude::*;
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
//!     uncovr::server::Server::new()
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
//! - **Microbackend Architecture** - Build composable, self-contained API modules
//! - **Auto-generated OpenAPI Documentation** - Interactive API docs with Scalar UI
//! - **Type-Safe Endpoints** - Full compile-time type checking for requests and responses
//! - **Modular Design** - Each endpoint is an independent, testable module
//! - **Built-in Logging** - Structured logging with development and production modes
//! - **CORS Support** - Environment-based CORS configuration out of the box
//! - **Configuration Management** - Centralized, type-safe configuration
//! - **Minimal Boilerplate** - Focus on business logic, not framework code
//! - **Production-Ready** - Built-in middleware, error handling, and best practices
//!
//! ## Configuration
//!
//! Create a `meta.rs` file to configure your application:
//!
//! ```rust
//! use uncovr::prelude::*;
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
//! use uncovr::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone)]
//! pub struct MyEndpoint;
//!
//! #[derive(Default, Deserialize, JsonSchema)]
//! pub struct MyRequest {
//!     name: String,
//! }
//!
//! #[derive(Serialize, JsonSchema)]
//! pub struct MyResponse {
//!     message: String,
//! }
//!
//! impl Metadata for MyEndpoint {
//!     fn metadata(&self) -> Endpoint {
//!         Endpoint::new("/path", "post")
//!             .summary("Endpoint description")
//!     }
//! }
//!
//! #[async_trait]
//! impl API for MyEndpoint {
//!     type Req = MyRequest;
//!     type Res = Json<MyResponse>;
//!
//!     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
//!         Json(MyResponse {
//!             message: format!("Hello, {}!", ctx.req.name)
//!         })
//!     }
//! }
//! ```
//!
//! ## Modules
//!
//! - [`prelude`] - Commonly used types and traits (import with `use uncovr::prelude::*`)
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
//! use uncovr::config::{LoggingConfig, LogLevel, LogFormat};
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
//! use uncovr::config::CorsConfig;
//!
//! // Development: allow all origins
//! let dev_cors = CorsConfig::development();
//!
//! // Production: specific origins only
//! let prod_cors = CorsConfig::production(vec![
//!     "https://yourdomain.com".to_string()
//! ]);
//! ```

/// Core API traits and types for defining endpoints.
///
/// This module contains the fundamental [`API`](api::api::API) trait that all endpoints must implement.
pub mod api;

/// Configuration types for application settings, logging, CORS, and environments.
///
/// Provides [`AppConfig`](config::AppConfig), [`LoggingConfig`](config::LoggingConfig),
/// and [`CorsConfig`](config::CorsConfig) for configuring the server.
pub mod config;

/// Request context types passed to endpoint handlers.
///
/// Contains the [`Context`](context::Context) struct that wraps request data and headers.
pub mod context;

/// Logging initialization and utilities.
///
/// Provides structured logging setup with support for development and production formats.
pub mod logging;

/// Middleware utilities and helpers.
///
/// Additional middleware components for the framework.
pub mod middleware;

/// OpenAPI documentation generation and serving.
///
/// Automatic API documentation generation with Scalar UI integration.
pub mod openapi;

/// Commonly used types and traits.
///
/// Import everything you need with `use uncovr::prelude::*;`
pub mod prelude;

/// Server builder and routing functionality.
///
/// Contains the [`Server`](server::Server) and [`ServerBuilder`](server::ServerBuilder)
/// for configuring and running the HTTP server.
pub mod server;

// Re-export schemars so derive macros work in user code
pub use schemars;

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
    use std::sync::Arc;

    use super::prelude::*;

    #[test]
    fn context_creation() {
        let ctx = Context {
            req: "request".to_string(),
            headers: Arc::new(HeaderMap::new()),
        };
        assert_eq!(ctx.req, "request");
    }
}
