use axum::{
    body::Body,
    extract::Request,
    http::{HeaderName, HeaderValue},
    response::Response,
};
use tower::{Layer, Service};
use uuid::Uuid;

/// Request ID middleware for tracing requests across uncovr applications.
///
/// Generates unique identifiers for each request and adds them to both request and response headers.
/// Preserves existing request IDs when present, enabling distributed tracing across services.
///
/// # Example
///
/// ```rust,no_run
/// use uncovr::server::Server;
/// use uncovr::middleware::RequestId;
///
/// let server = Server::new()
///     .layer(RequestId::new())
///     .register(MyEndpoint)
///     .build();
/// ```
#[derive(Clone)]
pub struct RequestId {
    header_name: String,
}

impl RequestId {
    /// Creates a RequestId middleware with the default header name `x-request-id`.
    pub fn new() -> Self {
        Self {
            header_name: "x-request-id".to_string(),
        }
    }

    /// Creates a RequestId middleware with the specified header name.
    pub fn with_header(header_name: impl Into<String>) -> Self {
        Self {
            header_name: header_name.into(),
        }
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for RequestId {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService {
            inner,
            header_name: self.header_name.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RequestIdService<S> {
    inner: S,
    header_name: String,
}

impl<S> Service<Request<Body>> for RequestIdService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
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

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let header_name = self.header_name.clone();

        // Check if request already has an ID
        let request_id = if let Some(existing_id) = req.headers().get(&header_name) {
            existing_id.clone()
        } else {
            // Generate new UUID
            HeaderValue::from_str(&Uuid::new_v4().to_string())
                .unwrap_or_else(|_| HeaderValue::from_static("unknown"))
        };

        // Add to request headers
        req.headers_mut().insert(
            HeaderName::from_bytes(header_name.as_bytes()).unwrap(),
            request_id.clone(),
        );

        let mut inner = self.inner.clone();
        let header_name_for_response = header_name.clone();

        Box::pin(async move {
            let mut response = inner.call(req).await?;

            // Add request ID to response headers
            response.headers_mut().insert(
                HeaderName::from_bytes(header_name_for_response.as_bytes()).unwrap(),
                request_id,
            );

            Ok(response)
        })
    }
}
