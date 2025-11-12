use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

use aide::OperationIo;
use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, patch_with, post_with, put_with},
};
use aide::openapi::{
    ApiKeyLocation as OpenApiKeyLocation, Parameter, ParameterData, ParameterSchemaOrContent,
    QueryStyle, ReferenceOr, SchemaObject, SecurityRequirement,
    SecurityScheme as OpenApiSecurityScheme,
};
use async_trait::async_trait;
use axum::{Extension, body::Body};
use axum::{
    extract::FromRequestParts,
    http::{Request, Response, request::Parts},
};
use http::Extensions;
use tower::Service;
use tower_http::trace::{MakeSpan, OnResponse, TraceLayer};

use crate::api::api::Handler;
use crate::config::App;
use crate::context::Context;
use crate::openapi::{OpenApiConfig, serve_docs, serve_scalar_ui};
use crate::server::endpoint::{ApiKeyLocation, Endpoint as EndpointTrait, SecurityScheme};
use crate::server::params::{Path, Query};

/// Custom extractor for HTTP Extensions.
///
/// This internal extractor allows uncovr to extract the entire Extensions map from incoming requests
/// and pass it through to the [`Context`], enabling handlers to access application state and
/// request-scoped data via `ctx.state::<T>()`.
///
/// This is part of uncovr's abstraction layer that hides Axum implementation details from users.
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

/// Custom request span maker for structured logging.
///
/// Creates tracing spans for HTTP requests with method and path information.
/// This enables uncovr's logging system to provide clean, structured request logs.
#[derive(Clone)]
struct RequestSpanMaker;

impl<B> MakeSpan<B> for RequestSpanMaker {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        tracing::info_span!(
            "http",
            method = %request.method(),
            path = %request.uri().path(),
        )
    }
}

/// Custom response logger with minimal output.
///
/// Logs HTTP responses with appropriate severity based on status code:
/// - 5xx errors: ERROR level (server errors that need immediate attention)
/// - 4xx errors: WARN level (client errors worth monitoring)
/// - 2xx/3xx success: DEBUG level (reduces noise in production logs)
///
/// Latency is automatically formatted with appropriate units (ms/µs).
#[derive(Clone)]
struct RequestLogger;

impl<B> OnResponse<B> for RequestLogger {
    fn on_response(
        self,
        response: &Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        let status = response.status();

        // Format latency with appropriate unit
        let latency_str = if latency.as_millis() > 0 {
            format!("{}ms", latency.as_millis())
        } else {
            format!("{}µs", latency.as_micros())
        };

        if status.is_server_error() {
            tracing::error!(parent: span, status = status.as_u16(), latency = %latency_str);
        } else if status.is_client_error() {
            tracing::warn!(parent: span, status = status.as_u16(), latency = %latency_str);
        } else {
            // Log successful requests at DEBUG level to reduce verbosity
            tracing::debug!(parent: span, status = status.as_u16(), latency = %latency_str);
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
/// use uncovr::config::App;
///
/// #[tokio::main]
/// async fn main() {
///     let config = App::new("My API", "1.0.0");
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
/// impl Handler for HelloEndpoint {
///     type Request = ();
///     type Response = &'static str;
///
///     async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
///         "Hello, World!"
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let config = App::new("Hello API", "1.0.0")
///         .logging(Logging::development());
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
    config: Option<App>,
    logging: Option<crate::config::Logging>,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            router: ApiRouter::new(),
            address: "127.0.0.1:3000".to_string(),
            openapi: None,
            config: None,
            logging: None,
        }
    }
}

/// Parameter information for OpenAPI documentation (internal helper).
///
/// This internal structure represents metadata about route parameters (query and path)
/// that will be converted to OpenAPI parameter specifications. It's part of uncovr's
/// automatic OpenAPI documentation generation.
#[derive(Debug, Clone)]
struct ParamInfo {
    /// Parameter name
    name: &'static str,
    /// Parameter description
    description: Option<&'static str>,
    /// Whether the parameter is required
    required: bool,
}

/// Helper function to convert ParamInfo to aide's Parameter type for OpenAPI.
///
/// Transforms uncovr's parameter representation into OpenAPI 3.0 parameter specifications
/// for automatic documentation generation. This keeps the high-level uncovr API simple
/// while ensuring proper OpenAPI schema generation.
fn param_info_to_query_param(param: &ParamInfo) -> ReferenceOr<Parameter> {
    ReferenceOr::Item(Parameter::Query {
        parameter_data: ParameterData {
            name: param.name.to_string(),
            description: param.description.map(|s| s.to_string()),
            required: param.required,
            deprecated: None,
            format: ParameterSchemaOrContent::Schema(SchemaObject {
                json_schema: serde_json::from_value(serde_json::json!({ "type": "string" }))
                    .unwrap(),
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

/// Convert uncovr SecurityScheme to OpenAPI security requirement name.
///
/// Maps security schemes to their standard OpenAPI security scheme names.
/// These names are used to reference security schemes defined in the OpenAPI spec.
fn security_scheme_name(scheme: &SecurityScheme) -> &'static str {
    match scheme {
        SecurityScheme::Bearer => "bearerAuth",
        SecurityScheme::Basic => "basicAuth",
        SecurityScheme::ApiKey { .. } => "apiKeyAuth",
        SecurityScheme::OAuth2 { .. } => "oauth2Auth",
    }
}

/// Convert uncovr SecurityScheme to OpenAPI SecurityScheme.
///
/// Transforms uncovr's security scheme definitions into OpenAPI 3.0 security scheme
/// specifications for automatic documentation generation.
fn to_openapi_security_scheme(scheme: &SecurityScheme) -> OpenApiSecurityScheme {
    match scheme {
        SecurityScheme::Bearer => OpenApiSecurityScheme::Http {
            scheme: "bearer".to_string(),
            bearer_format: Some("JWT".to_string()),
            description: Some("Bearer token authentication".to_string()),
            extensions: Default::default(),
        },
        SecurityScheme::Basic => OpenApiSecurityScheme::Http {
            scheme: "basic".to_string(),
            bearer_format: None,
            description: Some("HTTP Basic authentication".to_string()),
            extensions: Default::default(),
        },
        SecurityScheme::ApiKey { name, location } => {
            let api_location = match location {
                ApiKeyLocation::Header => OpenApiKeyLocation::Header,
                ApiKeyLocation::Query => OpenApiKeyLocation::Query,
                ApiKeyLocation::Cookie => OpenApiKeyLocation::Cookie,
            };
            OpenApiSecurityScheme::ApiKey {
                location: api_location,
                name: name.to_string(),
                description: Some(format!(
                    "API key in {}",
                    match location {
                        ApiKeyLocation::Header => "header",
                        ApiKeyLocation::Query => "query parameter",
                        ApiKeyLocation::Cookie => "cookie",
                    }
                )),
                extensions: Default::default(),
            }
        }
        SecurityScheme::OAuth2 { .. } => {
            // For OAuth2, we create a basic placeholder
            // Users can customize this via response_config if needed
            OpenApiSecurityScheme::OAuth2 {
                flows: Default::default(),
                description: Some("OAuth2 authentication".to_string()),
                extensions: Default::default(),
            }
        }
    }
}

impl ServerBuilder {
    /// Configure the server with an App
    pub fn with_config(mut self, config: App) -> Self {
        // Set address from config
        self.address = config.bind.clone();

        // Configure OpenAPI if enabled
        if config.docs {
            let mut openapi_config =
                OpenApiConfig::new(&config.name, &config.version).description(&config.description);

            // Add servers from config, or use bind address if no servers configured
            if config.servers.is_empty() {
                // Automatically derive server URL from bind address
                let server_url = if config.bind.starts_with("0.0.0.0:") {
                    format!(
                        "http://localhost:{}",
                        config.bind.strip_prefix("0.0.0.0:").unwrap()
                    )
                } else if config.bind.starts_with("127.0.0.1:")
                    || config.bind.starts_with("localhost:")
                {
                    format!("http://{}", config.bind)
                } else {
                    // For any other address (including domain names), use http://
                    format!("http://{}", config.bind)
                };

                openapi_config = openapi_config.server(server_url, "API Server");
            } else {
                // Use explicitly configured servers
                for server in &config.servers {
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

    /// Configure logging for the server
    ///
    /// This is now separate from App for better modularity.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use uncovr::server::Server;
    /// use uncovr::config::{App, Logging};
    ///
    /// let config = App::new("My API", "1.0.0");
    ///
    /// Server::new()
    ///     .with_config(config)
    ///     .with_logging(Logging::development())
    ///     .serve()
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn with_logging(mut self, logging: crate::config::Logging) -> Self {
        self.logging = Some(logging);
        self
    }

    /// Set application state that will be accessible in all handlers via `ctx.state()`.
    ///
    /// The state is stored in the request extensions and can be retrieved in handlers
    /// using `ctx.state::<T>()`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::server::Server;
    ///
    /// #[derive(Clone)]
    /// struct AppState {
    ///     db: sqlx::PgPool,
    /// }
    ///
    /// let state = AppState {
    ///     db: create_pool().await,
    /// };
    ///
    /// Server::new()
    ///     .with_state(state)
    ///     .register(MyEndpoint)
    ///     .serve()
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn with_state<S: Clone + Send + Sync + 'static>(mut self, state: S) -> Self {
        self.router = self.router.layer(Extension(state));
        self
    }

    /// Register an API endpoint with automatic routing and OpenAPI documentation.
    ///
    /// This is the core method for adding endpoints to your uncovr application. It handles:
    /// - **Route registration**: Extracts path, method, and parameters from `Endpoint::route()`
    /// - **Request handling**: Wires up the `Handler::handle()` method with proper context
    /// - **OpenAPI documentation**: Auto-generates schema from `Endpoint::meta()` and type information
    /// - **Type safety**: Ensures request/response types implement necessary traits at compile time
    ///
    /// # Type Parameters
    ///
    /// * `E` - The endpoint type that implements both ['Endpoint'] and ['Handler']
    ///
    /// # Generic Bounds
    ///
    /// The endpoint must satisfy:
    /// - `E::Request`: Deserializable from JSON and has a default (for GET requests without body)
    /// - `E::Response`: Convertible to HTTP response and has OpenAPI schema information
    /// - All types are `Send + 'static` for async runtime compatibility
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::prelude::*;
    /// use uncovr::server::Server;
    ///
    /// #[derive(Clone)]
    /// struct CreateUser;
    ///
    /// impl Endpoint for CreateUser {
    ///     fn route(&self) -> Route {
    ///         Route::post("/users")
    ///     }
    ///
    ///     fn meta(&self) -> Meta {
    ///         Meta::new()
    ///             .summary("Create a user")
    ///             .tag("users")
    ///     }
    /// }
    ///
    /// #[async_trait]
    /// impl Handler for CreateUser {
    ///     type Request = CreateUserRequest;
    ///     type Response = Json<User>;
    ///
    ///     async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
    ///         // Implementation
    ///         # Json(User { id: 1, name: "test".into() })
    ///     }
    /// }
    /// # #[derive(serde::Deserialize, schemars::JsonSchema, Default)]
    /// # struct CreateUserRequest { name: String }
    /// # #[derive(serde::Serialize, schemars::JsonSchema)]
    /// # struct User { id: i64, name: String }
    ///
    /// Server::new()
    ///     .register(CreateUser)
    ///     .serve()
    ///     .await
    ///     .unwrap();
    /// ```
    pub fn register<E>(mut self, endpoint: E) -> Self
    where
        E: EndpointTrait + Handler + Send + Sync + 'static,
        E::Request: serde::de::DeserializeOwned + schemars::JsonSchema + Default + Send + 'static,
        E::Response: aide::OperationOutput + axum::response::IntoResponse + Send + 'static,
        <E::Response as aide::OperationOutput>::Inner: schemars::JsonSchema,
    {
        let route_def = endpoint.route();
        let meta = endpoint.meta();

        let path = route_def.path;
        let method = route_def.method.as_str();
        let summary = meta.summary.unwrap_or("");
        let description = meta.description;
        let tags = meta.tags.clone();
        let response_config = meta.response_config;
        let security_schemes = meta.security.clone();

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
                                req: E::Request::default(),
                                headers: Default::default(),
                                path: Path::new(path_params),
                                query: Query::new(query_params),
                                extensions: ext,
                            };
                            ep.handle(ctx).await
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

                        // Add security requirements
                        if !security_schemes.is_empty() {
                            for scheme in &security_schemes {
                                let mut security_req = SecurityRequirement::new();
                                security_req
                                    .insert(security_scheme_name(scheme).to_string(), vec![]);
                                op.inner_mut().security.push(security_req);
                            }
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
                          axum::Json(payload): axum::Json<E::Request>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: Path::new(path_params),
                                query: Query::new(query_params),
                                extensions: ext,
                            };
                            ep.handle(ctx).await
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

                        // Add security requirements
                        if !security_schemes.is_empty() {
                            for scheme in &security_schemes {
                                let mut security_req = SecurityRequirement::new();
                                security_req
                                    .insert(security_scheme_name(scheme).to_string(), vec![]);
                                op.inner_mut().security.push(security_req);
                            }
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
                          axum::Json(payload): axum::Json<E::Request>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: Path::new(path_params),
                                query: Query::new(query_params),
                                extensions: ext,
                            };
                            ep.handle(ctx).await
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

                        // Add security requirements
                        if !security_schemes.is_empty() {
                            for scheme in &security_schemes {
                                let mut security_req = SecurityRequirement::new();
                                security_req
                                    .insert(security_scheme_name(scheme).to_string(), vec![]);
                                op.inner_mut().security.push(security_req);
                            }
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
                          axum::Json(payload): axum::Json<E::Request>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: Path::new(path_params),
                                query: Query::new(query_params),
                                extensions: ext,
                            };
                            ep.handle(ctx).await
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

                        // Add security requirements
                        if !security_schemes.is_empty() {
                            for scheme in &security_schemes {
                                let mut security_req = SecurityRequirement::new();
                                security_req
                                    .insert(security_scheme_name(scheme).to_string(), vec![]);
                                op.inner_mut().security.push(security_req);
                            }
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
                          axum::Json(payload): axum::Json<E::Request>| {
                        let ep = Arc::clone(&ep);
                        async move {
                            let ctx = Context {
                                req: payload,
                                headers: Default::default(),
                                path: Path::new(path_params),
                                query: Query::new(query_params),
                                extensions: ext,
                            };
                            ep.handle(ctx).await
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

                        // Add security requirements
                        if !security_schemes.is_empty() {
                            for scheme in &security_schemes {
                                let mut security_req = SecurityRequirement::new();
                                security_req
                                    .insert(security_scheme_name(scheme).to_string(), vec![]);
                                op.inner_mut().security.push(security_req);
                            }
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
                                req: E::Request::default(),
                                headers: Default::default(),
                                path: Path::new(path_params),
                                query: Query::new(query_params),
                                extensions: ext,
                            };
                            ep.handle(ctx).await
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

                        // Add security requirements
                        if !security_schemes.is_empty() {
                            for scheme in &security_schemes {
                                let mut security_req = SecurityRequirement::new();
                                security_req
                                    .insert(security_scheme_name(scheme).to_string(), vec![]);
                                op.inner_mut().security.push(security_req);
                            }
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

    /// Nest a service under a path prefix
    ///
    /// This allows you to nest external services like static file servers,
    /// custom tower services, etc.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::server::Server;
    /// use tower_http::services::ServeDir;
    ///
    /// let server = Server::new()
    ///     .nest_service("/static", ServeDir::new("public"))
    ///     .build();
    /// ```
    pub fn nest_service<S>(mut self, path: &str, service: S) -> Self
    where
        S: tower::Service<
                axum::http::Request<axum::body::Body>,
                Response = axum::http::Response<axum::body::Body>,
                Error = std::convert::Infallible,
            > + Clone
            + Send
            + 'static,
        S::Future: Send + 'static,
    {
        self.router = self.router.nest_service(path, service);
        self
    }

    /// Set the bind address for the server
    pub fn bind(mut self, addr: impl Into<String>) -> Self {
        self.address = addr.into();
        self
    }

    /// Build the server with the configured options
    pub fn build(mut self) -> Server {
        // Initialize logging if configured
        if let Some(ref logging) = self.logging {
            crate::logging::init(logging);
        }

        // Build trace layer for request logging if enabled
        let trace_layer = if self
            .logging
            .as_ref()
            .map(|c| c.log_requests)
            .unwrap_or(false)
        {
            use tower_http::trace::DefaultOnRequest;
            Some(
                TraceLayer::new_for_http()
                    .make_span_with(RequestSpanMaker)
                    .on_request(DefaultOnRequest::new().level(tracing::Level::TRACE))
                    .on_response(RequestLogger),
            )
        } else {
            None
        };

        if let Some(api) = self.openapi {
            // Get custom docs paths from config or use defaults
            let docs_path = self
                .config
                .as_ref()
                .map(|c| c.docs_path.as_str())
                .unwrap_or("/docs");
            let openapi_json_path = self
                .config
                .as_ref()
                .map(|c| c.spec_path.as_str())
                .unwrap_or("/openapi.json");

            let openapi_path_for_ui = if let Some(stripped) = openapi_json_path.strip_prefix('/') {
                format!("./{}", stripped)
            } else {
                format!("./{}", openapi_json_path)
            };

            let ui_handler = move || {
                let path = openapi_path_for_ui.clone();
                async move { serve_scalar_ui(path).await }
            };

            // Add documentation routes
            let docs_router = ApiRouter::new()
                .route(openapi_json_path, get_with(serve_docs, |op| op))
                .route(docs_path, get_with(ui_handler, |op| op));

            // Generate and set up the OpenAPI documentation
            let mut api = api.clone();

            // Add global security scheme definitions
            // These define the authentication methods available in the API
            api.components
                .get_or_insert_with(Default::default)
                .security_schemes
                .insert(
                    "bearerAuth".to_string(),
                    ReferenceOr::Item(to_openapi_security_scheme(&SecurityScheme::Bearer)),
                );

            api.components
                .get_or_insert_with(Default::default)
                .security_schemes
                .insert(
                    "basicAuth".to_string(),
                    ReferenceOr::Item(to_openapi_security_scheme(&SecurityScheme::Basic)),
                );

            api.components
                .get_or_insert_with(Default::default)
                .security_schemes
                .insert(
                    "apiKeyAuth".to_string(),
                    ReferenceOr::Item(to_openapi_security_scheme(&SecurityScheme::ApiKey {
                        name: "X-API-Key",
                        location: ApiKeyLocation::Header,
                    })),
                );

            let router = self.router.finish_api(&mut api);

            self.router = docs_router.merge(router).layer(Extension(api));

            // Apply trace layer if enabled
            if let Some(trace) = trace_layer {
                self.router = self.router.layer(trace);
            }
        } else {
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
    /// let config = App::new("My API", "1.0.0");
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
    ///     .with_config(App::new("My API", "1.0.0"))
    ///     .layer(CompressionLayer::new())
    ///     .build();
    /// ```
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: Service<Request<Body>> + Clone + Send + 'static,
        <L::Service as Service<Request<Body>>>::Response: axum::response::IntoResponse,
        <L::Service as Service<Request<Body>>>::Error: Into<Infallible> + std::error::Error,
        <L::Service as Service<Request<Body>>>::Future: Send + 'static,
    {
        self.router = self.router.layer(layer);
        self
    }

    /// Add a fallback handler for unmatched routes.
    ///
    /// This handler will be called when no route matches the incoming request.
    /// It's useful for providing custom 404 pages or API error responses.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::server::Server;
    /// use uncovr::prelude::*;
    ///
    /// async fn handle_404() -> (StatusCode, &'static str) {
    ///     (StatusCode::NOT_FOUND, "Route not found")
    /// }
    ///
    /// let server = Server::new()
    ///     .with_config(App::new("My API", "1.0.0"))
    ///     .fallback(handle_404)
    ///     .build();
    /// ```
    pub fn fallback<H, T>(mut self, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.fallback(handler);
        self
    }

    /// Add a fallback service for unmatched routes.
    ///
    /// This is similar to `fallback`, but accepts a Tower service instead of a handler.
    /// This is useful for integrating external services or more complex fallback logic.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::server::Server;
    /// use uncovr::prelude::*;
    /// use tower::service_fn;
    /// use axum::body::Body;
    /// use axum::http::{Request, Response};
    ///
    /// async fn fallback_service(req: Request<Body>) -> Result<Response<Body>, std::convert::Infallible> {
    ///     Ok(Response::builder()
    ///         .status(404)
    ///         .body(Body::from("Custom 404"))
    ///         .unwrap())
    /// }
    ///
    /// let server = Server::new()
    ///     .with_config(App::new("My API", "1.0.0"))
    ///     .fallback_service(service_fn(fallback_service))
    ///     .build();
    /// ```
    pub fn fallback_service<S>(mut self, service: S) -> Self
    where
        S: tower::Service<
                axum::http::Request<axum::body::Body>,
                Response = axum::http::Response<axum::body::Body>,
                Error = std::convert::Infallible,
            > + Clone
            + Send
            + 'static,
        S::Future: Send + 'static,
    {
        self.router = self.router.fallback_service(service);
        self
    }

    /// Add middleware using a function-based API
    ///
    /// Convenience wrapper around `.layer(from_fn(...))` for easier middleware composition.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::server::Server;
    /// use uncovr::prelude::*;
    /// use axum::middleware::Next;
    ///
    /// async fn my_middleware(req: Request<Body>, next: Next) -> Response {
    ///     // Middleware logic
    ///     next.run(req).await
    /// }
    ///
    /// let server = Server::new()
    ///     .middleware(my_middleware)
    ///     .build();
    /// ```
    pub fn middleware<F>(self, middleware: F) -> Self
    where
        F: Fn(
                Request<Body>,
                axum::middleware::Next,
            ) -> futures::future::BoxFuture<'static, Response<Body>>
            + Clone
            + Send
            + Sync
            + 'static,
    {
        self.layer(axum::middleware::from_fn(middleware))
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
        fn route(&self) -> Route {
            Route::get("/tests")
        }

        fn meta(&self) -> Meta {
            Meta::new().summary("Test endpoint")
        }
    }

    #[async_trait::async_trait]
    impl Handler for TestEndpoint {
        type Request = TestRequest;
        type Response = String;

        async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
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

    #[tokio::test]
    async fn test_nested_routes() {
        // Create v1 routes
        let v1_routes = Server::new().register(TestEndpoint).build().into_router();

        // Create v2 routes
        #[derive(Clone)]
        struct V2TestEndpoint;

        #[derive(Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        struct V2Response {
            version: String,
        }

        impl Endpoint for V2TestEndpoint {
            fn route(&self) -> Route {
                Route::get("/test")
            }

            fn meta(&self) -> Meta {
                Meta::new().summary("V2 test endpoint")
            }
        }

        #[async_trait::async_trait]
        impl Handler for V2TestEndpoint {
            type Request = TestRequest;
            type Response = Json<V2Response>;

            async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
                Json(V2Response {
                    version: "v2".to_string(),
                })
            }
        }

        let v2_routes = Server::new().register(V2TestEndpoint).build().into_router();

        // Nest both under versioned paths
        let server = Server::new()
            .with_openapi(OpenApiConfig::new("Nested API", "1.0.0"))
            .bind("127.0.0.1:3002")
            .nest("/v1", v1_routes)
            .nest("/v2", v2_routes)
            .build();

        assert_eq!(server.address.to_string(), "127.0.0.1:3002");
    }

    #[tokio::test]
    async fn test_feature_based_nesting() {
        // Create user routes
        #[derive(Clone)]
        struct GetUser;

        impl Endpoint for GetUser {
            fn route(&self) -> Route {
                let mut route = Route::get("/:id");
                route.path_param("id").desc("User ID");
                route
            }

            fn meta(&self) -> Meta {
                Meta::new().summary("Get user by ID").tag("users")
            }
        }

        #[async_trait::async_trait]
        impl Handler for GetUser {
            type Request = ();
            type Response = String;

            async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
                format!("User {}", ctx.path.get("id").unwrap_or("unknown"))
            }
        }

        let user_routes = Server::new().register(GetUser).build().into_router();

        // Create post routes
        #[derive(Clone)]
        struct GetPost;

        impl Endpoint for GetPost {
            fn route(&self) -> Route {
                let mut route = Route::get("/:id");
                route.path_param("id").desc("Post ID");
                route
            }

            fn meta(&self) -> Meta {
                Meta::new().summary("Get post by ID").tag("posts")
            }
        }

        #[async_trait::async_trait]
        impl Handler for GetPost {
            type Request = ();
            type Response = String;

            async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
                format!("Post {}", ctx.path.get("id").unwrap_or("unknown"))
            }
        }

        let post_routes = Server::new().register(GetPost).build().into_router();

        // Nest under feature paths
        let server = Server::new()
            .with_openapi(OpenApiConfig::new("Feature API", "1.0.0"))
            .bind("127.0.0.1:3003")
            .nest("/users", user_routes)
            .nest("/posts", post_routes)
            .build();

        assert_eq!(server.address.to_string(), "127.0.0.1:3003");
    }

    #[tokio::test]
    async fn test_nest_service_external() {
        // Create a simple service that responds with static text
        let external_service = axum::routing::get(|| async { "External service response" });

        // Test that we can nest external services using nest_service
        let server = Server::new()
            .with_openapi(OpenApiConfig::new("Service Nesting Test", "1.0.0"))
            .bind("127.0.0.1:3004")
            .register(TestEndpoint)
            .nest_service("/external", external_service)
            .build();

        assert_eq!(server.address.to_string(), "127.0.0.1:3004");
    }
}
