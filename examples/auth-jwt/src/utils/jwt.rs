use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// JWT secret key - In production, load this from environment variables!
const JWT_SECRET: &str = "your-secret-key-change-this-in-production";

/// JWT token expiration time (24 hours)
const TOKEN_EXPIRATION_HOURS: i64 = 24;

/// Claims stored in the JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub email: String, // User email
    pub exp: usize,    // Expiration time (Unix timestamp)
    pub iat: usize,    // Issued at (Unix timestamp)
}

/// Generate a JWT token for a user
pub fn generate_token(user_id: i64, email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expiration = now + Duration::hours(TOKEN_EXPIRATION_HOURS);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

/// Validate a JWT token and return the claims
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

/// Extract token from Authorization header
pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") {
        Some(&auth_header[7..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let token = generate_token(1, "test@example.com").unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, "1");
        assert_eq!(claims.email, "test@example.com");
    }

    #[test]
    fn test_extract_token() {
        let header = "Bearer abc123token";
        let token = extract_token_from_header(header);
        assert_eq!(token, Some("abc123token"));
    }
}
