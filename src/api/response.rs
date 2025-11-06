//! Standard API response types for handling success and error cases.
//!
//! The `ApiResponse` enum provides a type-safe way to return different HTTP responses
//! from your handlers with automatic OpenAPI documentation.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// Standard API response envelope for success and error cases.
///
/// This type automatically handles HTTP status codes, JSON serialization,
/// and OpenAPI documentation for all response variants.
///
/// # Type Parameters
///
/// * `T` - The success response type (must implement `Serialize` + `JsonSchema`)
///
/// # Examples
///
/// ## Simple success/error handling
///
/// ```rust
/// use uncovr::prelude::*;
///
/// #[derive(Serialize, JsonSchema)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// #[async_trait]
/// impl API for GetUser {
///     type Req = ();
///     type Res = ApiResponse<User>;
///
///     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
///         if let Some(user) = find_user(42) {
///             ApiResponse::Ok(user)
///         } else {
///             ApiResponse::NotFound("User not found")
///         }
///     }
/// }
/// ```
///
/// ## Validation errors
///
/// ```rust
/// use uncovr::prelude::*;
///
/// async fn handler(&self, ctx: Context<Self::Req>) -> ApiResponse<Response> {
///     if ctx.req.email.is_empty() {
///         return ApiResponse::BadRequest("Email is required");
///     }
///
///     if !is_valid_email(&ctx.req.email) {
///         return ApiResponse::BadRequest("Invalid email format");
///     }
///
///     ApiResponse::Ok(Response { success: true })
/// }
/// ```
///
/// ## Custom error details
///
/// ```rust
/// use uncovr::prelude::*;
///
/// #[derive(Serialize, JsonSchema)]
/// struct ValidationErrors {
///     errors: Vec<FieldError>,
/// }
///
/// async fn handler(&self, ctx: Context<Self::Req>) -> ApiResponse<User> {
///     let errors = validate_input(&ctx.req);
///     if !errors.is_empty() {
///         return ApiResponse::BadRequestWithDetails(ValidationErrors { errors });
///     }
///
///     ApiResponse::Ok(create_user(ctx.req))
/// }
/// ```
#[derive(Debug, Clone)]
pub enum ApiResponse<T> {
    // Success responses (2xx)
    /// 200 OK - Standard success response with data
    Ok(T),

    /// 201 Created - Resource successfully created
    Created(T),

    /// 204 No Content - Success with no response body
    NoContent,

    // Client error responses (4xx)
    /// 400 Bad Request - Invalid request data
    BadRequest(&'static str),

    /// 400 Bad Request with custom error details
    BadRequestWithDetails(ErrorDetails),

    /// 401 Unauthorized - Authentication required
    Unauthorized(&'static str),

    /// 403 Forbidden - Insufficient permissions
    Forbidden(&'static str),

    /// 404 Not Found - Resource doesn't exist
    NotFound(&'static str),

    /// 409 Conflict - Resource conflict (e.g., duplicate)
    Conflict(&'static str),

    /// 422 Unprocessable Entity - Validation failed
    UnprocessableEntity(&'static str),

    /// 422 Unprocessable Entity with validation details
    UnprocessableEntityWithDetails(ErrorDetails),

    // Server error responses (5xx)
    /// 500 Internal Server Error - Unexpected server error
    InternalError(&'static str),

    /// 503 Service Unavailable - Service temporarily unavailable
    ServiceUnavailable(&'static str),
}

/// Error details for custom error responses.
///
/// This type allows you to provide structured error information
/// beyond simple string messages.
///
/// # Examples
///
/// ```rust
/// use uncovr::prelude::*;
///
/// #[derive(Serialize, JsonSchema)]
/// struct ValidationError {
///     field: String,
///     message: String,
/// }
///
/// let errors = vec![
///     ValidationError {
///         field: "email".to_string(),
///         message: "Invalid email format".to_string(),
///     }
/// ];
///
/// return ApiResponse::BadRequestWithDetails(
///     ErrorDetails::new("validation_failed", errors)
/// );
/// ```
#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
pub struct ErrorDetails {
    /// Error code/type identifier
    pub error: String,

    /// Human-readable error message
    pub message: String,

    /// Optional additional details (any JSON-serializable data)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorDetails {
    /// Create a new error with code and message
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create an error with additional structured details
    pub fn with_details<D: Serialize>(
        error: impl Into<String>,
        message: impl Into<String>,
        details: D,
    ) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            details: serde_json::to_value(details).ok(),
        }
    }
}

/// Simple error response format for string-based errors
#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
struct SimpleError {
    error: String,
    message: String,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self {
            // Success responses
            ApiResponse::Ok(data) => (StatusCode::OK, axum::Json(data)).into_response(),
            ApiResponse::Created(data) => (StatusCode::CREATED, axum::Json(data)).into_response(),
            ApiResponse::NoContent => StatusCode::NO_CONTENT.into_response(),

            // Client errors
            ApiResponse::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                axum::Json(SimpleError {
                    error: "bad_request".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),

            ApiResponse::BadRequestWithDetails(details) => {
                (StatusCode::BAD_REQUEST, axum::Json(details)).into_response()
            }

            ApiResponse::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                axum::Json(SimpleError {
                    error: "unauthorized".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),

            ApiResponse::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                axum::Json(SimpleError {
                    error: "forbidden".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),

            ApiResponse::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                axum::Json(SimpleError {
                    error: "not_found".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),

            ApiResponse::Conflict(msg) => (
                StatusCode::CONFLICT,
                axum::Json(SimpleError {
                    error: "conflict".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),

            ApiResponse::UnprocessableEntity(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                axum::Json(SimpleError {
                    error: "unprocessable_entity".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),

            ApiResponse::UnprocessableEntityWithDetails(details) => {
                (StatusCode::UNPROCESSABLE_ENTITY, axum::Json(details)).into_response()
            }

            // Server errors
            ApiResponse::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(SimpleError {
                    error: "internal_error".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),

            ApiResponse::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(SimpleError {
                    error: "service_unavailable".to_string(),
                    message: msg.to_string(),
                }),
            )
                .into_response(),
        }
    }
}

// Implement OperationOutput for OpenAPI documentation
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
        ctx: &mut aide::r#gen::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        use aide::openapi::{Response as ApiDocResponse, SchemaObject};

        let mut responses = Vec::new();

        // Generate schema for success type T
        let success_schema = ctx.schema.subschema_for::<T>();

        // 200 OK response
        responses.push((
            Some(200),
            ApiDocResponse {
                description: "Success".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: success_schema.clone(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 201 Created response
        responses.push((
            Some(201),
            ApiDocResponse {
                description: "Resource created successfully".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: success_schema,
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // Error response schema
        let error_schema = ctx.schema.subschema_for::<SimpleError>();
        let error_details_schema = ctx.schema.subschema_for::<ErrorDetails>();

        // 400 Bad Request
        responses.push((
            Some(400),
            ApiDocResponse {
                description: "Bad request - invalid input data".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: error_schema.clone(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 401 Unauthorized
        responses.push((
            Some(401),
            ApiDocResponse {
                description: "Unauthorized - authentication required".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: error_schema.clone(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 403 Forbidden
        responses.push((
            Some(403),
            ApiDocResponse {
                description: "Forbidden - insufficient permissions".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: error_schema.clone(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 404 Not Found
        responses.push((
            Some(404),
            ApiDocResponse {
                description: "Not found - resource doesn't exist".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: error_schema.clone(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 409 Conflict
        responses.push((
            Some(409),
            ApiDocResponse {
                description: "Conflict - resource already exists or state conflict".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: error_schema.clone(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 422 Unprocessable Entity
        responses.push((
            Some(422),
            ApiDocResponse {
                description: "Unprocessable entity - validation failed".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: error_details_schema.clone(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 500 Internal Server Error
        responses.push((
            Some(500),
            ApiDocResponse {
                description: "Internal server error".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: error_schema,
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        // 503 Service Unavailable
        responses.push((
            Some(503),
            ApiDocResponse {
                description: "Service unavailable - temporary maintenance or overload".to_string(),
                content: [(
                    "application/json".to_string(),
                    aide::openapi::MediaType {
                        schema: Some(SchemaObject {
                            json_schema: ctx.schema.subschema_for::<SimpleError>(),
                            example: None,
                            external_docs: None,
                        }),
                        ..Default::default()
                    },
                )]
                .into(),
                ..Default::default()
            },
        ));

        responses
    }
}
