//! Handler trait for implementing endpoint business logic in uncovr applications.

use serde::de::DeserializeOwned;

use crate::context::Context;

/// Handler trait for implementing endpoint business logic.
///
/// Defines the request processing logic for uncovr endpoints. Each handler specifies
/// its request and response types and implements asynchronous request handling.
///
/// # Type Safety
///
/// - **Request**: Must be deserializable and have a default (for GET requests without body)
/// - **Response**: Must be convertible to HTTP response
/// - **Async**: Returns a `Send` future for async runtime compatibility
///
/// # Context Access
///
/// Handlers receive a [`Context<Request>`] providing access to:
/// - Deserialized request body via `ctx.req`
/// - Path parameters via `ctx.path.get()` or `ctx.path.parse()`
/// - Query parameters via `ctx.query.get()` or `ctx.query.parse()`
/// - Application state via `ctx.state::<T>()`
/// - HTTP headers via `ctx.headers`
///
/// # Example
///
/// ```rust,no_run
/// use uncovr::prelude::*;
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
    /// Request body type for this handler.
    type Request: DeserializeOwned + Default + Send + 'static;

    /// Response type for this handler.
    type Response: Send + 'static;

    /// Processes the request and returns a response.
    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response
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
