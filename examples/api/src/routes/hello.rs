use serde::Serialize;
use uncovr::prelude::*;
use uncovr::server::endpoint::{Docs, Endpoint, Route};

#[derive(Clone)]
pub struct Hello;

#[derive(Serialize, JsonSchema)]
pub struct HelloResponse {
    message: String,
}

impl Endpoint for Hello {
    fn ep(&self) -> Route {
        Route::GET("/hello")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Say hello")
                .description("Returns a friendly greeting message")
                .tag("greetings"),
        )
    }
}

#[async_trait]
impl API for Hello {
    type Req = ();
    type Res = Json<HelloResponse>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        Json(HelloResponse {
            message: "Hello, World!".into(),
        })
    }
}
