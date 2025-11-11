//! # Uncovr
//!
//! **Uncovr: Modular, type-safe, developer-friendly backend framework for Rust**
//!
//! Uncovr is a comprehensive backend framework that enables you to build production-ready
//! applications with automatic OpenAPI documentation, built-in middleware, and a clean
//! modular architecture. Focus on your business logic while Uncovr handles the infrastructure.
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
//! impl Endpoint for CreateUserEndpoint {
//!     fn ep(&self) -> Route {
//!         Route::POST("/users")
//!     }
//!
//!     fn docs(&self) -> Option<Docs> {
//!         Some(Docs::new()
//!             .summary("Create a new user")
//!             .tag("users"))
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
//!     let config = App::new("My API", "1.0.0")
//!         .logging(Logging::development());
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
//! - **Type-Safe** - Full compile-time type checking for requests, responses, and parameters
//! - **Automatic Documentation** - OpenAPI 3.0 specification with interactive Scalar UI
//! - **Modular Architecture** - Composable endpoints and middleware for clean separation of concerns
//! - **Developer-Friendly** - Intuitive APIs with excellent error messages and examples
//! - **Structured Logging** - Development and production logging modes with tracing support
//! - **Production-Ready** - Error handling, validation, and best practices out of the box
//! - **Extensible** - Easy integration with Axum ecosystem and Tower middleware
//!
//! ## Configuration
//!
//! Create a `meta.rs` file to configure your application:
//!
//! ```rust
//! use uncovr::prelude::*;
//!
//! pub fn config() -> App {
//!     App::new("My API", "1.0.0")
//!         .description("My awesome API")
//!         .environment(Environment::Development)
//!         .logging(Logging::development())
//!         .cors(CorsConfig::development())
//!         .docs(true)
//! }
//! ```
//!
//! ## Endpoint Definition
//!
//! Endpoints implement two traits:
//! - `Endpoint` for route definition and optional documentation
//! - `API` for handler logic
//!
//! ```rust,no_run
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
//! impl Endpoint for MyEndpoint {
//!     fn ep(&self) -> Route {
//!         Route::POST("/greet")
//!             .query("lang").desc("Language code")
//!     }
//!
//!     fn docs(&self) -> Option<Docs> {
//!         Some(Docs::new()
//!             .summary("Greet a user")
//!             .description("Returns a personalized greeting")
//!             .tag("greetings"))
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
//! ## HTTP Methods
//!
//! Use uppercase constructors for type-safe HTTP methods:
//!
//! ```rust
//! use uncovr::server::endpoint::Route;
//!
//! let get_route = Route::GET("/users");
//! let post_route = Route::POST("/users");
//! let put_route = Route::PUT("/users/:id");
//! let delete_route = Route::DELETE("/users/:id");
//! let patch_route = Route::PATCH("/users/:id");
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
//! - [`http`] - HTTP types re-exported from Axum
//! - [`extract`] - Extractors re-exported from Axum
//! - [`response`] - Response types re-exported from Axum
//! - [`routing`] - Routing utilities re-exported from Axum
//!
//! ## Fallback Routes
//!
//! Handle unmatched routes with custom fallback handlers:
//!
//! ```no_run
//! use uncovr::prelude::*;
//!
//! async fn handle_404() -> (StatusCode, Json<ErrorResponse>) {
//!     (
//!         StatusCode::NOT_FOUND,
//!         Json(ErrorResponse {
//!             error: "Not Found".to_string(),
//!             message: Some("The requested resource does not exist".to_string()),
//!         })
//!     )
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = App::new("My API", "1.0.0");
//!
//!     uncovr::server::Server::new()
//!         .with_config(config)
//!         .register(YourEndpoint)
//!         .fallback(handle_404)
//!         .serve()
//!         .await
//!         .expect("Server failed");
//! }
//! ```
//!
//! For more complex scenarios, use `fallback_service` to integrate Tower services:
//!
//! ```no_run
//! use uncovr::prelude::*;
//! use tower::service_fn;
//! use axum::body::Body;
//! use axum::http::{Request, Response};
//!
//! async fn custom_fallback(req: Request<Body>) -> Result<Response<Body>, std::convert::Infallible> {
//!     Ok(Response::builder()
//!         .status(404)
//!         .body(Body::from("Custom 404 response"))
//!         .unwrap())
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = App::new("My API", "1.0.0");
//!
//!     uncovr::server::Server::new()
//!         .with_config(config)
//!         .fallback_service(service_fn(custom_fallback))
//!         .serve()
//!         .await
//!         .expect("Server failed");
//! }
//! ```
//!
//! ## Logging
//!
//! Uncovr includes built-in structured logging:
//!
//! ```rust
//! use uncovr::config::{Logging, LogLevel, LogFormat};
//!
//! // Development: verbose, pretty output
//! let dev_logging = Logging::development();
//!
//! // Production: info level, JSON format
//! let prod_logging = Logging::production();
//!
//! // Custom configuration
//! let custom = Logging::default()
//!     .level(LogLevel::Debug)
//!     .format(LogFormat::Pretty)
//!     .log_requests(true);
//! ```
//!
//! ## OpenAPI Documentation
//!
//! ```no_run
//! use uncovr::openapi::OpenApiConfig;
//!
//! let openapi = OpenApiConfig::new("My API", "1.0.0")
//!     .description("A comprehensive API")
//!     .server("https://api.example.com", "Production")
//!     .server("http://localhost:3000", "Development");
//! ```
//!
//! The interactive documentation is automatically available at `/docs` when the server is running.

pub mod api;
pub mod config;
pub mod context;
pub mod logging;
pub mod openapi;
pub mod server;

/// Testing utilities for integration tests
#[cfg(feature = "testing")]
pub mod testing;

/// Built-in middleware collection
pub mod middleware;

/// HTTP types re-exported from Axum
pub mod http {
    pub use axum::http::*;
}

/// Re-exports commonly used types and traits
///
/// Import everything you need with `use uncovr::prelude::*;`
pub mod prelude;

/// Axum extractors
pub mod extract {
    pub use axum::extract::*;
}

/// Response types
pub mod response {
    pub use axum::response::*;
}

/// Routing utilities
pub mod routing {
    pub use axum::routing::*;
}

/// Axum middleware utilities
pub mod axum_middleware {
    pub use axum::middleware::*;
}

/// Tower middleware and service utilities
pub mod tower {
    pub use tower::*;
}

#[cfg(test)]
mod tests {
    use crate::context::Context;

    #[test]
    fn test_context_creation() {
        let ctx = Context::<()> {
            req: (),
            headers: Default::default(),
            path: crate::server::Path::new(Default::default()),
            query: crate::server::Query::new(Default::default()),
            extensions: Default::default(),
        };

        assert_eq!(ctx.path.get_string("test"), None);
    }
}
