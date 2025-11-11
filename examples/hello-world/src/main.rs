use std::net::SocketAddr;
use uncovr::prelude::*;

// GET /hello endpoint
#[derive(Clone)]
struct Hello;

impl Endpoint for Hello {
    fn route(&self) -> Route {
        Route::get("/hello")
    }

    fn meta(&self) -> Meta {
        Meta::new().summary("Say hello")
    }
}

#[async_trait]
impl Handler for Hello {
    type Request = ();
    type Response = &'static str;

    async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
        "Hello, World!"
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let app = App::new("Basic API", "1.0.0", addr.to_string())
        .description("A Web Service Uncovr API example");

    uncovr::server::Server::new()
        .with_config(app)
        .with_logging(Logging::development())
        .register(Hello)
        .serve()
        .await
        .expect("Server failed");
}
