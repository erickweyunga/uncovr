//! Application configuration types for Uncovr framework.
//!
//! This module provides configuration structures for managing application settings,
//! including logging, CORS, environment settings, and more.
//!
//! # Examples
//!
//! ```rust
//! use uncovr::config::{AppConfig, LoggingConfig, CorsConfig, Environment};
//!
//! let config = AppConfig::new("My API", "1.0.0")
//!     .description("My awesome API")
//!     .environment(Environment::Development)
//!     .logging(LoggingConfig::development())
//!     .cors(CorsConfig::development());
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Logging level configuration.
///
/// Determines the minimum severity level of log messages to display.
/// Levels from most to least verbose: Trace, Debug, Info, Warn, Error.
///
/// # Examples
///
/// ```rust
/// use uncovr::config::LogLevel;
///
/// let level = LogLevel::Debug;
/// assert_eq!(level.as_filter(), "debug");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Most verbose level - includes all log messages
    Trace,
    /// Debug information useful for development
    Debug,
    /// General informational messages
    #[default]
    Info,
    /// Warning messages for potentially problematic situations
    Warn,
    /// Error messages for failures
    Error,
}

impl LogLevel {
    /// Convert to tracing filter string
    pub fn as_filter(&self) -> &str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

/// Logging configuration for the application.
///
/// Controls all aspects of application logging including level, format,
/// and what to log (requests, responses).
///
/// # Examples
///
/// ```rust
/// use uncovr::config::{LoggingConfig, LogLevel, LogFormat};
///
/// // Development configuration
/// let dev_config = LoggingConfig::development();
///
/// // Production configuration
/// let prod_config = LoggingConfig::production();
///
/// // Custom configuration
/// let custom = LoggingConfig::development()
///     .level(LogLevel::Info)
///     .log_responses(false);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Enable or disable logging entirely
    pub enabled: bool,

    /// Minimum log level to display
    pub level: LogLevel,

    /// Enable HTTP request logging (method, URI, user agent, latency)
    pub log_requests: bool,

    /// Enable HTTP response logging (status, headers)
    pub log_responses: bool,

    /// Output format (Pretty for development, Json for production)
    pub format: LogFormat,
}

/// Log output format.
///
/// Determines how log messages are formatted and displayed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Pretty formatted logs with colors (ideal for development)
    Pretty,
    /// JSON formatted logs for log aggregation systems (ideal for production)
    Json,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            log_requests: true,
            log_responses: false,
            format: LogFormat::Pretty,
        }
    }
}

impl LoggingConfig {
    /// Create a development logging configuration.
    ///
    /// Enables verbose logging with pretty formatting, suitable for local development.
    /// - Level: Debug
    /// - Format: Pretty (with colors)
    /// - Logs: Requests and responses
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uncovr::config::LoggingConfig;
    ///
    /// let config = LoggingConfig::development();
    /// ```
    pub fn development() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Debug,
            log_requests: true,
            log_responses: true,
            format: LogFormat::Pretty,
        }
    }

    /// Create a production logging configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            log_requests: true,
            log_responses: false,
            format: LogFormat::Json,
        }
    }

    /// Disable logging
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            log_requests: false,
            log_responses: false,
            format: LogFormat::Pretty,
        }
    }

    /// Set the log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Enable or disable request logging
    pub fn log_requests(mut self, enabled: bool) -> Self {
        self.log_requests = enabled;
        self
    }

    /// Enable or disable response logging
    pub fn log_responses(mut self, enabled: bool) -> Self {
        self.log_responses = enabled;
        self
    }

    /// Set the log format
    pub fn format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Enable or disable logging entirely
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Application environment setting.
///
/// Determines default configurations for logging, CORS, and other settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    /// Development environment with verbose logging and permissive CORS
    #[default]
    Development,
    /// Staging environment with production-like settings for testing
    Staging,
    /// Production environment with optimized settings and restricted CORS
    Production,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Allowed origins. Use ["*"] to allow all origins (not recommended for production)
    pub allowed_origins: Vec<String>,

    /// Allowed HTTP methods
    pub allowed_methods: Vec<String>,

    /// Allowed headers
    pub allowed_headers: Vec<String>,

    /// Whether to allow credentials
    #[serde(default)]
    pub allow_credentials: bool,

    /// Max age for preflight requests in seconds
    #[serde(default)]
    pub max_age: Option<u64>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec![],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec!["*".to_string()],
            allow_credentials: false,
            max_age: Some(3600),
        }
    }
}

impl CorsConfig {
    /// Create a development CORS config that allows all origins
    pub fn development() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec!["*".to_string()],
            allow_credentials: false,
            max_age: Some(3600),
        }
    }

    /// Create a production CORS config with specific origins
    pub fn production(origins: Vec<String>) -> Self {
        Self {
            allowed_origins: origins,
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
            ],
            allowed_headers: vec!["content-type".to_string(), "authorization".to_string()],
            allow_credentials: true,
            max_age: Some(3600),
        }
    }

    /// Check if all origins are allowed
    pub fn allows_all_origins(&self) -> bool {
        self.allowed_origins.contains(&"*".to_string())
    }
}

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub name: String,

    /// Application description
    pub description: String,

    /// Application version
    pub version: String,

    /// Server bind address
    #[serde(default = "default_bind_address")]
    pub bind_address: String,

    /// Environment
    #[serde(default)]
    pub environment: Environment,

    /// CORS configuration
    pub cors: CorsConfig,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,

    /// Environment variables
    #[serde(default)]
    pub env_vars: HashMap<String, String>,

    /// Enable OpenAPI documentation
    #[serde(default = "default_true")]
    pub enable_docs: bool,

    /// OpenAPI server URLs
    #[serde(default)]
    pub api_servers: Vec<ApiServer>,

    /// Enable response compression (gzip, brotli)
    #[serde(default = "default_true")]
    pub enable_compression: bool,

    /// Maximum number of concurrent connections (None = unlimited)
    #[serde(default)]
    pub max_connections: Option<usize>,

    /// TCP keep-alive timeout in seconds (None = disabled)
    #[serde(default)]
    pub keep_alive_timeout: Option<u64>,

    /// Connection timeout in seconds
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
}

/// OpenAPI server configuration.
///
/// Defines a server entry in the OpenAPI specification for API documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServer {
    /// The URL of the API server
    pub url: String,
    /// Description of this server instance
    pub description: String,
}

fn default_bind_address() -> String {
    "127.0.0.1:3000".to_string()
}

fn default_true() -> bool {
    true
}

fn default_connection_timeout() -> u64 {
    30
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Uncovr API".to_string(),
            description: "API built with Uncovr framework".to_string(),
            version: "1.0.0".to_string(),
            bind_address: default_bind_address(),
            environment: Environment::Development,
            cors: CorsConfig::development(),
            logging: LoggingConfig::default(),
            env_vars: HashMap::new(),
            enable_docs: true,
            api_servers: vec![ApiServer {
                url: "http://localhost:3000".to_string(),
                description: "Local development".to_string(),
            }],
            enable_compression: true,
            max_connections: None,
            keep_alive_timeout: Some(60),
            connection_timeout: 30,
        }
    }
}

impl AppConfig {
    /// Create a new application configuration
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            ..Default::default()
        }
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the bind address
    ///
    /// This automatically updates the api_servers list to match the bind address,
    /// ensuring the OpenAPI documentation shows the correct server URL.
    pub fn bind(mut self, address: impl Into<String>) -> Self {
        let addr = address.into();
        self.bind_address = addr.clone();

        let url = if addr.starts_with("0.0.0.0:") {
            format!(
                "http://localhost:{}",
                addr.strip_prefix("0.0.0.0:").unwrap()
            )
        } else {
            format!("http://{}", addr)
        };

        self.api_servers = vec![ApiServer {
            url,
            description: "Local development".to_string(),
        }];

        self
    }

    /// Set the environment
    pub fn environment(mut self, env: Environment) -> Self {
        // Auto-configure CORS based on environment
        self.cors = match &env {
            Environment::Development => CorsConfig::development(),
            Environment::Staging | Environment::Production => {
                // Use existing origins or empty list
                CorsConfig::production(self.cors.allowed_origins.clone())
            }
        };

        self.environment = env;
        self
    }

    /// Set CORS configuration
    pub fn cors(mut self, cors: CorsConfig) -> Self {
        self.cors = cors;
        self
    }

    /// Set logging configuration
    pub fn logging(mut self, logging: LoggingConfig) -> Self {
        self.logging = logging;
        self
    }

    /// Add an environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.insert(key.into(), value.into());
        self
    }

    /// Enable or disable documentation
    pub fn docs(mut self, enable: bool) -> Self {
        self.enable_docs = enable;
        self
    }

    /// Add an API server URL
    pub fn add_server(mut self, url: impl Into<String>, description: impl Into<String>) -> Self {
        self.api_servers.push(ApiServer {
            url: url.into(),
            description: description.into(),
        });
        self
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }

    /// Enable or disable response compression
    pub fn compression(mut self, enable: bool) -> Self {
        self.enable_compression = enable;
        self
    }

    /// Set maximum concurrent connections
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Set TCP keep-alive timeout in seconds
    pub fn keep_alive_timeout(mut self, timeout: u64) -> Self {
        self.keep_alive_timeout = Some(timeout);
        self
    }

    /// Set connection timeout in seconds
    pub fn connection_timeout(mut self, timeout: u64) -> Self {
        self.connection_timeout = timeout;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.environment, Environment::Development);
        assert!(config.cors.allows_all_origins());
        assert_eq!(config.bind_address, "127.0.0.1:3000");
    }

    #[test]
    fn test_production_config() {
        let config = AppConfig::new("My API", "1.0.0")
            .environment(Environment::Production)
            .cors(CorsConfig::production(vec![
                "https://example.com".to_string(),
            ]));

        assert_eq!(config.environment, Environment::Production);
        assert!(!config.cors.allows_all_origins());
        assert_eq!(config.cors.allowed_origins[0], "https://example.com");
    }

    #[test]
    fn test_development_cors() {
        let cors = CorsConfig::development();
        assert!(cors.allows_all_origins());
        assert!(!cors.allow_credentials);
    }

    #[test]
    fn test_production_cors() {
        let cors = CorsConfig::production(vec!["https://example.com".to_string()]);
        assert!(!cors.allows_all_origins());
        assert!(cors.allow_credentials);
    }
}
