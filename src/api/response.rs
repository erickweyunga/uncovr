//! Type-safe HTTP response types for uncovr handlers with automatic OpenAPI schema generation.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response as AxumResponse};
use serde::Serialize;
use std::fmt;

/// Success response type with semantic HTTP status codes.
///
/// Provides type-safe response variants for common success scenarios with automatic
/// JSON serialization and OpenAPI schema generation.
///
/// # Examples
///
/// ```rust,ignore
/// use uncovr::prelude::*;
///
/// Response::ok(user)         // 200 OK
/// Response::created(user)    // 201 Created
/// Response::no_content()     // 204 No Content
/// ```
#[derive(Debug, Clone)]
pub enum Response<T> {
    /// 200 OK - Successful response
    Ok(T),
    /// 201 Created - Resource successfully created
    Created(T),
    /// 204 No Content - Success with no response body
    NoContent,
}

impl<T> Response<T> {
    /// Creates a 200 OK response with the provided data.
    pub fn ok(data: T) -> Self {
        Self::Ok(data)
    }

    /// Creates a 201 Created response with the provided data.
    pub fn created(data: T) -> Self {
        Self::Created(data)
    }

    /// Creates a 204 No Content response.
    pub fn no_content() -> Self {
        Self::NoContent
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> AxumResponse {
        match self {
            Response::Ok(data) => (StatusCode::OK, axum::Json(data)).into_response(),
            Response::Created(data) => (StatusCode::CREATED, axum::Json(data)).into_response(),
            Response::NoContent => StatusCode::NO_CONTENT.into_response(),
        }
    }
}

impl<T> aide::OperationOutput for Response<T>
where
    T: schemars::JsonSchema + Serialize,
{
    type Inner = T;

    fn operation_response(
        _ctx: &mut aide::r#gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        None
    }

    fn inferred_responses(
        _ctx: &mut aide::r#gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        Vec::new()
    }
}

/// Structured error response type with semantic HTTP status codes.
///
/// Provides standardized error responses for uncovr handlers with automatic status code
/// mapping and JSON serialization. Implements `std::error::Error` for compatibility with
/// the `?` operator and error conversion traits.
///
/// # Error Conversion Strategy
///
/// uncovr automatically converts common error types to appropriate HTTP responses:
/// - **`ParamError`** → 400 Bad Request
/// - **`serde_json::Error`** → 400 Bad Request
/// - **`ParseIntError`**, **`ParseFloatError`**, **`ParseBoolError`** → 400 Bad Request
/// - **`std::io::Error`** → 500 Internal Server Error (with error logging)
/// - **`validator::ValidationErrors`** → 422 Unprocessable Entity (with field details)
///
/// # Examples
///
/// ```rust,ignore
/// use uncovr::prelude::*;
///
/// #[async_trait]
/// impl Handler for GetUser {
///     type Request = ();
///     type Response = Result<Json<User>, Error>;
///
///     async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
///         let id = ctx.path.parse::<i64>("id")?;  // ParamError auto-converts to 400
///         let user = fetch_user(id).await
///             .map_err(|_| Error::not_found("user_not_found", "User not found"))?;
///         Ok(Json(user))
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum Error {
    /// 400 Bad Request - Invalid request data
    BadRequest {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// 401 Unauthorized - Authentication required
    Unauthorized {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// 403 Forbidden - Insufficient permissions
    Forbidden {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// 404 Not Found - Resource doesn't exist
    NotFound {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// 409 Conflict - Resource conflict
    Conflict {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// 422 Unprocessable Entity - Validation failed
    UnprocessableEntity {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// 500 Internal Server Error - Unexpected error
    InternalError {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
    /// 503 Service Unavailable - Service temporarily unavailable
    ServiceUnavailable {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<serde_json::Value>,
    },
}

impl Error {
    /// Creates a 400 Bad Request error.
    pub fn bad_request(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::BadRequest {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a 400 Bad Request error with additional details.
    pub fn bad_request_with_details<D: Serialize>(
        code: impl Into<String>,
        message: impl Into<String>,
        details: D,
    ) -> Self {
        Self::BadRequest {
            code: code.into(),
            message: message.into(),
            details: serde_json::to_value(details).ok(),
        }
    }

    /// Creates a 401 Unauthorized error.
    pub fn unauthorized(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Unauthorized {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a 403 Forbidden error.
    pub fn forbidden(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Forbidden {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a 404 Not Found error.
    pub fn not_found(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::NotFound {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a 409 Conflict error.
    pub fn conflict(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Conflict {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a 422 Unprocessable Entity error.
    pub fn unprocessable(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::UnprocessableEntity {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a 422 Unprocessable Entity error with validation details.
    pub fn unprocessable_with_details<D: Serialize>(
        code: impl Into<String>,
        message: impl Into<String>,
        details: D,
    ) -> Self {
        Self::UnprocessableEntity {
            code: code.into(),
            message: message.into(),
            details: serde_json::to_value(details).ok(),
        }
    }

    /// Creates a 500 Internal Server Error.
    pub fn internal(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InternalError {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Creates a 503 Service Unavailable error.
    pub fn service_unavailable(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Returns the HTTP status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Error::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            Error::Forbidden { .. } => StatusCode::FORBIDDEN,
            Error::NotFound { .. } => StatusCode::NOT_FOUND,
            Error::Conflict { .. } => StatusCode::CONFLICT,
            Error::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Error::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadRequest { code, message, .. } => {
                write!(f, "Bad Request [{}]: {}", code, message)
            }
            Error::Unauthorized { code, message, .. } => {
                write!(f, "Unauthorized [{}]: {}", code, message)
            }
            Error::Forbidden { code, message, .. } => {
                write!(f, "Forbidden [{}]: {}", code, message)
            }
            Error::NotFound { code, message, .. } => {
                write!(f, "Not Found [{}]: {}", code, message)
            }
            Error::Conflict { code, message, .. } => {
                write!(f, "Conflict [{}]: {}", code, message)
            }
            Error::UnprocessableEntity { code, message, .. } => {
                write!(f, "Unprocessable Entity [{}]: {}", code, message)
            }
            Error::InternalError { code, message, .. } => {
                write!(f, "Internal Server Error [{}]: {}", code, message)
            }
            Error::ServiceUnavailable { code, message, .. } => {
                write!(f, "Service Unavailable [{}]: {}", code, message)
            }
        }
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> AxumResponse {
        let status = self.status_code();
        (status, axum::Json(self)).into_response()
    }
}

impl aide::OperationOutput for Error {
    type Inner = Self;

    fn operation_response(
        _ctx: &mut aide::r#gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        None
    }

    fn inferred_responses(
        _ctx: &mut aide::r#gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        Vec::new()
    }
}

// Automatic error conversions for uncovr handlers

impl From<crate::server::params::ParamError> for Error {
    fn from(err: crate::server::params::ParamError) -> Self {
        Error::bad_request("invalid_parameter", err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::bad_request("json_parse_error", format!("Failed to parse JSON: {}", err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        tracing::error!("I/O error: {}", err);
        Error::internal("io_error", "Internal server error")
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::bad_request("parse_error", format!("Failed to parse number: {}", err))
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::bad_request("parse_error", format!("Failed to parse number: {}", err))
    }
}

impl From<std::str::ParseBoolError> for Error {
    fn from(err: std::str::ParseBoolError) -> Self {
        Error::bad_request("parse_error", format!("Failed to parse boolean: {}", err))
    }
}

#[cfg(feature = "validation")]
impl From<validator::ValidationErrors> for Error {
    fn from(errors: validator::ValidationErrors) -> Self {
        use std::collections::HashMap;

        let mut field_errors = HashMap::new();

        for (field, errors) in errors.field_errors() {
            let messages: Vec<String> = errors
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();

            if !messages.is_empty() {
                field_errors.insert(field.to_string(), messages);
            }
        }

        Error::unprocessable_with_details(
            "validation_failed",
            "Request validation failed",
            field_errors,
        )
    }
}
