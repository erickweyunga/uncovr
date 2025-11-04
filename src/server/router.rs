use axum::Router;

/// Extension trait for Router to add Uncover-specific functionality.
///
/// This trait is automatically implemented for all Axum Routers.
pub trait RouterExt {
    /// Adds CORS support with permissive defaults.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use uncover::server::RouterExt;
    ///
    /// let router = Router::new()
    ///     .with_cors();
    /// ```
    #[cfg(feature = "cors")]
    fn with_cors(self) -> Self;

    /// Adds request logging middleware.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let router = Router::new()
    ///     .with_logging();
    /// ```
    #[cfg(feature = "logging")]
    fn with_logging(self) -> Self;
}

impl RouterExt for Router {
    #[cfg(feature = "cors")]
    fn with_cors(self) -> Self {
        use tower_http::cors::CorsLayer;
        self.layer(CorsLayer::permissive())
    }

    #[cfg(feature = "logging")]
    fn with_logging(self) -> Self {
        use tower_http::trace::TraceLayer;
        self.layer(TraceLayer::new_for_http())
    }
}
