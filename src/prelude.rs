//! Re-exports commonly used types and traits
//!
//! Import everything you need with `use uncovr::prelude::*;`

// Core Uncovr types
pub use crate::api::api::Handler;
pub use crate::api::response::{Error, Response};
pub use crate::config::{App, LogFormat, LogLevel, Logging};
pub use crate::context::Context;
pub use crate::server::params::{Path, Query};
pub use crate::server::{ApiKeyLocation, Endpoint, HttpMethod, Meta, Route, SecurityScheme};

// Core traits
pub use async_trait::async_trait;
pub use serde::{Deserialize, Serialize};

// Axum types
pub use axum::Json;
pub use axum::http::{HeaderMap, StatusCode};
pub use axum::response::IntoResponse;

// Validation
#[cfg(feature = "validation")]
pub use validator::{Validate, ValidationError, ValidationErrors};
