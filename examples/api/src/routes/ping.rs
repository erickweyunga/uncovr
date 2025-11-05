use uncovr::{
    api::API,
    prelude::{Context, async_trait},
    server::{Endpoint, Metadata},
};

#[derive(Default, Clone)]
pub struct Ping;

impl Metadata for Ping {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/ping", "get").summary("Ping the server")
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
