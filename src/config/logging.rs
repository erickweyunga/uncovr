//! Logging configuration types
//!
//! Simple logging configuration that can be passed to the logging initialization.

use serde::{Deserialize, Serialize};

/// Logging level configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
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

/// Log output format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Pretty formatted logs with colors (ideal for development)
    Pretty,
    /// JSON formatted logs for log aggregation systems (ideal for production)
    Json,
}

/// Logging configuration
///
/// Simple configuration for application logging.
///
/// # Example
///
/// ```rust
/// use uncovr::config::{Logging, LogLevel, LogFormat};
///
/// // Development
/// let config = Logging::development();
///
/// // Production
/// let config = Logging::production();
///
/// // Custom
/// let config = Logging::new(LogLevel::Info, LogFormat::Json);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logging {
    /// Minimum log level to display
    pub level: LogLevel,

    /// Output format
    pub format: LogFormat,

    /// Enable HTTP request logging
    pub log_requests: bool,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Pretty,
            log_requests: true,
        }
    }
}

impl Logging {
    /// Create a new logging configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::{Logging, LogLevel, LogFormat};
    ///
    /// let config = Logging::new(LogLevel::Debug, LogFormat::Pretty);
    /// ```
    pub fn new(level: LogLevel, format: LogFormat) -> Self {
        Self {
            level,
            format,
            log_requests: true,
        }
    }

    /// Create a development logging configuration
    ///
    /// - Level: Debug
    /// - Format: Pretty
    /// - Request logging: Enabled
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::Logging;
    ///
    /// let config = Logging::development();
    /// ```
    pub fn development() -> Self {
        Self {
            level: LogLevel::Debug,
            format: LogFormat::Pretty,
            log_requests: true,
        }
    }

    /// Create a production logging configuration
    ///
    /// - Level: Info
    /// - Format: JSON
    /// - Request logging: Enabled
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::config::Logging;
    ///
    /// let config = Logging::production();
    /// ```
    pub fn production() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Json,
            log_requests: true,
        }
    }

    /// Set the log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the log format
    pub fn format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Enable or disable request logging
    pub fn log_requests(mut self, enabled: bool) -> Self {
        self.log_requests = enabled;
        self
    }
}
