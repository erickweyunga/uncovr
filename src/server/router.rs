use axum::Router;

/// Extension trait for a web application router to add Uncovr-specific functionality.
///
/// This trait is automatically implemented for the router type.
pub trait RouterExt {
    /// Adds Cross-Origin Resource Sharing (CORS) support with permissive defaults.
    #[cfg(feature = "cors")]
    fn with_cors(self) -> Self;

    /// Adds request logging middleware.
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
