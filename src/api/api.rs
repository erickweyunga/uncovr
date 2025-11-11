//! Core Handler trait for implementing endpoints
//!
//! This module provides the core Handler trait that must be implemented by
//! all endpoints in the application.

use serde::de::DeserializeOwned;

use crate::context::Context;

/// Handler trait that must be implemented by all endpoints
///
/// This trait defines the core functionality required for API endpoints:
/// - Request type (must be deserializable and implement Default)
/// - Response type (must be convertible to HTTP response)
/// - Handle logic (must return a Send future)
///
/// # Example
///
/// ```rust,no_run
/// use uncovr::prelude::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, JsonSchema, Default)]
/// struct GreetRequest {
///     name: String,
/// }
///
/// #[derive(Serialize, JsonSchema)]
/// struct GreetResponse {
///     message: String,
/// }
///
/// struct GreetEndpoint;
///
/// #[async_trait]
/// impl Handler for GreetEndpoint {
///     type Request = GreetRequest;
///     type Response = Json<GreetResponse>;
///
///     async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
///         Json(GreetResponse {
///             message: format!("Hello, {}!", ctx.req.name)
///         })
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Handler {
    /// The request type for this endpoint.
    /// Must implement DeserializeOwned and Default.
    type Request: DeserializeOwned + Default + Send + 'static;

    /// The response type for this endpoint.
    /// Must implement IntoResponse (from Axum).
    type Response: Send + 'static;

    /// Handle the request for this endpoint.
    ///
    /// Takes a Context containing the deserialized request, path/query parameters,
    /// headers, and state access, and returns a future that resolves to the response.
    ///
    /// The returned future must be Send to support async execution
    /// across thread boundaries.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
    ///     let id = ctx.path.parse::<i64>("id")?;
    ///     let state = ctx.state::<AppState>();
    ///
    ///     let user = fetch_user(&state.db, id).await?;
    ///     Ok(Json(user))
    /// }
    /// ```
    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response
    where
        Self: Send + Sync;
}

/// Legacy type alias for backward compatibility during migration
#[deprecated(since = "0.3.0", note = "Use `Handler` instead")]
pub trait API: Handler {
    type Req;
    type Res;
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res
    where
        Self: Send + Sync;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use http::Extensions;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    #[derive(Default, Deserialize)]
    struct TestRequest {
        message: String,
    }

    #[derive(Serialize, PartialEq, Debug)]
    struct TestResponse {
        echo: String,
    }

    struct TestEndpoint;

    #[async_trait::async_trait]
    impl Handler for TestEndpoint {
        type Request = TestRequest;
        type Response = TestResponse;

        async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
            TestResponse {
                echo: ctx.req.message,
            }
        }
    }

    #[tokio::test]
    async fn test_handler() {
        let endpoint = TestEndpoint;
        let ctx = Context {
            req: TestRequest {
                message: "hello".into(),
            },
            headers: Arc::new(HeaderMap::new()),
            path: crate::server::Path::empty(),
            query: crate::server::Query::empty(),
            extensions: Extensions::new(),
        };

        let response = endpoint.handle(ctx).await;
        assert_eq!(
            response,
            TestResponse {
                echo: "hello".into()
            }
        );
    }
}
