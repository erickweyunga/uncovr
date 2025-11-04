//! Request context types for API handlers.
//!
//! This module provides the `Context` struct which wraps request data
//! and headers, providing a consistent interface for endpoint handlers.

use axum::http::HeaderMap;

/// Request context passed to API handlers.
///
/// Contains the deserialized request body and HTTP headers,
/// providing access to all request information needed by handlers.
///
/// # Type Parameters
///
/// * `Req` - The request body type. Defaults to `()` for endpoints without a body.
///
/// # Examples
///
/// ```rust
/// use uncover::prelude::*;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Default)]
/// struct CreateUserRequest {
///     name: String,
///     email: String,
/// }
///
/// #[derive(Clone)]
/// struct CreateUserEndpoint;
///
/// #[async_trait]
/// impl API for CreateUserEndpoint {
///     type Req = CreateUserRequest;
///     type Res = String;
///
///     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
///         // Access request body
///         let name = &ctx.req.name;
///
///         // Access headers
///         let user_agent = ctx.headers
///             .get("user-agent")
///             .and_then(|v| v.to_str().ok())
///             .unwrap_or("unknown");
///
///         format!("User {} from {}", name, user_agent)
///     }
/// }
/// ```
#[derive(Clone)]
pub struct Context<Req = ()> {
    /// The deserialized request body
    pub req: Req,

    /// HTTP request headers
    pub headers: HeaderMap,
}
