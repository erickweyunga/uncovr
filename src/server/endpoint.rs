//! Endpoint definition module for Uncovr framework.
//!
//! This module provides a clean separation between route definition and documentation.
//!
//! # Example
//!
//! ```no_run
//! use uncovr::server::endpoint::{Endpoint, Route, Docs};
//!
//! struct CreateUser;
//!
//! impl Endpoint for CreateUser {
//!     fn ep(&self) -> Route {
//!         Route::POST("/users")
//!             .query("notify")
//!     }
//!
//!     fn docs(&self) -> Option<Docs> {
//!         Some(Docs::new()
//!             .summary("Create a new user")
//!             .description("Creates a user with the provided information")
//!             .tag("users"))
//!     }
//! }
//! ```

/// HTTP method enumeration.
///
/// Represents the standard HTTP methods used in REST APIs.
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
    /// Convert to lowercase string representation for routing
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

/// Query parameter information.
#[derive(Debug, Clone)]
pub struct QueryParam {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
}

/// Path parameter information.
#[derive(Debug, Clone)]
pub struct PathParam {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
}

/// Parameter builder for fluent API.
///
/// This is used internally to build query and path parameters with a fluent interface.
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
    /// Mark the parameter as required.
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

    /// Add a description to the parameter.
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
/// Defines the technical aspects of a route including path, HTTP method,
/// and parameters.
///
/// # Example
///
/// ```
/// use uncovr::server::endpoint::Route;
///
/// let route = Route::POST("/users/:id")
///     .path_param("id").required().desc("User ID")
///     .query("notify").desc("Send notification");
/// ```
#[derive(Debug, Clone)]
pub struct Route {
    pub path: &'static str,
    pub method: HttpMethod,
    pub query_params: Vec<QueryParam>,
    pub path_params: Vec<PathParam>,
}

impl Route {
    /// Create a new route with the specified method and path.
    pub fn new(method: HttpMethod, path: &'static str) -> Self {
        Self {
            path,
            method,
            query_params: Vec::new(),
            path_params: Vec::new(),
        }
    }

    /// Create a GET route.
    #[allow(non_snake_case)]
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::GET("/users");
    /// ```
    pub fn GET(path: &'static str) -> Self {
        Self::new(HttpMethod::GET, path)
    }

    /// Create a POST route.
    #[allow(non_snake_case)]
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::POST("/users");
    /// ```
    pub fn POST(path: &'static str) -> Self {
        Self::new(HttpMethod::POST, path)
    }

    /// Create a PUT route.
    #[allow(non_snake_case)]
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::PUT("/users/:id");
    /// ```
    pub fn PUT(path: &'static str) -> Self {
        Self::new(HttpMethod::PUT, path)
    }

    /// Create a PATCH route.
    #[allow(non_snake_case)]
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::PATCH("/users/:id");
    /// ```
    pub fn PATCH(path: &'static str) -> Self {
        Self::new(HttpMethod::PATCH, path)
    }

    /// Create a DELETE route.
    #[allow(non_snake_case)]
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::DELETE("/users/:id");
    /// ```
    pub fn DELETE(path: &'static str) -> Self {
        Self::new(HttpMethod::DELETE, path)
    }

    /// Create an OPTIONS route.
    #[allow(non_snake_case)]
    pub fn OPTIONS(path: &'static str) -> Self {
        Self::new(HttpMethod::OPTIONS, path)
    }

    /// Create a HEAD route.
    #[allow(non_snake_case)]
    pub fn HEAD(path: &'static str) -> Self {
        Self::new(HttpMethod::HEAD, path)
    }

    /// Add a query parameter.
    ///
    /// Returns a mutable reference to self for further chaining,
    /// or use `.required()` or `.desc()` to configure the parameter.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::GET("/users")
    ///     .query("page")
    ///     .query("limit").required();
    /// ```
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

    /// Add a path parameter.
    ///
    /// Returns a mutable reference to self for further chaining,
    /// or use `.required()` or `.desc()` to configure the parameter.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::DELETE("/users/:id")
    ///     .path_param("id").required().desc("User ID");
    /// ```
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

/// Response callback type for configuring OpenAPI responses
pub type ResponseCallback = Box<
    dyn FnOnce(aide::transform::TransformOperation) -> aide::transform::TransformOperation + Send,
>;

/// Documentation for an API endpoint.
///
/// Provides human-readable information about the endpoint for API documentation
/// and OpenAPI specification generation.
///
/// # Example
///
/// ```
/// use uncovr::server::endpoint::Docs;
///
/// let docs = Docs::new()
///     .summary("Create a new user")
///     .description("Creates a user with the provided information")
///     .tag("users")
///     .tag("authentication");
/// ```
#[derive(Default)]
pub struct Docs {
    pub summary: Option<&'static str>,
    pub description: Option<&'static str>,
    pub tags: Vec<&'static str>,
    pub deprecated: bool,
    pub response_config: Option<ResponseCallback>,
}

impl Docs {
    /// Create a new documentation builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the summary (short description) for the endpoint.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Docs;
    ///
    /// let docs = Docs::new().summary("Get all users");
    /// ```
    pub fn summary(mut self, text: &'static str) -> Self {
        self.summary = Some(text);
        self
    }

    /// Set the detailed description for the endpoint.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Docs;
    ///
    /// let docs = Docs::new()
    ///     .summary("Create user")
    ///     .description("Creates a new user account with the provided information. Requires admin privileges.");
    /// ```
    pub fn description(mut self, text: &'static str) -> Self {
        self.description = Some(text);
        self
    }

    /// Add a tag to categorize the endpoint.
    ///
    /// Tags are used to group related endpoints in API documentation.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Docs;
    ///
    /// let docs = Docs::new()
    ///     .summary("Get user")
    ///     .tag("users")
    ///     .tag("public");
    /// ```
    pub fn tag(mut self, tag: &'static str) -> Self {
        self.tags.push(tag);
        self
    }

    /// Mark the endpoint as deprecated.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Docs;
    ///
    /// let docs = Docs::new()
    ///     .summary("Old API endpoint")
    ///     .deprecated();
    /// ```
    pub fn deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }

    /// Configure OpenAPI responses for this endpoint.
    ///
    /// This allows you to document different response status codes and their types.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncovr::server::endpoint::Docs;
    /// use uncovr::prelude::*;
    /// # struct UserResponse;
    /// # impl schemars::JsonSchema for UserResponse {
    /// #     fn schema_name() -> String { "UserResponse".to_string() }
    /// #     fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    /// #         schemars::schema::Schema::Bool(true)
    /// #     }
    /// # }
    ///
    /// let docs = Docs::new()
    ///     .summary("Get user")
    ///     .responses(|op| {
    ///         op.response::<200, Json<UserResponse>>()
    ///           .response::<404, Json<ErrorResponse>>()
    ///           .response::<500, Json<ErrorResponse>>()
    ///     });
    /// ```
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

/// Trait for defining API endpoints.
///
/// Separates route definition from documentation, allowing optional documentation
/// and better separation of concerns.
///
/// # Example
///
/// ```no_run
/// use uncovr::server::endpoint::{Endpoint, Route, Docs};
///
/// struct GetUsers;
///
/// impl Endpoint for GetUsers {
///     fn ep(&self) -> Route {
///         Route::GET("/users")
///             .query("page")
///             .query("limit").required()
///     }
///
///     fn docs(&self) -> Option<Docs> {
///         Some(Docs::new()
///             .summary("List all users")
///             .description("Returns a paginated list of users")
///             .tag("users"))
///     }
/// }
/// ```
pub trait Endpoint {
    /// Define the route (path, method, parameters).
    ///
    /// This is the core routing definition for the endpoint.
    fn ep(&self) -> Route;

    /// Optional documentation for the endpoint.
    ///
    /// Return `None` for quick prototyping or internal endpoints.
    /// Return `Some(Docs)` for production APIs with full documentation.
    fn docs(&self) -> Option<Docs> {
        None
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
        let route = Route::GET("/users");
        assert_eq!(route.path, "/users");
        assert_eq!(route.method, HttpMethod::GET);

        let route = Route::POST("/users");
        assert_eq!(route.method, HttpMethod::POST);
    }

    #[test]
    fn test_route_with_params() {
        let mut route = Route::GET("/users");
        route.query("page").required();
        route.query("limit");

        assert_eq!(route.query_params.len(), 2);
        assert_eq!(route.query_params[0].name, "page");
        assert!(route.query_params[0].required);
        assert_eq!(route.query_params[1].name, "limit");
        assert!(!route.query_params[1].required);
    }

    #[test]
    fn test_docs_builder() {
        let docs = Docs::new()
            .summary("Test endpoint")
            .description("This is a test")
            .tag("test")
            .tag("example")
            .deprecated();

        assert_eq!(docs.summary, Some("Test endpoint"));
        assert_eq!(docs.description, Some("This is a test"));
        assert_eq!(docs.tags.len(), 2);
        assert!(docs.deprecated);
    }

    struct TestEndpoint;

    impl Endpoint for TestEndpoint {
        fn ep(&self) -> Route {
            Route::GET("/test")
        }
    }

    #[test]
    fn test_endpoint_trait() {
        let endpoint = TestEndpoint;
        let route = endpoint.ep();

        assert_eq!(route.path, "/test");
        assert_eq!(route.method, HttpMethod::GET);
        assert!(endpoint.docs().is_none());
    }
}
