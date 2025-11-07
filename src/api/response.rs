//! Type-safe HTTP response handling with automatic OpenAPI documentation.
//!
//! The `ApiResponse` enum provides a standardized way to return different HTTP responses
//! from your handlers. Only the response types you actually use will appear in the
//! OpenAPI documentation.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use serde::Serialize;

/// Standard API response for success and error cases.
///
/// This type handles HTTP status codes, JSON serialization, and OpenAPI documentation
/// automatically. Only response variants you actually return will be documented.
///
/// # Type Parameters
///
/// * `T` - The success response type (must implement `Serialize` + `JsonSchema`)
///
/// # Examples
///
/// ```rust,no_run
/// use uncovr::prelude::*;
/// use serde::{Serialize, Deserialize};
/// use schemars::JsonSchema;
///
/// #[derive(Serialize, Deserialize, JsonSchema, Default)]
/// struct UserRequest {
///     id: u64,
/// }
///
/// #[derive(Serialize, JsonSchema)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// #[derive(Clone)]
/// struct GetUserApi;
///
/// impl Metadata for GetUserApi {
///     fn metadata(&self) -> Endpoint {
///         Endpoint::new("/users/:id", "get")
///     }
/// }
///
/// #[async_trait]
/// impl API for GetUserApi {
///     type Req = UserRequest;
///     type Res = ApiResponse<User>;
///
///     async fn handler(&self, ctx: Context<Self::Req>) -> ApiResponse<User> {
///         if ctx.req.id == 0 {
///             return ApiResponse::BadRequest {
///                 code: "invalid_id",
///                 message: "ID must be greater than 0",
///             };
///         }
///
///         ApiResponse::Ok(User { id: ctx.req.id, name: "John".into() })
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub enum ApiResponse<T> {
    // Success responses (2xx)
    /// 200 OK - Successful response
    Ok(T),
    /// 201 Created - Resource successfully created
    Created(T),
    /// 204 No Content - Success with no response body
    NoContent,

    // Redirect responses (3xx)
    /// 301 Moved Permanently - Permanent redirect
    MovedPermanently(String),
    /// 302 Found - Temporary redirect
    Found(String),
    /// 303 See Other - Redirect to different resource
    SeeOther(String),
    /// 307 Temporary Redirect - Temporary redirect preserving method
    TemporaryRedirect(String),
    /// 308 Permanent Redirect - Permanent redirect preserving method
    PermanentRedirect(String),

    // Client error responses (4xx)
    /// 400 Bad Request - Invalid request data
    BadRequest { code: &'static str, message: String },
    /// 401 Unauthorized - Authentication required
    Unauthorized { code: &'static str, message: String },
    /// 403 Forbidden - Insufficient permissions
    Forbidden { code: &'static str, message: String },
    /// 404 Not Found - Resource doesn't exist
    NotFound { code: &'static str, message: String },
    /// 409 Conflict - Resource conflict
    Conflict { code: &'static str, message: String },
    /// 422 Unprocessable Entity - Validation failed
    UnprocessableEntity { code: &'static str, message: String },
    /// Custom client error with error details
    ClientError { status: u16, error: ErrorResponse },

    // Server error responses (5xx)
    /// 500 Internal Server Error - Unexpected error
    InternalError { code: &'static str, message: String },
    /// 503 Service Unavailable - Service temporarily unavailable
    ServiceUnavailable { code: &'static str, message: String },
    /// Custom server error with error details
    ServerError { status: u16, error: ErrorResponse },
}

#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    /// Create a new error response with code and message
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create an error response with additional structured details
    pub fn with_details<D: Serialize>(
        code: impl Into<String>,
        message: impl Into<String>,
        details: D,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: serde_json::to_value(details).ok(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Ok(data) => (StatusCode::OK, axum::Json(data)).into_response(),
            ApiResponse::Created(data) => (StatusCode::CREATED, axum::Json(data)).into_response(),
            ApiResponse::NoContent => StatusCode::NO_CONTENT.into_response(),

            ApiResponse::MovedPermanently(uri) => Redirect::permanent(&uri).into_response(),
            ApiResponse::Found(uri) => Redirect::temporary(&uri).into_response(),
            ApiResponse::SeeOther(uri) => Redirect::to(&uri).into_response(),
            ApiResponse::TemporaryRedirect(uri) => {
                (StatusCode::TEMPORARY_REDIRECT, [("Location", uri)]).into_response()
            }
            ApiResponse::PermanentRedirect(uri) => {
                (StatusCode::PERMANENT_REDIRECT, [("Location", uri)]).into_response()
            }

            ApiResponse::BadRequest { code, message } => (
                StatusCode::BAD_REQUEST,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::Unauthorized { code, message } => (
                StatusCode::UNAUTHORIZED,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::Forbidden { code, message } => (
                StatusCode::FORBIDDEN,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::NotFound { code, message } => (
                StatusCode::NOT_FOUND,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::Conflict { code, message } => (
                StatusCode::CONFLICT,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::UnprocessableEntity { code, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::ClientError { status, error } => {
                let status_code = StatusCode::from_u16(status).unwrap_or(StatusCode::BAD_REQUEST);
                (status_code, axum::Json(error)).into_response()
            }

            ApiResponse::InternalError { code, message } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::ServiceUnavailable { code, message } => (
                StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(ErrorResponse::new(code, message)),
            )
                .into_response(),
            ApiResponse::ServerError { status, error } => {
                let status_code =
                    StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                (status_code, axum::Json(error)).into_response()
            }
        }
    }
}

impl<T> aide::OperationOutput for ApiResponse<T>
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
