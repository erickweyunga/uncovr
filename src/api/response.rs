//! Type-safe HTTP response handling with automatic OpenAPI documentation.
//!
//! The `Response` enum provides a standardized way to return different HTTP responses
//! from your handlers. The `Error` type provides structured error responses with
//! automatic status code mapping.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response as AxumResponse};
use serde::Serialize;
use std::fmt;

/// Standard response enum for success cases with automatic status code mapping.
///
/// This type handles HTTP status codes, JSON serialization, and OpenAPI documentation
/// automatically.
///
/// # Type Parameters
///
/// * `T` - The success response type (must implement `Serialize`)
///
/// # Examples
///
/// ```rust,ignore
/// use uncovr::prelude::*;
///
/// // Return 200 OK
/// Ok(Response::ok(user))
///
/// // Return 201 Created
/// Ok(Response::created(user))
///
/// // Return 204 No Content
/// Ok(Response::no_content())
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
    /// Create a 200 OK response
    pub fn ok(data: T) -> Self {
        Self::Ok(data)
    }

    /// Create a 201 Created response
    pub fn created(data: T) -> Self {
        Self::Created(data)
    }

    /// Create a 204 No Content response
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

/// Structured error response with automatic HTTP status code mapping.
///
/// This type provides semantic error responses for common HTTP error scenarios.
/// It implements `std::error::Error` and can be used with the `?` operator.
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
///         let id = ctx.path.parse::<i64>("id")?;
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
    /// Create a 400 Bad Request error
    pub fn bad_request(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::BadRequest {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a 400 Bad Request error with details
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

    /// Create a 401 Unauthorized error
    pub fn unauthorized(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Unauthorized {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a 403 Forbidden error
    pub fn forbidden(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Forbidden {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a 404 Not Found error
    pub fn not_found(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::NotFound {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a 409 Conflict error
    pub fn conflict(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Conflict {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a 422 Unprocessable Entity error
    pub fn unprocessable(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::UnprocessableEntity {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a 422 Unprocessable Entity error with details
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

    /// Create a 500 Internal Server Error
    pub fn internal(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InternalError {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create a 503 Service Unavailable error
    pub fn service_unavailable(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Get the HTTP status code for this error
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

/// Legacy type alias for backward compatibility during migration
///
/// This will be removed in v0.4.0
#[deprecated(since = "0.3.0", note = "Use `Error` instead")]
pub type ErrorResponse = Error;
