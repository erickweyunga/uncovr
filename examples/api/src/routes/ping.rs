use uncovr::{
    api::API,
    prelude::{Context, async_trait},
    server::endpoint::{Docs, Endpoint, Route},
};

#[derive(Default, Clone)]
pub struct Ping;

impl Endpoint for Ping {
    fn ep(&self) -> Route {
        Route::GET("/ping")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Ping the server")
                .description("Simple health check endpoint")
                .tag("health"),
        )
    }
}

#[async_trait]
impl API for Ping {
    type Req = ();
    type Res = &'static str;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        "pong"
    }
}
