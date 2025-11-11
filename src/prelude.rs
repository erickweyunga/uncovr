//! Re-exports commonly used types and traits
//!
//! Import everything you need with `use uncovr::prelude::*;`

// Core Uncovr types - no renaming
pub use crate::api::api::Handler;
pub use crate::api::response::{Error, Response};
pub use crate::config::{App, LogFormat, LogLevel, Logging};
pub use crate::context::Context;
pub use crate::server::params::{Path, Query};
pub use crate::server::{Endpoint, HttpMethod, Meta, Route};

// Core traits
pub use async_trait::async_trait;
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};

// Axum types (can rename to avoid conflicts)
pub use axum::Json;
pub use axum::http::{HeaderMap, StatusCode};
pub use axum::response::IntoResponse;

// Validation (optional)
#[cfg(feature = "validation")]
pub use validator::{Validate, ValidationError, ValidationErrors};
