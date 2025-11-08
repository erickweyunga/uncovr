use serde::{Deserialize, Serialize};
use tokio::time::{Duration, sleep};
use uncovr::prelude::*;
use uncovr::server::endpoint::{Docs, Endpoint, Route};

#[derive(Clone)]
pub struct Users;

#[derive(Default, Deserialize, Serialize, JsonSchema)]
pub struct CreateUser {
    name: String,
    email: String,
}

#[derive(Default, Serialize, Deserialize, JsonSchema)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

impl Endpoint for Users {
    fn ep(&self) -> Route {
        Route::POST("/users")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Create a new user")
                .description("Creates a new user account with the provided name and email")
                .tag("users"),
        )
    }
}

#[async_trait]
impl API for Users {
    type Req = CreateUser;
    type Res = Json<User>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Simulate async DB delay
        sleep(Duration::from_millis(5)).await;

        Json(User {
            id: 1,
            name: ctx.req.name.clone(),
            email: ctx.req.email.clone(),
        })
    }
}
