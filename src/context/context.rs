//! Request context types for API handlers.
//!
//! This module provides the `Context` struct which wraps request data
//! and headers, providing a consistent interface for endpoint handlers.

use axum::http::HeaderMap;
use http::Extensions;
use std::sync::Arc;

use crate::server::params::{PathParams, QueryParams};

/// Request context passed to API handlers.
///
/// Contains the deserialized request body, HTTP headers, path parameters,
/// and query parameters, providing access to all request information needed by handlers.
///
/// Headers are wrapped in `Arc` for zero-copy sharing across async tasks.
///
/// # Type Parameters
///
/// * `Req` - The request body type. Defaults to `()` for endpoints without a body.
///
/// # Examples
///
/// ```rust
/// use uncovr::prelude::*;
/// use serde::Deserialize;
///
/// // GET /users/:id
/// #[derive(Clone)]
/// struct GetUser;
///
/// #[async_trait]
/// impl API for GetUser {
///     type Req = ();
///     type Res = String;
///
///     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
///         // Access path parameters
///         let id = ctx.path.get_u64("id").unwrap_or(0);
///         format!("User {}", id)
///     }
/// }
///
/// // GET /users?page=1&limit=10
/// #[derive(Clone)]
/// struct ListUsers;
///
/// #[async_trait]
/// impl API for ListUsers {
///     type Req = ();
///     type Res = String;
///
///     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
///         // Access query parameters
///         let page = ctx.query.get_u32("page").unwrap_or(1);
///         let limit = ctx.query.get_u32("limit").unwrap_or(10);
///         format!("Page {} with {} items", page, limit)
///     }
/// }
///
/// // POST /users/:id with JSON body
/// #[derive(Deserialize, Default)]
/// struct UpdateUserBody {
///     name: String,
///     email: String,
/// }
///
/// #[derive(Clone)]
/// struct UpdateUser;
///
/// #[async_trait]
/// impl API for UpdateUser {
///     type Req = UpdateUserBody;
///     type Res = String;
///
///     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
///         // Access path parameter
///         let id = ctx.path.get_u64("id").unwrap_or(0);
///
///         // Access request body
///         let name = &ctx.req.name;
///         let email = &ctx.req.email;
///
///         format!("Updated user {} with {} <{}>", id, name, email)
///     }
/// }
/// ```
pub struct Context<Req = ()> {
    /// The deserialized request body (from JSON for POST/PUT/PATCH)
    pub req: Req,

    /// HTTP request headers
    pub headers: Arc<HeaderMap>,

    /// Path parameters extracted from the URL (e.g., `:id` in `/users/:id`)
    pub path: PathParams,

    /// Query parameters from the URL query string (e.g., `?page=1&limit=10`)
    pub query: QueryParams,

    /// Extensions extracted from the request (e.g., authentication tokens)
    pub extensions: Extensions,
}
