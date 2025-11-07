use std::env;

use uncovr::config::{AppConfig, Environment};

use crate::routes::create_routes;

mod middleware;
mod routes;
mod user;
mod utils;

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap_or("sqlite://users.db".to_string());

    let pool = utils::db::connect_db(&database_url)
        .await
        .expect("Failed to connect to database");
    utils::db::migrate(&pool)
        .await
        .expect("Failed to run migrations");

    let state = utils::state::AppState::new(pool);

    let config = AppConfig::new("Authentication API", "0.1.0")
        .bind("0.0.0.0:8000")
        .environment(Environment::Development);

    create_routes(state.clone(), config.clone()).await
}
