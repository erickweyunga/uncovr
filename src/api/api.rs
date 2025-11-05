//! Core API traits for implementing endpoints
//!
//! This module provides the core API trait that must be implemented by
//! all endpoints in the application.

use serde::de::DeserializeOwned;

use crate::context::Context;

/// API trait that must be implemented by all endpoints
///
/// This trait defines the core functionality required for API endpoints:
/// - Request type (must be deserializable and implement Default)
/// - Response type (must be serializable)
/// - Handler logic (must return a Send future)
///
/// # Example
///
/// ```rust,no_run
/// use uncovr::prelude::*;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Default)]
/// struct GreetRequest {
///     name: String,
/// }
///
/// #[derive(Serialize)]
/// struct GreetResponse {
///     message: String,
/// }
///
/// struct GreetEndpoint;
///
/// #[async_trait]
/// impl API for GreetEndpoint {
///     type Req = GreetRequest;
///     type Res = GreetResponse;
///
///     async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
///         GreetResponse {
///             message: format!("Hello, {}!", ctx.req.name)
///         }
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait API {
    /// The request type for this endpoint.
    /// Must implement DeserializeOwned and Default.
    type Req: DeserializeOwned + Default + Send + 'static;

    /// The response type for this endpoint.
    /// Must implement Serialize.
    type Res: Send + 'static;

    /// Handler logic for this endpoint.
    ///
    /// Takes a Context containing the deserialized request and headers,
    /// and returns a future that resolves to the response.
    ///
    /// The returned future must be Send to support async execution
    /// across thread boundaries.
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res
    where
        Self: Send + Sync;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
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
    impl API for TestEndpoint {
        type Req = TestRequest;
        type Res = TestResponse;

        async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
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
        };

        let response = endpoint.handler(ctx).await;
        assert_eq!(
            response,
            TestResponse {
                echo: "hello".into()
            }
        );
    }
}
