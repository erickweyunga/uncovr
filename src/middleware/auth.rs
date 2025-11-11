use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::future::Future;
use tower::{Layer, Service};

/// Bearer token authentication middleware for uncovr endpoints.
///
/// Validates Bearer tokens from the `Authorization` header using an async validation function.
/// Returns HTTP 401 for missing, malformed, or invalid tokens.
///
/// # Implementation
///
/// - Extracts tokens from `Authorization: Bearer <token>` headers
/// - Invokes the provided validator with the extracted token
/// - Allows requests through on successful validation
/// - Returns 401 with the validator's error message on failure
///
/// # Example
///
/// ```rust,no_run
/// use uncovr::server::Server;
/// use uncovr::middleware::BearerAuth;
///
/// async fn validate_token(token: String) -> Result<(), String> {
///     if token == "secret_token" {
///         Ok(())
///     } else {
///         Err("Invalid token".to_string())
///     }
/// }
///
/// let server = Server::new()
///     .layer(BearerAuth::new(validate_token))
///     .register(MyEndpoint)
///     .build();
/// ```
#[derive(Clone)]
pub struct BearerAuth<F> {
    validator: F,
}

impl<F, Fut> BearerAuth<F>
where
    F: Fn(String) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    /// Creates a Bearer authentication middleware with the specified validator function.
    pub fn new(validator: F) -> Self {
        Self { validator }
    }
}

impl<S, F, Fut> Layer<S> for BearerAuth<F>
where
    F: Fn(String) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    type Service = BearerAuthService<S, F>;

    fn layer(&self, inner: S) -> Self::Service {
        BearerAuthService {
            inner,
            validator: self.validator.clone(),
        }
    }
}

#[derive(Clone)]
pub struct BearerAuthService<S, F> {
    inner: S,
    validator: F,
}

impl<S, F, Fut> Service<Request<Body>> for BearerAuthService<S, F>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    F: Fn(String) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // Extract Authorization header
        let auth_header = req
            .headers()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Extract Bearer token
        let token = auth_header.and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(header[7..].to_string())
            } else {
                None
            }
        });

        let Some(token) = token else {
            // No token provided
            let response = (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header. Expected: Bearer <token>",
            )
                .into_response();

            return Box::pin(async move { Ok(response) });
        };

        // Validate token
        let validator = self.validator.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            match validator(token).await {
                Ok(()) => {
                    // Token is valid, continue with request
                    inner.call(req).await
                }
                Err(error) => {
                    // Token is invalid
                    let response = (
                        StatusCode::UNAUTHORIZED,
                        format!("Authentication failed: {}", error),
                    )
                        .into_response();
                    Ok(response)
                }
            }
        })
    }
}
