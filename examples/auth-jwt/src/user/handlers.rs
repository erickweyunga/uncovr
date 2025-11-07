use uncovr::prelude::*;

use crate::middleware::AuthUser;
use crate::utils::db::{CreateUser, TokenResponse, UserResponse};
use crate::utils::jwt::generate_token;
use crate::utils::password::{hash_password, verify_password};
use crate::utils::state::AppState;

/// Handler for getting a user by ID
pub async fn get_user_by_id(state: &AppState, id: i64) -> ApiResponse<UserResponse> {
    let result = sqlx::query_as!(
        crate::utils::db::UserRecord,
        r#"SELECT id as "id!", email, password, created_at, updated_at FROM users WHERE id = ?"#,
        id
    )
    .fetch_optional(&state.db_pool)
    .await;

    match result {
        Ok(Some(user)) => {
            let response = UserResponse {
                id: user.id,
                email: user.email,
                created_at: user.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: user.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
            };
            ApiResponse::Ok(response)
        }
        Ok(None) => ApiResponse::NotFound {
            code: "user_not_found",
            message: "User with the specified ID does not exist".to_string(),
        },
        Err(_e) => ApiResponse::InternalError {
            code: "database_error",
            message: "Failed to query database".to_string(),
        },
    }
}

/// Handler for user registration
pub async fn register_user(state: &AppState, payload: CreateUser) -> ApiResponse<UserResponse> {
    // Check if user already exists
    let existing = sqlx::query!(r#"SELECT id FROM users WHERE email = ?"#, payload.email)
        .fetch_optional(&state.db_pool)
        .await;

    match existing {
        Ok(Some(_)) => {
            return ApiResponse::Conflict {
                code: "user_exists",
                message: "A user with this email already exists".to_string(),
            };
        }
        Err(_) => {
            return ApiResponse::InternalError {
                code: "database_error",
                message: "Failed to check existing user".to_string(),
            };
        }
        _ => {}
    }

    let hashed_pwd = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => {
            return ApiResponse::InternalError {
                code: "hashing_error",
                message: "Failed to hash password".to_string(),
            };
        }
    };

    // Insert the new user
    let result = sqlx::query!(
        r#"
            INSERT INTO users (email, password)
            VALUES (?, ?)
        "#,
        payload.email,
        hashed_pwd
    )
    .execute(&state.db_pool)
    .await;

    match result {
        Ok(result) => {
            let user_id = result.last_insert_rowid();

            // Fetch the created user
            let user = sqlx::query_as!(
                crate::utils::db::UserRecord,
                r#"SELECT id as "id!", email, password, created_at, updated_at FROM users WHERE id = ?"#,
                user_id
            )
            .fetch_one(&state.db_pool)
            .await;

            match user {
                Ok(user) => {
                    let response = UserResponse {
                        id: user.id,
                        email: user.email,
                        created_at: user.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                        updated_at: user.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
                    };
                    ApiResponse::Created(response)
                }
                Err(_) => ApiResponse::InternalError {
                    code: "database_error",
                    message: "Failed to fetch created user".to_string(),
                },
            }
        }
        Err(_) => ApiResponse::InternalError {
            code: "database_error",
            message: "Failed to create user".to_string(),
        },
    }
}

/// Handler for user login
pub async fn login_user(state: &AppState, payload: CreateUser) -> ApiResponse<TokenResponse> {
    // Fetch user by email
    let result = sqlx::query_as!(
        crate::utils::db::UserRecord,
        r#"SELECT id as "id!", email, password, created_at, updated_at FROM users WHERE email = ?"#,
        payload.email
    )
    .fetch_optional(&state.db_pool)
    .await;

    match result {
        Ok(Some(user)) => {
            let verify_password =
                matches!(verify_password(&payload.password, &user.password), Ok(true));

            if !verify_password {
                return ApiResponse::Unauthorized {
                    code: "invalid_credentials",
                    message: "Invalid email or password".to_string(),
                };
            }

            let token = match generate_token(user.id, &user.email) {
                Ok(token) => token,
                Err(err) => {
                    return ApiResponse::InternalError {
                        code: "token_generation_error",
                        message: format!("Failed to generate token: {}", err),
                    };
                }
            };

            let response = TokenResponse { token };
            ApiResponse::Ok(response)
        }
        Ok(None) => ApiResponse::Unauthorized {
            code: "invalid_credentials",
            message: "Invalid email or password".to_string(),
        },
        Err(_) => ApiResponse::InternalError {
            code: "database_error",
            message: "Failed to query database".to_string(),
        },
    }
}

/// Handler for whoami (get current authenticated user)
pub async fn whoami(state: &AppState, user: AuthUser) -> ApiResponse<UserResponse> {
    // Fetch the user by ID from the authenticated user's information
    get_user_by_id(state, user.user_id).await
}
