//! Endpoint definition module for Uncovr framework.
//!
//! This module provides a clean separation between route definition and metadata (documentation).
//!
//! # Example
//!
//! ```no_run
//! use uncovr::server::endpoint::{Endpoint, Route, Meta};
//!
//! struct CreateUser;
//!
//! impl Endpoint for CreateUser {
//!     fn route(&self) -> Route {
//!         Route::post("/users")
//!     }
//!
//!     fn meta(&self) -> Meta {
//!         Meta::new()
//!             .summary("Create a new user")
//!             .describe("Creates a user with the provided information")
//!             .tag("users")
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
/// let route = Route::post("/users/:id")
///     .param("id", "User ID")
///     .query("notify");
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

    /// Create a GET route (lowercase, following Rust conventions).
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::get("/users");
    /// ```
    pub fn get(path: &'static str) -> Self {
        Self::new(HttpMethod::GET, path)
    }

    /// Create a POST route (lowercase, following Rust conventions).
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::post("/users");
    /// ```
    pub fn post(path: &'static str) -> Self {
        Self::new(HttpMethod::POST, path)
    }

    /// Create a PUT route (lowercase, following Rust conventions).
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::put("/users/:id");
    /// ```
    pub fn put(path: &'static str) -> Self {
        Self::new(HttpMethod::PUT, path)
    }

    /// Create a PATCH route (lowercase, following Rust conventions).
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::patch("/users/:id");
    /// ```
    pub fn patch(path: &'static str) -> Self {
        Self::new(HttpMethod::PATCH, path)
    }

    /// Create a DELETE route (lowercase, following Rust conventions).
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::delete("/users/:id");
    /// ```
    pub fn delete(path: &'static str) -> Self {
        Self::new(HttpMethod::DELETE, path)
    }

    /// Create an OPTIONS route (lowercase, following Rust conventions).
    pub fn options(path: &'static str) -> Self {
        Self::new(HttpMethod::OPTIONS, path)
    }

    /// Create a HEAD route (lowercase, following Rust conventions).
    pub fn head(path: &'static str) -> Self {
        Self::new(HttpMethod::HEAD, path)
    }

    /// Add a query parameter with optional description.
    ///
    /// Returns a mutable reference to self for further chaining,
    /// or use `.required()` or `.desc()` to configure the parameter.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::get("/users")
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

    /// Add a path parameter with description (shorthand method).
    ///
    /// This is a convenience method that combines adding a parameter and description.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::delete("/users/:id")
    ///     .param("id", "User ID");
    /// ```
    pub fn param(mut self, name: &'static str, description: &'static str) -> Self {
        self.path_params.push(PathParam {
            name,
            description: Some(description),
            required: true,
        });
        self
    }

    /// Add a path parameter (legacy method for backward compatibility).
    ///
    /// Returns a builder for fluent configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Route;
    ///
    /// let route = Route::delete("/users/:id")
    ///     .path_param("id").desc("User ID").required();
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

/// Metadata for an API endpoint.
///
/// Provides human-readable information about the endpoint for API documentation
/// and OpenAPI specification generation.
///
/// # Example
///
/// ```
/// use uncovr::server::endpoint::Meta;
///
/// let meta = Meta::new()
///     .summary("Create a new user")
///     .describe("Creates a user with the provided information")
///     .tag("users")
///     .tag("authentication");
/// ```
#[derive(Default)]
pub struct Meta {
    pub summary: Option<&'static str>,
    pub description: Option<&'static str>,
    pub tags: Vec<&'static str>,
    pub deprecated: bool,
    pub response_config: Option<ResponseCallback>,
}

impl Meta {
    /// Create a new metadata builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the summary (short description) for the endpoint.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Meta;
    ///
    /// let meta = Meta::new().summary("Get all users");
    /// ```
    pub fn summary(mut self, text: &'static str) -> Self {
        self.summary = Some(text);
        self
    }

    /// Set the detailed description for the endpoint.
    ///
    /// Renamed from `description` to `describe` for verb consistency.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Meta;
    ///
    /// let meta = Meta::new()
    ///     .summary("Create user")
    ///     .describe("Creates a new user account with the provided information. Requires admin privileges.");
    /// ```
    pub fn describe(mut self, text: &'static str) -> Self {
        self.description = Some(text);
        self
    }

    /// Set the detailed description for the endpoint (alias for backward compatibility).
    ///
    /// Use `describe()` for verb consistency in new code.
    pub fn description(self, text: &'static str) -> Self {
        self.describe(text)
    }

    /// Add a tag to categorize the endpoint.
    ///
    /// Tags are used to group related endpoints in API documentation.
    ///
    /// # Example
    ///
    /// ```
    /// use uncovr::server::endpoint::Meta;
    ///
    /// let meta = Meta::new()
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
    /// use uncovr::server::endpoint::Meta;
    ///
    /// let meta = Meta::new()
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
    /// use uncovr::server::endpoint::Meta;
    /// use uncovr::prelude::*;
    /// # struct UserResponse;
    /// # impl schemars::JsonSchema for UserResponse {
    /// #     fn schema_name() -> String { "UserResponse".to_string() }
    /// #     fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    /// #         schemars::schema::Schema::Bool(true)
    /// #     }
    /// # }
    ///
    /// let meta = Meta::new()
    ///     .summary("Get user")
    ///     .responses(|op| {
    ///         op.response::<200, Json<UserResponse>>()
    ///           .response::<404, Json<Error>>()
    ///           .response::<500, Json<Error>>()
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
/// Separates route definition from metadata (documentation), providing clear
/// separation of concerns.
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
///             .query("page")
///             .query("limit").required()
///     }
///
///     fn meta(&self) -> Meta {
///         Meta::new()
///             .summary("List all users")
///             .describe("Returns a paginated list of users")
///             .tag("users")
///     }
/// }
/// ```
pub trait Endpoint {
    /// Define the route (path, method, parameters).
    ///
    /// This is the core routing definition for the endpoint.
    fn route(&self) -> Route;

    /// Define metadata (documentation) for the endpoint.
    ///
    /// Always return metadata for proper API documentation.
    fn meta(&self) -> Meta {
        Meta::new()
    }
}

/// Legacy type alias for backward compatibility during migration
#[deprecated(since = "0.3.0", note = "Use `Meta` instead")]
pub type Docs = Meta;

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
