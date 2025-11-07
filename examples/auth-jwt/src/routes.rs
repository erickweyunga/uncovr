use crate::middleware::auth_middleware;
use crate::user::apis::{GetUserRouter, LoginEndpoint, RegisterEndpoint, WhoAmIEndpoint};
use crate::utils::state::AppState;
use uncovr::axum_middleware::from_fn;
use uncovr::config::AppConfig;
use uncovr::server::Server;
use uncovr::tower::{Layer, layer::layer_fn};

pub async fn create_routes(state: AppState, config: AppConfig) {
    let public_routes = Server::new()
        .register(RegisterEndpoint::new(state.clone()))
        .register(LoginEndpoint::new(state.clone()))
        .build()
        .into_router();

    let private_routes = Server::new()
        .register(GetUserRouter::new(state.clone()))
        .register(WhoAmIEndpoint::new(state.clone()))
        .layer(layer_fn(|inner| from_fn(auth_middleware).layer(inner)))
        .build()
        .into_router();

    Server::new()
        .with_config(config)
        .nest("/auth", public_routes)
        .nest("/api", private_routes)
        .serve()
        .await
        .expect("Something went wrong");
}
