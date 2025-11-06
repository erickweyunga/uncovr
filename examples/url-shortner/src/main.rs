use uncovr::{prelude::*, server::Server};

use crate::url::apis::UrlApi;

mod fun;
mod url;

#[tokio::main]
async fn main() {
    let config = AppConfig::new("URL SHORTENER API", "0.1.0")
        .bind("0.0.0.0:8000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(UrlApi)
        .serve()
        .await
        .expect("Something went wrong while starting Url Shortner Server")
}
