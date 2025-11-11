use http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

/// CORS (Cross-Origin Resource Sharing) middleware configuration
///
/// Provides a simpler API for configuring CORS without coupling to App.
/// Can be used as middleware in the server builder.
///
/// # Example
///
/// ```rust,no_run
/// use uncovr::server::Server;
/// use uncovr::middleware::Cors;
///
/// // Allow all origins (development)
/// let server = Server::new()
///     .layer(Cors::permissive())
///     .register(MyEndpoint)
///     .build();
///
/// // Specific origins (production)
/// let server = Server::new()
///     .layer(Cors::new()
///         .allow_origin("https://example.com")
///         .allow_credentials(true))
///     .register(MyEndpoint)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct Cors {
    allowed_origins: Vec<String>,
    allowed_methods: Vec<Method>,
    allowed_headers: Vec<String>,
    allow_credentials: bool,
    max_age: Option<u64>,
}

impl Cors {
    /// Create a new CORS configuration with sensible defaults
    ///
    /// Defaults:
    /// - No origins allowed (must be explicitly added)
    /// - Methods: GET, POST, PUT, DELETE, PATCH, OPTIONS
    /// - Headers: All (*)
    /// - Credentials: false
    /// - Max age: 1 hour
    pub fn new() -> Self {
        Self {
            allowed_origins: vec![],
            allowed_methods: vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
                Method::OPTIONS,
            ],
            allowed_headers: vec!["*".to_string()],
            allow_credentials: false,
            max_age: Some(3600),
        }
    }

    /// Create a permissive CORS configuration that allows all origins
    ///
    /// Useful for development but **not recommended for production**.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::permissive();
    /// ```
    pub fn permissive() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
                Method::OPTIONS,
            ],
            allowed_headers: vec!["*".to_string()],
            allow_credentials: false,
            max_age: Some(3600),
        }
    }

    /// Create a restrictive CORS configuration for production
    ///
    /// Requires explicit origins, allows credentials, and restricts headers.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::restrictive(vec![
    ///     "https://example.com".to_string(),
    ///     "https://app.example.com".to_string(),
    /// ]);
    /// ```
    pub fn restrictive(origins: Vec<String>) -> Self {
        Self {
            allowed_origins: origins,
            allowed_methods: vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
            ],
            allowed_headers: vec!["content-type".to_string(), "authorization".to_string()],
            allow_credentials: true,
            max_age: Some(3600),
        }
    }

    /// Allow a specific origin
    ///
    /// Can be called multiple times to allow multiple origins.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::new()
    ///     .allow_origin("https://example.com")
    ///     .allow_origin("https://app.example.com");
    /// ```
    pub fn allow_origin(mut self, origin: impl Into<String>) -> Self {
        self.allowed_origins.push(origin.into());
        self
    }

    /// Allow all origins (sets origin to "*")
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::new().allow_all_origins();
    /// ```
    pub fn allow_all_origins(mut self) -> Self {
        self.allowed_origins = vec!["*".to_string()];
        self
    }

    /// Set allowed HTTP methods
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    /// use http::Method;
    ///
    /// let cors = Cors::new()
    ///     .methods(vec![Method::GET, Method::POST]);
    /// ```
    pub fn methods(mut self, methods: Vec<Method>) -> Self {
        self.allowed_methods = methods;
        self
    }

    /// Set allowed headers
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::new()
    ///     .headers(vec!["content-type".to_string(), "authorization".to_string()]);
    /// ```
    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.allowed_headers = headers;
        self
    }

    /// Allow all headers (sets headers to "*")
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::new().allow_all_headers();
    /// ```
    pub fn allow_all_headers(mut self) -> Self {
        self.allowed_headers = vec!["*".to_string()];
        self
    }

    /// Enable or disable credentials
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::new()
    ///     .allow_origin("https://example.com")
    ///     .allow_credentials(true);
    /// ```
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }

    /// Set max age for preflight requests in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::Cors;
    ///
    /// let cors = Cors::new().max_age(7200); // 2 hours
    /// ```
    pub fn max_age(mut self, seconds: u64) -> Self {
        self.max_age = Some(seconds);
        self
    }

    /// Build into a Tower CorsLayer
    ///
    /// This is called internally when the middleware is added to the server.
    pub fn into_layer(self) -> CorsLayer {
        let mut cors = CorsLayer::new();

        // Configure origins
        if self.allowed_origins.contains(&"*".to_string()) {
            cors = cors.allow_origin(Any);
        } else {
            let origins: Vec<HeaderValue> = self
                .allowed_origins
                .iter()
                .filter_map(|o| o.parse().ok())
                .collect();
            if !origins.is_empty() {
                cors = cors.allow_origin(AllowOrigin::list(origins));
            }
        }

        // Configure methods
        cors = cors.allow_methods(self.allowed_methods);

        // Configure headers
        if self.allowed_headers.contains(&"*".to_string()) {
            cors = cors.allow_headers(Any);
        } else {
            let headers: Vec<http::header::HeaderName> = self
                .allowed_headers
                .iter()
                .filter_map(|h| h.parse().ok())
                .collect();
            cors = cors.allow_headers(headers);
        }

        // Configure credentials
        if self.allow_credentials {
            cors = cors.allow_credentials(true);
        }

        // Configure max age
        if let Some(max_age) = self.max_age {
            cors = cors.max_age(std::time::Duration::from_secs(max_age));
        }

        cors
    }
}

impl Default for Cors {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Layer trait so it can be used directly with .layer()
impl<S> tower::Layer<S> for Cors {
    type Service = <CorsLayer as tower::Layer<S>>::Service;

    fn layer(&self, inner: S) -> Self::Service {
        self.clone().into_layer().layer(inner)
    }
}
