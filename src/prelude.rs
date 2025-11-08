//! Re-exports commonly used types and traits
//!
//! This module provides a convenient way to import all the common types and traits
//! needed to build APIs with uncover.

// Re-export core traits and types
pub use crate::api::api::API;
pub use crate::api::response::{ApiResponse, ErrorResponse};
pub use crate::config::{AppConfig, CorsConfig, Environment, LogFormat, LogLevel, LoggingConfig};
pub use crate::context::Context;
pub use crate::logging;
pub use crate::server::{
    Docs, Endpoint, HttpMethod, PathParam, PathParams, QueryParam, QueryParams, ResponseCallback,
    Route,
};

// Re-export axum types
pub use axum::Json;
pub use axum::extract::{Json as AxumJson, Path, Query, State};
pub use axum::http::{HeaderMap, StatusCode};
pub use axum::response::{IntoResponse, Response};

// Re-export schemars
pub use schemars;

// Re-export aide types
pub use aide::{
    OperationOutput,
    axum::{ApiRouter, routing::*},
    openapi::OpenApi,
};

// Re-export core traits
pub use async_trait::async_trait;
// Re-export derive macros
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};

// Re-export OpenAPI config
pub use crate::openapi::OpenApiConfig;
pub use aide::axum::IntoApiResponse;
