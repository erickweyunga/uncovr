//! Re-exports commonly used types and traits
//!
//! This module provides a convenient way to import all the common types and traits
//! needed to build APIs with uncover.

// Re-export core traits and types
pub use crate::api::api::Handler;
pub use crate::api::response::{Error, Response};
pub use crate::config::{AppConfig, CorsConfig, Environment, LogFormat, LogLevel, LoggingConfig};
pub use crate::context::Context;
pub use crate::logging;
pub use crate::server::{
    Endpoint, HttpMethod, Meta, PathParam, QueryParam, ResponseCallback, Route,
};

// Re-export parameter types (avoid conflict with axum::extract::Path/Query)
pub use crate::server::params::{Path as PathParams, Query as QueryParams};

// Re-export axum types
pub use axum::Json;
pub use axum::extract::{Path as AxumPath, Query as AxumQuery, State};
pub use axum::http::{HeaderMap, StatusCode};
pub use axum::response::{IntoResponse, Response as AxumResponse};

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
