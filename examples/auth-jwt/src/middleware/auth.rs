use serde_json::json;
use uncovr::{axum_middleware::Next, extract::Request, prelude::*};

use crate::utils::jwt::{Claims, extract_token_from_header, validate_token};

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub email: String,
}

impl AuthUser {
    pub fn from_claims(claims: Claims) -> Result<Self, String> {
        let user_id = claims
            .sub
            .parse::<i64>()
            .map_err(|_| "Invalid user ID in token".to_string())?;

        Ok(Self {
            user_id,
            email: claims.email,
        })
    }
}

pub async fn auth_middleware(headers: HeaderMap, mut request: Request, next: Next) -> Response {
    // Extract Authorization header
    let auth_header = match headers.get("authorization") {
        Some(header) => match header.to_str() {
            Ok(h) => h,
            Err(_) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "code": "invalid_header",
                        "message": "Invalid authorization header"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "missing_token",
                    "message": "Authorization header required"
                })),
            )
                .into_response();
        }
    };

    // Extract token from "Bearer <token>"
    let token = match extract_token_from_header(auth_header) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "invalid_format",
                    "message": "Authorization header must be 'Bearer <token>'"
                })),
            )
                .into_response();
        }
    };

    // Validate token
    let claims = match validate_token(token) {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "invalid_token",
                    "message": "Invalid or expired token"
                })),
            )
                .into_response();
        }
    };

    // Convert claims to AuthUser
    let auth_user = match AuthUser::from_claims(claims) {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "invalid_claims",
                    "message": e
                })),
            )
                .into_response();
        }
    };

    // Insert AuthUser into request extensions
    request.extensions_mut().insert(auth_user);

    // Continue to next middleware/handler
    next.run(request).await
}
