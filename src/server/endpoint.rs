//! Endpoint routing and metadata definitions for uncovr applications.
//!
//! This module provides the [`Endpoint`] trait and supporting types for defining
//! API routes with automatic OpenAPI documentation generation.

/// HTTP method types for REST API endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl HttpMethod {
    /// Returns the lowercase string representation for routing.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GET => "get",
            Self::POST => "post",
            Self::PUT => "put",
            Self::PATCH => "patch",
            Self::DELETE => "delete",
            Self::OPTIONS => "options",
            Self::HEAD => "head",
        }
    }
}

/// Query parameter metadata for OpenAPI documentation.
#[derive(Debug, Clone)]
pub struct QueryParam {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
}

/// Path parameter metadata for OpenAPI documentation.
#[derive(Debug, Clone)]
pub struct PathParam {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
}

/// Builder for configuring route parameters with a fluent API.
///
/// Created by [`Route::query()`] and [`Route::path_param()`] methods. Allows chaining
/// `.required()` and `.desc()` calls to configure parameter metadata for OpenAPI documentation.
#[derive(Debug)]
pub struct ParamBuilder<'a> {
    route: &'a mut Route,
    param_type: ParamType,
}

#[derive(Debug)]
enum ParamType {
    Query(usize),
    Path(usize),
}

impl<'a> ParamBuilder<'a> {
    /// Marks the parameter as required in the OpenAPI specification.
    pub fn required(self) -> &'a mut Route {
        match self.param_type {
            ParamType::Query(idx) => {
                if let Some(param) = self.route.query_params.get_mut(idx) {
                    param.required = true;
                }
            }
            ParamType::Path(idx) => {
                if let Some(param) = self.route.path_params.get_mut(idx) {
                    param.required = true;
                }
            }
        }
        self.route
    }

    /// Adds a description to the parameter for OpenAPI documentation.
    pub fn desc(self, description: &'static str) -> &'a mut Route {
        match self.param_type {
            ParamType::Query(idx) => {
                if let Some(param) = self.route.query_params.get_mut(idx) {
                    param.description = Some(description);
                }
            }
            ParamType::Path(idx) => {
                if let Some(param) = self.route.path_params.get_mut(idx) {
                    param.description = Some(description);
                }
            }
        }
        self.route
    }
}

/// Route definition for an API endpoint.
///
/// Specifies the HTTP method, path, and parameter metadata for an endpoint.
/// Used with [`Endpoint::route()`] to configure routing behavior.
#[derive(Debug, Clone)]
pub struct Route {
    pub path: &'static str,
    pub method: HttpMethod,
    pub query_params: Vec<QueryParam>,
    pub path_params: Vec<PathParam>,
}

impl Route {
    /// Creates a route with the specified HTTP method and path.
    pub fn new(method: HttpMethod, path: &'static str) -> Self {
        Self {
            path,
            method,
            query_params: Vec::new(),
            path_params: Vec::new(),
        }
    }

    /// Creates a GET route.
    pub fn get(path: &'static str) -> Self {
        Self::new(HttpMethod::GET, path)
    }

    /// Creates a POST route.
    pub fn post(path: &'static str) -> Self {
        Self::new(HttpMethod::POST, path)
    }

    /// Creates a PUT route.
    pub fn put(path: &'static str) -> Self {
        Self::new(HttpMethod::PUT, path)
    }

    /// Creates a PATCH route.
    pub fn patch(path: &'static str) -> Self {
        Self::new(HttpMethod::PATCH, path)
    }

    /// Creates a DELETE route.
    pub fn delete(path: &'static str) -> Self {
        Self::new(HttpMethod::DELETE, path)
    }

    /// Creates an OPTIONS route.
    pub fn options(path: &'static str) -> Self {
        Self::new(HttpMethod::OPTIONS, path)
    }

    /// Creates a HEAD route.
    pub fn head(path: &'static str) -> Self {
        Self::new(HttpMethod::HEAD, path)
    }

    /// Adds a query parameter and returns a builder for configuration.
    pub fn query(&mut self, name: &'static str) -> ParamBuilder<'_> {
        self.query_params.push(QueryParam {
            name,
            description: None,
            required: false,
        });

        let idx = self.query_params.len() - 1;
        ParamBuilder {
            route: self,
            param_type: ParamType::Query(idx),
        }
    }

    /// Adds a path parameter with description.
    pub fn param(mut self, name: &'static str, description: &'static str) -> Self {
        self.path_params.push(PathParam {
            name,
            description: Some(description),
            required: true,
        });
        self
    }

    /// Adds a path parameter and returns a builder for configuration.
    pub fn path_param(&mut self, name: &'static str) -> ParamBuilder<'_> {
        self.path_params.push(PathParam {
            name,
            description: None,
            required: false,
        });

        let idx = self.path_params.len() - 1;
        ParamBuilder {
            route: self,
            param_type: ParamType::Path(idx),
        }
    }
}

/// Callback type for configuring OpenAPI response schemas.
pub type ResponseCallback = Box<
    dyn FnOnce(aide::transform::TransformOperation) -> aide::transform::TransformOperation + Send,
>;

/// Endpoint metadata for API documentation and OpenAPI specification generation.
///
/// Defines human-readable information about endpoints including summaries, descriptions,
/// tags, and deprecation status.
#[derive(Default)]
pub struct Meta {
    pub summary: Option<&'static str>,
    pub description: Option<&'static str>,
    pub tags: Vec<&'static str>,
    pub deprecated: bool,
    pub response_config: Option<ResponseCallback>,
}

impl Meta {
    /// Creates metadata builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the endpoint summary (brief description).
    pub fn summary(mut self, text: &'static str) -> Self {
        self.summary = Some(text);
        self
    }

    /// Sets the detailed description for the endpoint.
    pub fn describe(mut self, text: &'static str) -> Self {
        self.description = Some(text);
        self
    }

    /// Sets the detailed description (alias for `describe`).
    pub fn description(self, text: &'static str) -> Self {
        self.describe(text)
    }

    /// Adds a tag for categorizing the endpoint in documentation.
    pub fn tag(mut self, tag: &'static str) -> Self {
        self.tags.push(tag);
        self
    }

    /// Marks the endpoint as deprecated in the OpenAPI specification.
    pub fn deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }

    /// Configures OpenAPI response schemas for this endpoint.
    pub fn responses<F>(mut self, callback: F) -> Self
    where
        F: FnOnce(aide::transform::TransformOperation) -> aide::transform::TransformOperation
            + Send
            + 'static,
    {
        self.response_config = Some(Box::new(callback));
        self
    }
}

/// Trait for defining API endpoint routing and documentation.
///
/// Separates technical route configuration from human-readable metadata,
/// enabling clean endpoint definitions with automatic OpenAPI generation.
///
/// # Example
///
/// ```no_run
/// use uncovr::server::endpoint::{Endpoint, Route, Meta};
///
/// struct GetUsers;
///
/// impl Endpoint for GetUsers {
///     fn route(&self) -> Route {
///         Route::get("/users")
///     }
///
///     fn meta(&self) -> Meta {
///         Meta::new()
///             .summary("List all users")
///             .tag("users")
///     }
/// }
/// ```
pub trait Endpoint {
    /// Returns the route configuration for this endpoint.
    fn route(&self) -> Route;

    /// Returns the metadata for OpenAPI documentation.
    fn meta(&self) -> Meta {
        Meta::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_as_str() {
        assert_eq!(HttpMethod::GET.as_str(), "get");
        assert_eq!(HttpMethod::POST.as_str(), "post");
        assert_eq!(HttpMethod::PUT.as_str(), "put");
        assert_eq!(HttpMethod::DELETE.as_str(), "delete");
    }

    #[test]
    fn test_route_builders() {
        let route = Route::get("/users");
        assert_eq!(route.path, "/users");
        assert_eq!(route.method, HttpMethod::GET);

        let route = Route::post("/users");
        assert_eq!(route.method, HttpMethod::POST);
    }

    #[test]
    fn test_route_with_params() {
        let mut route = Route::get("/users");
        route.query("page").required();
        route.query("limit");

        assert_eq!(route.query_params.len(), 2);
        assert_eq!(route.query_params[0].name, "page");
        assert!(route.query_params[0].required);
        assert_eq!(route.query_params[1].name, "limit");
        assert!(!route.query_params[1].required);
    }

    #[test]
    fn test_meta_builder() {
        let meta = Meta::new()
            .summary("Test endpoint")
            .describe("This is a test")
            .tag("test")
            .tag("example")
            .deprecated();

        assert_eq!(meta.summary, Some("Test endpoint"));
        assert_eq!(meta.description, Some("This is a test"));
        assert_eq!(meta.tags.len(), 2);
        assert!(meta.deprecated);
    }

    struct TestEndpoint;

    impl Endpoint for TestEndpoint {
        fn route(&self) -> Route {
            Route::get("/test")
        }
    }

    #[test]
    fn test_endpoint_trait() {
        let endpoint = TestEndpoint;
        let route = endpoint.route();

        assert_eq!(route.path, "/test");
        assert_eq!(route.method, HttpMethod::GET);
    }
}
