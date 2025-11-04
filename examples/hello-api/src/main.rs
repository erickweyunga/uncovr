//! Minimal Hello World API example
//!
//! This example demonstrates the simplest possible Uncover API:
//! - Single GET endpoint returning "Hello, World!"
//! - Minimal configuration
//! - No request/response types needed

use uncover::prelude::*;

#[derive(Clone)]
pub struct HelloWorld;

impl Metadata for HelloWorld {
    fn metadata(&self) -> EndpointMetadata {
        EndpointMetadata::new("/", "get").summary("Say hello")
    }
}

#[async_trait]
impl API for HelloWorld {
    type Req = ();
    type Res = &'static str;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        "Hello, World!"
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::new("Hello API", "1.0.0").logging(LoggingConfig::development());

    uncover::server::Server::new()
        .with_config(config)
        .register(HelloWorld)
        .serve()
        .await
        .expect("Server failed");
}
