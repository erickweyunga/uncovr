use serde::{Deserialize, Serialize};
use uncovr::prelude::*;
use uncovr::server::endpoint::{Docs, Endpoint, Route};

#[derive(Clone)]
pub struct Echo;

#[derive(Default, Deserialize, JsonSchema)]
pub struct EchoRequest {
    text: String,
}

#[derive(Serialize, JsonSchema)]
pub struct EchoResponse {
    echo: String,
}

impl Endpoint for Echo {
    fn ep(&self) -> Route {
        Route::POST("/echo")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Echo text back")
                .description("Echoes back the text sent in the request")
                .tag("utilities"),
        )
    }
}

#[async_trait]
impl API for Echo {
    type Req = EchoRequest;
    type Res = Json<EchoResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(EchoResponse {
            echo: ctx.req.text.clone(),
        })
    }
}
