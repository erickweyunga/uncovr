//! Simplified application configuration
//!
//! This module provides a lightweight configuration structure focused on
//! application metadata and basic server settings.

use serde::{Deserialize, Serialize};

/// Simplified application configuration
///
/// Focuses on application metadata and basic settings.
/// Middleware (CORS, logging, etc.) is configured separately via the server builder.
///
/// # Example
///
/// ```rust
/// use uncovr::config::AppConfig;
///
/// let config = AppConfig::new("My API", "1.0.0")
///     .description("My awesome API")
///     .bind("0.0.0.0:8080");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub name: String,

    /// Application description
    pub description: String,

    /// Application version
    pub version: String,

    /// Server bind address
    pub bind_address: String,

    /// Enable OpenAPI documentation
    pub enable_docs: bool,

    /// OpenAPI documentation UI path (default: "/docs")
    pub docs_path: String,

    /// OpenAPI JSON specification path (default: "/openapi.json")
    pub openapi_json_path: String,

    /// OpenAPI server URLs
    pub api_servers: Vec<ApiServer>,
}

/// OpenAPI server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServer {
    /// The URL of the API server
    pub url: String,
    /// Description of this server instance
    pub description: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Uncovr API".to_string(),
            description: "API built with Uncovr framework".to_string(),
            version: "1.0.0".to_string(),
            bind_address: "127.0.0.1:3000".to_string(),
            enable_docs: true,
            docs_path: "/docs".to_string(),
            openapi_json_path: "/openapi.json".to_string(),
            api_servers: vec![],
        }
    }
}

impl AppConfig {
    /// Create a new application configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::AppConfig;
    ///
    /// let config = AppConfig::new("My API", "1.0.0");
    /// ```
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            ..Default::default()
        }
    }

    /// Set the description
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::AppConfig;
    ///
    /// let config = AppConfig::new("My API", "1.0.0")
    ///     .description("A REST API for managing users");
    /// ```
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the bind address
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::AppConfig;
    ///
    /// let config = AppConfig::new("My API", "1.0.0")
    ///     .bind("0.0.0.0:8080");
    /// ```
    pub fn bind(mut self, address: impl Into<String>) -> Self {
        self.bind_address = address.into();
        self
    }

    /// Enable or disable OpenAPI documentation
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::AppConfig;
    ///
    /// let config = AppConfig::new("My API", "1.0.0")
    ///     .docs(false); // Disable docs in production
    /// ```
    pub fn docs(mut self, enable: bool) -> Self {
        self.enable_docs = enable;
        self
    }

    /// Set the path for the OpenAPI documentation UI
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::AppConfig;
    ///
    /// let config = AppConfig::new("My API", "1.0.0")
    ///     .docs_path("/swagger");
    /// ```
    pub fn docs_path(mut self, path: impl Into<String>) -> Self {
        self.docs_path = path.into();
        self
    }

    /// Set the path for the OpenAPI JSON specification
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::AppConfig;
    ///
    /// let config = AppConfig::new("My API", "1.0.0")
    ///     .openapi_json_path("/api-spec.json");
    /// ```
    pub fn openapi_json_path(mut self, path: impl Into<String>) -> Self {
        self.openapi_json_path = path.into();
        self
    }

    /// Add an API server URL to the OpenAPI specification
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::AppConfig;
    ///
    /// let config = AppConfig::new("My API", "1.0.0")
    ///     .add_server("https://api.example.com", "Production")
    ///     .add_server("https://staging-api.example.com", "Staging");
    /// ```
    pub fn add_server(mut self, url: impl Into<String>, description: impl Into<String>) -> Self {
        self.api_servers.push(ApiServer {
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
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.name, "Uncovr API");
        assert_eq!(config.bind_address, "127.0.0.1:3000");
        assert!(config.enable_docs);
    }

    #[test]
    fn test_new_config() {
        let config = AppConfig::new("Test API", "2.0.0")
            .description("Test description")
            .bind("0.0.0.0:8080")
            .docs(false);

        assert_eq!(config.name, "Test API");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.description, "Test description");
        assert_eq!(config.bind_address, "0.0.0.0:8080");
        assert!(!config.enable_docs);
    }

    #[test]
    fn test_add_server() {
        let config = AppConfig::new("Test API", "1.0.0")
            .add_server("https://api.example.com", "Production")
            .add_server("https://staging.example.com", "Staging");

        assert_eq!(config.api_servers.len(), 2);
        assert_eq!(config.api_servers[0].url, "https://api.example.com");
        assert_eq!(config.api_servers[1].description, "Staging");
    }
}
