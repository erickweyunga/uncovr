//! Request parameter types for path and query parameters.
//!
//! This module provides types for working with URL path parameters and query strings
//! in a type-safe manner.

use std::collections::HashMap;
use std::str::FromStr;

/// Path parameters extracted from the URL.
///
/// Example: For route `/users/:id/:name`, accessing `/users/42/alice`:
/// - `path.get("id")` returns `Some("42")`
/// - `path.get::<u64>("id")` returns `Some(42)`
/// - `path.get("name")` returns `Some("alice")`
#[derive(Debug, Clone, Default)]
pub struct Path {
    params: HashMap<String, String>,
}

impl Path {
    /// Create new Path from a HashMap
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }

    /// Create empty Path
    pub fn empty() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Get a parameter as a string slice
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Get a parameter as a String
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.params.get(key).cloned()
    }

    /// Parse a parameter as any type that implements FromStr
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let id: i64 = ctx.path.parse("id")?;
    /// let active: bool = ctx.path.parse("active")?;
    /// ```
    pub fn parse<T: FromStr>(&self, key: &str) -> Result<T, ParamError> {
        let value = self
            .params
            .get(key)
            .ok_or_else(|| ParamError::Missing(key.to_string()))?;

        value.parse().map_err(|_| ParamError::InvalidType {
            key: key.to_string(),
            value: value.clone(),
            expected: std::any::type_name::<T>(),
        })
    }

    /// Get a parameter and parse as u64
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i64
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as u32
    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i32
    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as f64
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as bool
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.params.get(key)?.parse().ok()
    }

    /// Check if a parameter exists
    pub fn contains(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Get all parameter names
    pub fn keys(&self) -> Vec<&str> {
        self.params.keys().map(|s| s.as_str()).collect()
    }
}

/// Query parameters extracted from the URL query string.
///
/// Example: For URL `/users?page=2&limit=10`:
/// - `query.get("page")` returns `Some("2")`
/// - `query.get::<u32>("page")` returns `Some(2)`
/// - `query.get::<u32>("limit")` returns `Some(10)`
#[derive(Debug, Clone, Default)]
pub struct Query {
    params: HashMap<String, String>,
}

impl Query {
    /// Create new Query from a HashMap
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }

    /// Create empty Query
    pub fn empty() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Get a parameter as a string slice
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Get a parameter as a String
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.params.get(key).cloned()
    }

    /// Parse a parameter as any type that implements FromStr
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let page: u32 = ctx.query.parse("page").unwrap_or(1);
    /// let limit: u32 = ctx.query.parse("limit").unwrap_or(10);
    /// ```
    pub fn parse<T: FromStr>(&self, key: &str) -> Result<T, ParamError> {
        let value = self
            .params
            .get(key)
            .ok_or_else(|| ParamError::Missing(key.to_string()))?;

        value.parse().map_err(|_| ParamError::InvalidType {
            key: key.to_string(),
            value: value.clone(),
            expected: std::any::type_name::<T>(),
        })
    }

    /// Get a parameter and parse as u64
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i64
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as u32
    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i32
    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as f64
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as bool
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.params.get(key)?.parse().ok()
    }

    /// Check if a parameter exists
    pub fn contains(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Get all parameter names
    pub fn keys(&self) -> Vec<&str> {
        self.params.keys().map(|s| s.as_str()).collect()
    }
}

/// Error type for parameter extraction
#[derive(Debug, Clone)]
pub enum ParamError {
    /// Parameter is missing
    Missing(String),
    /// Parameter value cannot be parsed to expected type
    InvalidType {
        key: String,
        value: String,
        expected: &'static str,
    },
}

impl std::fmt::Display for ParamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamError::Missing(key) => write!(f, "Missing parameter: {}", key),
            ParamError::InvalidType {
                key,
                value,
                expected,
            } => write!(
                f,
                "Invalid parameter '{}': cannot parse '{}' as {}",
                key, value, expected
            ),
        }
    }
}

impl std::error::Error for ParamError {}

/// Legacy type alias for backward compatibility during migration
#[deprecated(since = "0.3.0", note = "Use `Path` instead")]
pub type PathParams = Path;

/// Legacy type alias for backward compatibility during migration
#[deprecated(since = "0.3.0", note = "Use `Query` instead")]
pub type QueryParams = Query;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_params() {
        let mut map = HashMap::new();
        map.insert("id".to_string(), "42".to_string());
        map.insert("name".to_string(), "alice".to_string());

        let params = Path::new(map);

        assert_eq!(params.get("id"), Some("42"));
        assert_eq!(params.get_u64("id"), Some(42));
        assert_eq!(params.get("name"), Some("alice"));
        assert!(params.contains("id"));
        assert!(!params.contains("age"));
    }

    #[test]
    fn test_path_parse() {
        let mut map = HashMap::new();
        map.insert("id".to_string(), "42".to_string());
        map.insert("active".to_string(), "true".to_string());

        let params = Path::new(map);

        assert_eq!(params.parse::<i64>("id").unwrap(), 42);
        assert_eq!(params.parse::<bool>("active").unwrap(), true);
        assert!(params.parse::<i64>("missing").is_err());
    }

    #[test]
    fn test_query_params() {
        let mut map = HashMap::new();
        map.insert("page".to_string(), "2".to_string());
        map.insert("limit".to_string(), "10".to_string());

        let params = Query::new(map);

        assert_eq!(params.get_u32("page"), Some(2));
        assert_eq!(params.get_u32("limit"), Some(10));
        assert_eq!(params.get("page"), Some("2"));
    }

    #[test]
    fn test_query_parse() {
        let mut map = HashMap::new();
        map.insert("page".to_string(), "2".to_string());
        map.insert("limit".to_string(), "10".to_string());

        let params = Query::new(map);

        assert_eq!(params.parse::<u32>("page").unwrap(), 2);
        assert_eq!(params.parse::<u32>("limit").unwrap(), 10);
        assert!(params.parse::<u32>("missing").is_err());
    }
}
