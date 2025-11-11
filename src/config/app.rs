//! Application configuration
//!
//! This module provides configuration for application metadata and server settings.

use serde::{Deserialize, Serialize};

/// Application configuration
///
/// Focuses on application metadata and basic settings.
/// Middleware (CORS, logging, etc.) is configured separately via the server builder.
///
/// # Example
///
/// ```rust
/// use uncovr::config::App;
///
/// let app = App::new("My API", "1.0.0", "0.0.0.0:8080")
///     .description("My awesome API");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    /// Application name
    pub name: String,

    /// Application description
    pub description: String,

    /// Application version
    pub version: String,

    /// Bind address
    pub bind: String,

    /// Enable documentation
    pub docs: bool,

    /// Documentation path (default: "/docs")
    pub docs_path: String,

    /// Specification path (default: "/api.json")
    pub spec_path: String,

    /// Server URLs
    pub servers: Vec<Server>,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// Server URL
    pub url: String,
    /// Server description
    pub description: String,
}

impl App {
    /// Create a new application configuration
    ///
    /// # Arguments
    ///
    /// * `name` - Application name
    /// * `version` - Application version
    /// * `bind` - Bind address (e.g., "0.0.0.0:8080" or "127.0.0.1:3000")
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::App;
    ///
    /// let app = App::new("My API", "1.0.0", "0.0.0.0:8080");
    /// ```
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        bind: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            bind: bind.into(),
            docs: true,
            docs_path: "/docs".to_string(),
            spec_path: "/api.json".to_string(),
            servers: vec![],
        }
    }

    /// Set the description
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::App;
    ///
    /// let app = App::new("My API", "1.0.0", "0.0.0.0:8080")
    ///     .description("A REST API for managing users");
    /// ```
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Enable or disable documentation
    pub fn with_docs(mut self, enable: bool) -> Self {
        self.docs = enable;
        self
    }

    /// Set documentation UI path
    pub fn with_docs_path(mut self, path: impl Into<String>) -> Self {
        self.docs_path = path.into();
        self
    }

    /// Set specification path
    pub fn with_spec_path(mut self, path: impl Into<String>) -> Self {
        self.spec_path = path.into();
        self
    }

    /// Add a server URL
    pub fn server(mut self, url: impl Into<String>, description: impl Into<String>) -> Self {
        self.servers.push(Server {
            url: url.into(),
            description: description.into(),
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app() {
        let app = App::new("Test API", "2.0.0", "0.0.0.0:8080")
            .description("Test description")
            .with_docs(false);

        assert_eq!(app.name, "Test API");
        assert_eq!(app.version, "2.0.0");
        assert_eq!(app.description, "Test description");
        assert_eq!(app.bind, "0.0.0.0:8080");
        assert!(!app.docs);
    }

    #[test]
    fn test_add_server() {
        let app = App::new("Test API", "1.0.0", "127.0.0.1:3000")
            .server("https://api.example.com", "Production")
            .server("https://staging.example.com", "Staging");

        assert_eq!(app.servers.len(), 2);
        assert_eq!(app.servers[0].url, "https://api.example.com");
        assert_eq!(app.servers[1].description, "Staging");
    }
}
