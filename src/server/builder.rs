use std::sync::Arc;
use std::time::Duration;
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

use aide::OperationIo;
use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, patch_with, post_with, put_with},
};
use aide::openapi::{
    Parameter, ParameterData, ParameterSchemaOrContent, QueryStyle, ReferenceOr, SchemaObject,
};
use axum::{Extension, async_trait, body::Body};
use axum::{
    extract::FromRequestParts,
    http::{HeaderValue, Method, Request, Response, request::Parts},
    response::Response as AxumResponse,
};
use http::Extensions;
use schemars::schema::{InstanceType, Schema, SchemaObject as SchemarsSchemaObject};
use tower::Service;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::{MakeSpan, OnResponse, TraceLayer};

use crate::api::api::API;
use crate::config::{AppConfig, CorsConfig};
use crate::context::Context;
use crate::openapi::{OpenApiConfig, serve_docs, serve_scalar_ui};
use crate::server::endpoint::Endpoint as EndpointTrait;
use crate::server::params::{PathParams, QueryParams};

/// Custom extractor for HTTP Extensions.
///
/// This extractor allows us to extract the entire Extensions map from the request.
#[derive(OperationIo)]
#[aide(input)]
struct ExtractExtensions(Extensions);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractExtensions
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(ExtractExtensions(parts.extensions.clone()))
    }
}

/// Custom request span maker that logs essential request information.
///
/// Creates a tracing span for each HTTP request with:
/// - HTTP method (GET, POST, etc.)
/// - Request URI path
/// - User agent string
///
/// This provides clean, readable logs without exposing all headers.
#[derive(Clone)]
struct RequestSpanMaker;

impl<B> MakeSpan<B> for RequestSpanMaker {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        let method = request.method();
        let uri = request.uri().path();
        let user_agent = request
            .headers()
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("-");

        tracing::info_span!(
            "request",
            method = %method,
            uri = %uri,
            user_agent = %user_agent,
        )
    }
}

/// Custom response logger that logs request completion with appropriate severity.
///
/// Logs HTTP request completion with:
/// - Status code
/// - Latency in milliseconds
///
/// Log levels based on response status:
/// - 5xx errors: ERROR level
/// - 4xx errors: WARN level
/// - 2xx/3xx success: INFO level
#[derive(Clone)]
struct RequestLogger;

impl<B> OnResponse<B> for RequestLogger {
    fn on_response(
        self,
        response: &Response<B>,
        latency: std::time::Duration,
        _span: &tracing::Span,
    ) {
        let status = response.status();
        let latency_ms = latency.as_millis();

        if status.is_server_error() {
            tracing::error!(
                status = %status,
                latency_ms = %latency_ms,
                "Request failed"
            );
        } else if status.is_client_error() {
            tracing::warn!(
                status = %status,
                latency_ms = %latency_ms,
                "Request error"
            );
        } else {
            tracing::info!(
                status = %status,
                latency_ms = %latency_ms,
                "Request completed"
            );
        }
    }
}

/// Main HTTP server that serves the configured API.
///
/// This struct represents a fully configured HTTP server ready to serve requests.
/// It's created via the [`ServerBuilder`] and contains the compiled router and bind address.
///
/// # Examples
///
/// ```rust,no_run
/// use uncovr::server::Server;
/// use uncovr::config::AppConfig;
///
/// #[tokio::main]
/// async fn main() {
///     let config = AppConfig::new("My API", "1.0.0");
///
///     Server::new()
///         .with_config(config)
///         .serve()
///         .await
///         .expect("Server failed");
/// }
/// ```
pub struct Server {
    router: ApiRouter,
    address: SocketAddr,
}

impl Server {
    /// Create a new server builder
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        ServerBuilder::default()
    }

    /// Consumes the server and returns the underlying router.
    pub fn into_router(self) -> ApiRouter {
        self.router
    }

    /// Start serving the application
    pub async fn serve(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.address).await?;
        tracing::info!("Server running: http://{}", self.address);
        tracing::info!("Press Ctrl+C to stop");

        axum::serve(listener, self.router.into_make_service())
            .await
            .map_err(std::io::Error::other)
    }
}

/// Builder for configuring and creating an HTTP server.
///
/// Provides a fluent API for configuring the server with endpoints, middleware,
/// OpenAPI documentation, CORS, logging, and more.
///
/// # Examples
///
/// ```rust,no_run
/// use uncovr::prelude::*;
/// use uncovr::server::Server;
///
/// #[derive(Clone)]
/// struct HelloEndpoint;
///
/// impl Metadata for HelloEndpoint {
///     fn metadata(&self) -> Endpoint {
///         Endpoint::new("/hello", "get")
///             .summary("Say hello")
///     }
/// }
///
/// #[async_trait]
/// impl API for HelloEndpoint {
///     type Req = ();
///     type Res = &'static str;
///
///     async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
///         "Hello, World!"
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let config = AppConfig::new("Hello API", "1.0.0")
///         .logging(LoggingConfig::development());
///
///     Server::new()
///         .with_config(config)
///         .register(HelloEndpoint)
///         .serve()
///         .await
///         .expect("Server failed");
/// }
/// ```
pub struct ServerBuilder {
    router: ApiRouter,
    address: String,
    openapi: Option<aide::openapi::OpenApi>,
    config: Option<AppConfig>,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            router: ApiRouter::new(),
            address: "127.0.0.1:3000".to_string(),
            openapi: None,
            config: None,
        }
    }
}

/// Parameter information for OpenAPI documentation (internal helper)
#[derive(Debug, Clone)]
struct ParamInfo {
    /// Parameter name
    name: &'static str,
    /// Parameter description
    description: Option<&'static str>,
    /// Whether the parameter is required
    required: bool,
}

/// Helper function to convert ParamInfo to aide's Parameter type for OpenAPI
fn param_info_to_query_param(param: &ParamInfo) -> ReferenceOr<Parameter> {
    ReferenceOr::Item(Parameter::Query {
        parameter_data: ParameterData {
            name: param.name.to_string(),
            description: param.description.map(|s| s.to_string()),
            required: param.required,
            deprecated: None,
            format: ParameterSchemaOrContent::Schema(SchemaObject {
                json_schema: Schema::Object(SchemarsSchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    ..Default::default()
                }),
                external_docs: None,
                example: None,
            }),
            example: None,
            examples: Default::default(),
            explode: None,
            extensions: Default::default(),
        },
        allow_reserved: false,
        style: QueryStyle::Form,
        allow_empty_value: None,
    })
}

impl ServerBuilder {
    /// Configure the server with an AppConfig
    pub fn with_config(mut self, config: AppConfig) -> Self {
        // Set address from config
        self.address = config.bind_address.clone();

        // Configure OpenAPI if enabled
        if config.enable_docs {
            let mut openapi_config =
                OpenApiConfig::new(&config.name, &config.version).description(&config.description);

            // Add servers from config, or use bind address if no servers configured
            if config.api_servers.is_empty() {
                // Automatically derive server URL from bind address
                let server_url = if config.bind_address.starts_with("0.0.0.0:") {
                    format!(
                        "http://localhost:{}",
                        config.bind_address.strip_prefix("0.0.0.0:").unwrap()
                    )
                } else if config.bind_address.starts_with("127.0.0.1:")
                    || config.bind_address.starts_with("localhost:")
                {
                    format!("http://{}", config.bind_address)
                } else {
                    // For any other address (including domain names), use http://
                    format!("http://{}", config.bind_address)
                };

                openapi_config = openapi_config.server(server_url, "API Server");
            } else {
                // Use explicitly configured servers
                for server in &config.api_servers {
                    openapi_config = openapi_config.server(&server.url, &server.description);
                }
            }

            self.openapi = Some(openapi_config.build());
        }

        self.config = Some(config);
        self
    }

    /// Configure OpenAPI documentation
    pub fn with_openapi(mut self, config: OpenApiConfig) -> Self {
        let api = config.build();
        self.openapi = Some(api);
        self
    }

    /// Register an API endpoint.
    ///
    /// This method uses the new Endpoint trait that separates route definition from documentation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::server::Server;
    /// use uncovr::server::endpoint::{Endpoint, Route, Docs};
    /// use uncovr::api::API;
    ///
    /// #[derive(Clone)]
    /// struct CreateUser;
    ///
    /// impl Endpoint for CreateUser {
    ///     fn ep(&self) -> Route {
    ///         Route::POST("/users")
    ///     }
    ///
    ///     fn docs(&self) -> Option<Docs> {
    ///         Some(Docs::new().summary("Create a user"))
    ///     }
    /// }
    ///
    /// // Also implement API trait...
    /// ```
    pub fn register<E>(mut self, endpoint: E) -> Self
    where
        E: EndpointTrait + API + Send + Sync + 'static,
        E::Req: serde::de::DeserializeOwned + schemars::JsonSchema + Default + Send + 'static,
        E::Res: aide::OperationOutput + axum::response::IntoResponse + Send + 'static,
        <E::Res as aide::OperationOutput>::Inner: schemars::JsonSchema,
    {
        let route_def = endpoint.ep();
        let docs = endpoint.docs();

        let path = route_def.path;
        let method = route_def.method.as_str();
        let summary = docs.as_ref().and_then(|d| d.summary).unwrap_or("");
        let description = docs.as_ref().and_then(|d| d.description);
        let tags = docs.as_ref().map(|d| d.tags.clone()).unwrap_or_default();
        let response_config = docs.and_then(|d| d.response_config);

        let endpoint = Arc::new(endpoint);

        let route = match method {
            "get" => {
                let ep = Arc::clone(&endpoint);
                get_with(
                    move |axum::extract::Path(path_params): axum::extract::Path<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Query(query_params): axum::extract::Query<
                        std::collections::HashMap<String, String>,
                    >,
                          ExtractExtensions(ext): ExtractExtensions| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: E::Req::default(),
                                headers: Default::default(),
                                path: PathParams::new(path_params),
                                query: QueryParams::new(query_params),
                                extensions: ext,
                            };
                            ep.handler(ctx).await
                        }
                    },
                    |mut op| {
                        // Add query parameters
                        for param in &route_def.query_params {
                            let param_info = ParamInfo {
                                name: param.name,
                                description: param.description,
                                required: param.required,
                            };
                            op.inner_mut()
                                .parameters
                                .push(param_info_to_query_param(&param_info));
                        }

                        op = op.summary(summary);
                        if let Some(desc) = description {
                            op = op.description(desc);
                        }

                        for tag in &tags {
                            op = op.tag(tag);
                        }

                        // Apply response config callback if provided
                        if let Some(callback) = response_config {
                            op = callback(op);
                        }

                        op
                    },
                )
            }
            "post" => {
                let ep = Arc::clone(&endpoint);
                post_with(
                    move |axum::extract::Path(path_params): axum::extract::Path<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Query(query_params): axum::extract::Query<
                        std::collections::HashMap<String, String>,
                    >,
                          ExtractExtensions(ext): ExtractExtensions,
                          axum::Json(payload): axum::Json<E::Req>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: PathParams::new(path_params),
                                query: QueryParams::new(query_params),
                                extensions: ext,
                            };
                            ep.handler(ctx).await
                        }
                    },
                    |mut op| {
                        for param in &route_def.query_params {
                            let param_info = ParamInfo {
                                name: param.name,
                                description: param.description,
                                required: param.required,
                            };
                            op.inner_mut()
                                .parameters
                                .push(param_info_to_query_param(&param_info));
                        }

                        op = op.summary(summary);
                        if let Some(desc) = description {
                            op = op.description(desc);
                        }

                        for tag in &tags {
                            op = op.tag(tag);
                        }

                        // Apply response config callback if provided
                        if let Some(callback) = response_config {
                            op = callback(op);
                        }

                        op
                    },
                )
            }
            "put" => {
                let ep = Arc::clone(&endpoint);
                put_with(
                    move |axum::extract::Path(path_params): axum::extract::Path<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Query(query_params): axum::extract::Query<
                        std::collections::HashMap<String, String>,
                    >,
                          ExtractExtensions(ext): ExtractExtensions,
                          axum::Json(payload): axum::Json<E::Req>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: PathParams::new(path_params),
                                query: QueryParams::new(query_params),
                                extensions: ext,
                            };
                            ep.handler(ctx).await
                        }
                    },
                    |mut op| {
                        for param in &route_def.query_params {
                            let param_info = ParamInfo {
                                name: param.name,
                                description: param.description,
                                required: param.required,
                            };
                            op.inner_mut()
                                .parameters
                                .push(param_info_to_query_param(&param_info));
                        }

                        op = op.summary(summary);
                        if let Some(desc) = description {
                            op = op.description(desc);
                        }

                        for tag in &tags {
                            op = op.tag(tag);
                        }

                        // Apply response config callback if provided
                        if let Some(callback) = response_config {
                            op = callback(op);
                        }

                        op
                    },
                )
            }
            "delete" => {
                let ep = Arc::clone(&endpoint);
                delete_with(
                    move |axum::extract::Path(path_params): axum::extract::Path<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Query(query_params): axum::extract::Query<
                        std::collections::HashMap<String, String>,
                    >,
                          ExtractExtensions(ext): ExtractExtensions,
                          axum::Json(payload): axum::Json<E::Req>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: PathParams::new(path_params),
                                query: QueryParams::new(query_params),
                                extensions: ext,
                            };
                            ep.handler(ctx).await
                        }
                    },
                    |mut op| {
                        for param in &route_def.query_params {
                            let param_info = ParamInfo {
                                name: param.name,
                                description: param.description,
                                required: param.required,
                            };
                            op.inner_mut()
                                .parameters
                                .push(param_info_to_query_param(&param_info));
                        }

                        op = op.summary(summary);
                        if let Some(desc) = description {
                            op = op.description(desc);
                        }

                        for tag in &tags {
                            op = op.tag(tag);
                        }

                        // Apply response config callback if provided
                        if let Some(callback) = response_config {
                            op = callback(op);
                        }

                        op
                    },
                )
            }
            "patch" => {
                let ep = Arc::clone(&endpoint);
                patch_with(
                    move |axum::extract::Path(path_params): axum::extract::Path<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Query(query_params): axum::extract::Query<
                        std::collections::HashMap<String, String>,
                    >,
                          ExtractExtensions(ext): ExtractExtensions,
                          axum::Json(payload): axum::Json<E::Req>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: PathParams::new(path_params),
                                query: QueryParams::new(query_params),
                                extensions: ext,
                            };
                            ep.handler(ctx).await
                        }
                    },
                    |mut op| {
                        for param in &route_def.query_params {
                            let param_info = ParamInfo {
                                name: param.name,
                                description: param.description,
                                required: param.required,
                            };
                            op.inner_mut()
                                .parameters
                                .push(param_info_to_query_param(&param_info));
                        }

                        op = op.summary(summary);
                        if let Some(desc) = description {
                            op = op.description(desc);
                        }

                        for tag in &tags {
                            op = op.tag(tag);
                        }

                        // Apply response config callback if provided
                        if let Some(callback) = response_config {
                            op = callback(op);
                        }

                        op
                    },
                )
            }
            _ => {
                let ep = Arc::clone(&endpoint);
                get_with(
                    move |axum::extract::Path(path_params): axum::extract::Path<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Query(query_params): axum::extract::Query<
                        std::collections::HashMap<String, String>,
                    >,
                          ExtractExtensions(ext): ExtractExtensions| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: E::Req::default(),
                                headers: Default::default(),
                                path: PathParams::new(path_params),
                                query: QueryParams::new(query_params),
                                extensions: ext,
                            };
                            ep.handler(ctx).await
                        }
                    },
                    |mut op| {
                        for param in &route_def.query_params {
                            let param_info = ParamInfo {
                                name: param.name,
                                description: param.description,
                                required: param.required,
                            };
                            op.inner_mut()
                                .parameters
                                .push(param_info_to_query_param(&param_info));
                        }

                        op = op.summary(summary);
                        if let Some(desc) = description {
                            op = op.description(desc);
                        }

                        for tag in &tags {
                            op = op.tag(tag);
                        }

                        // Apply response config callback if provided
                        if let Some(callback) = response_config {
                            op = callback(op);
                        }

                        op
                    },
                )
            }
        };

        self.router = self.router.api_route(path, route);
        self
    }

    /// Merge another router into this server builder
    pub fn merge(mut self, router: ApiRouter) -> Self {
        self.router = self.router.merge(router);
        self
    }

    /// Nest another router under a path prefix
    pub fn nest(mut self, path: &str, router: ApiRouter) -> Self {
        self.router = self.router.nest(path, router);
        self
    }

    /// Set the bind address for the server
    pub fn bind(mut self, addr: impl Into<String>) -> Self {
        self.address = addr.into();
        self
    }

    /// Build the server with the configured options
    pub fn build(mut self) -> Server {
        // Initialize logging if config is available
        if let Some(ref config) = self.config {
            crate::logging::init(&config.logging);
        }

        // Get CORS config from AppConfig or use restrictive default
        let cors_config = self
            .config
            .as_ref()
            .map(|c| c.cors.clone())
            .unwrap_or_default();

        // Build CORS layer based on configuration
        let cors = self.build_cors_layer(&cors_config);

        // Build trace layer for request logging
        let trace_layer = if self
            .config
            .as_ref()
            .map(|c| c.logging.enabled && c.logging.log_requests)
            .unwrap_or(false)
        {
            Some(
                TraceLayer::new_for_http()
                    .make_span_with(RequestSpanMaker)
                    .on_response(RequestLogger),
            )
        } else {
            None
        };

        if let Some(api) = self.openapi {
            // Add documentation routes
            let docs_router = ApiRouter::new()
                .route("/openapi.json", get_with(serve_docs, |op| op))
                .route("/docs", get_with(serve_scalar_ui, |op| op));

            // Generate and set up the OpenAPI documentation
            let mut api = api.clone();
            let router = self.router.finish_api(&mut api);

            self.router = docs_router.merge(router).layer(Extension(api)).layer(cors);

            // Apply trace layer if enabled
            if let Some(trace) = trace_layer {
                self.router = self.router.layer(trace);
            }
            tracing::info!("Interactive Docs: http://{}/docs", self.address);
        } else {
            // Apply CORS even without OpenAPI
            self.router = self.router.layer(cors);

            // Apply trace layer if enabled
            if let Some(trace) = trace_layer {
                self.router = self.router.layer(trace);
            }
        }

        let address = self.address.parse().expect("Invalid bind address");
        Server {
            router: self.router,
            address,
        }
    }

    /// Build a CORS layer from configuration
    fn build_cors_layer(&self, config: &CorsConfig) -> CorsLayer {
        let mut cors = CorsLayer::new();

        // Configure origins
        if config.allows_all_origins() {
            cors = cors.allow_origin(Any);
        } else {
            let origins: Vec<HeaderValue> = config
                .allowed_origins
                .iter()
                .filter_map(|origin| origin.parse().ok())
                .collect();

            if !origins.is_empty() {
                cors = cors.allow_origin(AllowOrigin::list(origins));
            }
        }

        // Configure methods
        let methods: Vec<Method> = config
            .allowed_methods
            .iter()
            .filter_map(|method| method.parse().ok())
            .collect();

        if !methods.is_empty() {
            cors = cors.allow_methods(methods);
        }

        // Configure headers
        if config.allowed_headers.contains(&"*".to_string()) {
            cors = cors.allow_headers(Any);
        } else {
            let headers: Vec<axum::http::HeaderName> = config
                .allowed_headers
                .iter()
                .filter_map(|header| header.parse().ok())
                .collect();

            if !headers.is_empty() {
                cors = cors.allow_headers(headers);
            }
        }

        // Configure credentials
        if config.allow_credentials {
            cors = cors.allow_credentials(true);
        }

        // Configure max age
        if let Some(max_age) = config.max_age {
            cors = cors.max_age(Duration::from_secs(max_age));
        }

        cors
    }

    /// Build and start serving the application.
    ///
    /// This is a convenience method that combines `build()` and `serve()` into a single call.
    ///
    /// # Errors
    ///
    /// Returns an error if the server fails to bind to the configured address or
    /// encounters an error while serving requests.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use uncovr::prelude::*;
    /// use uncovr::server::Server;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let config = AppConfig::new("My API", "1.0.0");
    ///
    /// Server::new()
    ///     .with_config(config)
    ///     .serve()
    ///     .await
    ///     .expect("Server failed");
    /// # }
    /// ```
    pub async fn serve(self) -> Result<(), std::io::Error> {
        self.build().serve().await
    }

    /// Add a custom middleware layer to the server's router.
    ///
    /// This allows adding any Tower-compatible layer, such as logging, compression,
    /// rate limiting, authentication, etc.
    ///
    /// # Example
    /// ```
    /// use tower_http::compression::CompressionLayer;
    /// use uncovr::server::Server;
    ///
    /// let server = Server::new()
    ///     .with_config(AppConfig::new("My API", "1.0.0"))
    ///     .layer(CompressionLayer::new())
    ///     .build();
    /// ```
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: Service<Request<Body>, Response = AxumResponse, Error = Infallible>
            + Clone
            + Send
            + 'static,
        <L::Service as Service<Request<Body>>>::Future: Send + 'static,
    {
        self.router = self.router.layer(layer);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone)]
    struct TestEndpoint;

    #[derive(Default, Serialize, Deserialize, schemars::JsonSchema)]
    struct TestRequest {
        name: String,
    }

    impl Endpoint for TestEndpoint {
        fn ep(&self) -> Route {
            Route::GET("/tests")
        }

        fn docs(&self) -> Option<Docs> {
            Some(Docs::new().summary("Test endpoint"))
        }
    }

    #[async_trait::async_trait]
    impl API for TestEndpoint {
        type Req = TestRequest;
        type Res = String;

        async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
            format!("Hello, {}!", ctx.req.name)
        }
    }

    #[tokio::test]
    async fn test_server_builder() {
        let server = Server::new()
            .with_openapi(OpenApiConfig::new("Test API", "1.0.0"))
            .bind("127.0.0.1:3001")
            .register(TestEndpoint)
            .build();

        assert_eq!(server.address.to_string(), "127.0.0.1:3001");
    }
}
