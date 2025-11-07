use uncovr::prelude::*;

use super::handlers;
use crate::middleware::AuthUser;
use crate::utils::db::{CreateUser, TokenResponse, UserResponse};
use crate::utils::state::AppState;

// ============================================================================
// Get User by ID Endpoint
// ============================================================================

#[derive(Clone)]
pub struct GetUserRouter {
    pub state: AppState,
}

impl GetUserRouter {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Metadata for GetUserRouter {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/user/:id", "get")
            .summary("Get user by id")
            .path_param("id")
            .desc("User ID")
            .with_responses(|op| {
                op.response::<200, Json<UserResponse>>()
                    .response::<400, Json<ErrorResponse>>()
                    .response::<404, Json<ErrorResponse>>()
                    .response::<500, Json<ErrorResponse>>()
            })
    }
}

#[async_trait]
impl API for GetUserRouter {
    type Req = ();
    type Res = ApiResponse<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let id = match ctx.path.get("id").and_then(|s| s.parse::<i64>().ok()) {
            Some(id) => id,
            None => {
                return ApiResponse::BadRequest {
                    code: "invalid_id",
                    message: "Invalid user ID format".to_string(),
                };
            }
        };

        handlers::get_user_by_id(&self.state, id).await
    }
}

// ============================================================================
// Register Endpoint
// ============================================================================

#[derive(Clone)]
pub struct RegisterEndpoint {
    pub state: AppState,
}

impl RegisterEndpoint {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Metadata for RegisterEndpoint {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/register", "post")
            .summary("Register a new user")
            .description("Create a new user account with email and password")
            .with_responses(|op| {
                op.response::<201, Json<UserResponse>>()
                    .response::<409, Json<ErrorResponse>>()
                    .response::<500, Json<ErrorResponse>>()
            })
    }
}

#[async_trait]
impl API for RegisterEndpoint {
    type Req = CreateUser;
    type Res = ApiResponse<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        handlers::register_user(&self.state, ctx.req).await
    }
}

// ============================================================================
// Login Endpoint
// ============================================================================

#[derive(Clone)]
pub struct LoginEndpoint {
    pub state: AppState,
}

impl LoginEndpoint {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Metadata for LoginEndpoint {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/login", "post")
            .summary("Login user")
            .description("Authenticate user with email and password")
            .with_responses(|op| {
                op.response::<200, Json<TokenResponse>>()
                    .response::<401, Json<ErrorResponse>>()
                    .response::<500, Json<ErrorResponse>>()
            })
    }
}

#[async_trait]
impl API for LoginEndpoint {
    type Req = CreateUser;
    type Res = ApiResponse<TokenResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        handlers::login_user(&self.state, ctx.req).await
    }
}

// ============================================================================
// WhoAmI Endpoint
// ============================================================================

#[derive(Clone)]
pub struct WhoAmIEndpoint {
    pub state: AppState,
}

impl WhoAmIEndpoint {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Metadata for WhoAmIEndpoint {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/whoami", "get")
            .summary("Get current user")
            .description("Get the currently authenticated user's information")
            .query("user_id")
            .desc("User ID (temporary - will be replaced with JWT token)")
            .with_responses(|op| {
                op.response::<200, Json<UserResponse>>()
                    .response::<401, Json<ErrorResponse>>()
                    .response::<404, Json<ErrorResponse>>()
                    .response::<500, Json<ErrorResponse>>()
            })
    }
}

#[async_trait]
impl API for WhoAmIEndpoint {
    type Req = ();
    type Res = ApiResponse<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let user = match ctx.extensions.get::<AuthUser>() {
            Some(user) => user.clone(),
            None => {
                return ApiResponse::Unauthorized {
                    code: "not_authenticated",
                    message: "User not authenticated".to_string(),
                };
            }
        };

        handlers::whoami(&self.state, user).await
    }
}
