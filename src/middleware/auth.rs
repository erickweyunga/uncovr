use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::future::Future;
use tower::{Layer, Service};

/// Bearer token authentication middleware
///
/// Validates Bearer tokens from the Authorization header using a custom validation function.
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
    /// Create a new Bearer authentication middleware
    ///
    /// # Arguments
    ///
    /// * `validator` - Async function that validates the token and returns `Ok(())` if valid
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::BearerAuth;
    ///
    /// async fn my_validator(token: String) -> Result<(), String> {
    ///     // Validate JWT, check database, etc.
    ///     if token.starts_with("valid_") {
    ///         Ok(())
    ///     } else {
    ///         Err("Token validation failed".to_string())
    ///     }
    /// }
    ///
    /// let auth = BearerAuth::new(my_validator);
    /// ```
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

/// Create a token validator from a list of valid tokens
///
/// # Example
///
/// ```rust
/// use uncovr::middleware::auth::token_validator;
///
/// let validator = token_validator(vec![
///     "token1".to_string(),
///     "token2".to_string(),
/// ]);
/// ```
pub fn token_validator(
    valid_tokens: Vec<String>,
) -> impl Fn(String) -> std::future::Ready<Result<(), String>> + Clone {
    move |token: String| {
        if valid_tokens.contains(&token) {
            std::future::ready(Ok(()))
        } else {
            std::future::ready(Err("Invalid token".to_string()))
        }
    }
}
