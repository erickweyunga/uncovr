use serde::{Deserialize, Serialize};
use uncovr::prelude::*;

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

impl Metadata for Large {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/large", "post").summary("Handle large payload")
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
