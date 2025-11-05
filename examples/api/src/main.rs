mod routes;

use crate::routes::{echo::Echo, hello::Hello, large::Large, ping::Ping, users::Users};
use uncovr::config::{AppConfig, LoggingConfig};

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let config = AppConfig::new("Example API", "1.0.0").logging(LoggingConfig::development());

    uncovr::server::Server::new()
        .with_config(config)
        .register(Ping)
        .register(Hello)
        .register(Echo)
        .register(Users)
        .register(Large)
        .serve()
        .await
        .expect("Server failed");
}
