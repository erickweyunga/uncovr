use serde::Serialize;
use uncovr::prelude::*;

#[derive(Clone)]
pub struct Hello;

#[derive(Serialize, JsonSchema)]
pub struct HelloResponse {
    message: String,
}

impl Metadata for Hello {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/hello", "get").summary("Dynamic hello")
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
