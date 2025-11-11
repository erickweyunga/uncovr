//! Request context types for API handlers.
//!
//! This module provides the `Context` struct which wraps request data,
//! headers, parameters, and provides access to application state.

use axum::http::HeaderMap;
use http::Extensions;
use std::sync::Arc;

use crate::server::params::{Path, Query};

/// Request context passed to API handlers.
///
/// Contains the deserialized request body, HTTP headers, path parameters,
/// query parameters, and provides access to application state through extensions.
///
/// Headers are wrapped in `Arc` for zero-copy sharing across async tasks.
///
/// # Type Parameters
///
/// * `Req` - The request body type. Defaults to `()` for endpoints without a body.
///
/// # Examples
///
/// ```rust,ignore
/// use uncovr::prelude::*;
/// use serde::Deserialize;
///
/// // GET /users/:id
/// #[derive(Clone)]
/// struct GetUser;
///
/// #[async_trait]
/// impl Handler for GetUser {
///     type Request = ();
///     type Response = Result<Json<User>, Error>;
///
///     async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
///         // Access path parameters with type safety
///         let id = ctx.path.parse::<i64>("id")?;
///
///         // Access application state
///         let state = ctx.state::<AppState>();
///
///         let user = fetch_user(&state.db, id).await?;
///         Ok(Json(user))
///     }
/// }
///
/// // GET /users?page=1&limit=10
/// #[derive(Clone)]
/// struct ListUsers;
///
/// #[async_trait]
/// impl Handler for ListUsers {
///     type Request = ();
///     type Response = Json<Vec<User>>;
///
///     async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
///         // Access query parameters with defaults
///         let page = ctx.query.parse::<u32>("page").unwrap_or(1);
///         let limit = ctx.query.parse::<u32>("limit").unwrap_or(10);
///
///         Json(vec![])
///     }
/// }
///
/// // POST /users with JSON body
/// #[derive(Deserialize, JsonSchema, Default)]
/// struct CreateUserRequest {
///     name: String,
///     email: String,
/// }
///
/// #[derive(Clone)]
/// struct CreateUser;
///
/// #[async_trait]
/// impl Handler for CreateUser {
///     type Request = CreateUserRequest;
///     type Response = Result<Json<User>, Error>;
///
///     async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
///         // Access state
///         let state = ctx.state::<AppState>();
///
///         // Access request body
///         let name = &ctx.req.name;
///         let email = &ctx.req.email;
///
///         let user = create_user(&state.db, name, email).await?;
///         Ok(Json(user))
///     }
/// }
/// ```
pub struct Context<Req = ()> {
    /// The deserialized request body (from JSON for POST/PUT/PATCH)
    pub req: Req,

    /// HTTP request headers
    pub headers: Arc<HeaderMap>,

    /// Path parameters extracted from the URL (e.g., `:id` in `/users/:id`)
    pub path: Path,

    /// Query parameters from the URL query string (e.g., `?page=1&limit=10`)
    pub query: Query,

    /// Extensions extracted from the request (e.g., authentication tokens, state)
    pub extensions: Extensions,
}

impl<Req> Context<Req> {
    /// Get application state from the context.
    ///
    /// The state must have been registered with `.with_state()` on the ServerBuilder.
    ///
    /// # Panics
    ///
    /// Panics if the state type was not registered. Make sure to call
    /// `.with_state()` before `.register()` in your server setup.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// #[derive(Clone)]
    /// struct AppState {
    ///     db: Pool<Postgres>,
    /// }
    ///
    /// async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
    ///     let state = ctx.state::<AppState>();
    ///     let users = fetch_users(&state.db).await?;
    ///     Ok(Json(users))
    /// }
    /// ```
    pub fn state<S: Clone + Send + Sync + 'static>(&self) -> S {
        self.extensions
            .get::<S>()
            .cloned()
            .expect("State not found. Did you forget to call .with_state() on the server builder?")
    }

    /// Try to get application state from the context.
    ///
    /// Returns `None` if the state type was not registered.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
    ///     if let Some(state) = ctx.try_state::<AppState>() {
    ///         // Use state
    ///     } else {
    ///         // Handle missing state
    ///     }
    /// }
    /// ```
    pub fn try_state<S: Clone + Send + Sync + 'static>(&self) -> Option<S> {
        self.extensions.get::<S>().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Clone)]
    struct TestState {
        value: i32,
    }

    #[test]
    fn test_state_access() {
        let state = TestState { value: 42 };
        let mut extensions = Extensions::new();
        extensions.insert(state.clone());

        let ctx = Context {
            req: (),
            headers: Arc::new(HeaderMap::new()),
            path: Path::empty(),
            query: Query::empty(),
            extensions,
        };

        let retrieved_state = ctx.state::<TestState>();
        assert_eq!(retrieved_state.value, 42);
    }

    #[test]
    fn test_try_state_some() {
        let state = TestState { value: 42 };
        let mut extensions = Extensions::new();
        extensions.insert(state.clone());

        let ctx = Context {
            req: (),
            headers: Arc::new(HeaderMap::new()),
            path: Path::empty(),
            query: Query::empty(),
            extensions,
        };

        let retrieved_state = ctx.try_state::<TestState>();
        assert!(retrieved_state.is_some());
        assert_eq!(retrieved_state.unwrap().value, 42);
    }

    #[test]
    fn test_try_state_none() {
        let ctx = Context {
            req: (),
            headers: Arc::new(HeaderMap::new()),
            path: Path::empty(),
            query: Query::empty(),
            extensions: Extensions::new(),
        };

        let retrieved_state = ctx.try_state::<TestState>();
        assert!(retrieved_state.is_none());
    }
}
