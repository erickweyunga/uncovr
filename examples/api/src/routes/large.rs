use serde::{Deserialize, Serialize};
use uncovr::prelude::*;
use uncovr::server::endpoint::{Docs, Endpoint, Route};

#[derive(Clone)]
pub struct Large;

#[derive(Default, Deserialize, JsonSchema)]
pub struct LargeRequest {
    data: Vec<u8>,
}

#[derive(Serialize, JsonSchema)]
pub struct LargeResponse {
    size: usize,
}

impl Endpoint for Large {
    fn ep(&self) -> Route {
        Route::POST("/large")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Handle large payload")
                .description("Accepts and processes large binary payloads")
                .tag("utilities"),
        )
    }
}

#[async_trait]
impl API for Large {
    type Req = LargeRequest;
    type Res = Json<LargeResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(LargeResponse {
            size: ctx.req.data.len(),
        })
    }
}
