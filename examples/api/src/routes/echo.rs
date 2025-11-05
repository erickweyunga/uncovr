use serde::{Deserialize, Serialize};
use uncovr::prelude::*;

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

impl Metadata for Echo {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/echo", "post").summary("Echoes back request")
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
