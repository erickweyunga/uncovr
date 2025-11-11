# Uncovr

Uncovr is a modular, type-safe, and developer-friendly backend framework for Rust

[![Crates.io](https://img.shields.io/crates/v/uncovr.svg)](https://crates.io/crates/uncovr)
[![Documentation](https://docs.rs/uncovr/badge.svg)](https://docs.rs/uncovr)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Features

- Type-safe endpoint definitions with compile-time validation
- Automatic OpenAPI 3.0 documentation with interactive Scalar UI
- Modular architecture with composable endpoints and middleware
- Built-in middleware (CORS, rate limiting, authentication, request ID)
- Structured logging with development and production modes
- Full Tokio compatibility
- Request validation with custom error types
- Path and query parameter extraction
- Application state management
- Environment-based configuration
- Integrates with Axum and Tower ecosystems
- Runs on stable Rust 1.85+

## Documentation

- [User Guide](https://docs.rs/uncovr)
- [API Documentation](https://docs.rs/uncovr)
- [Examples Repository](https://github.com/erickweyunga/uncovr/tree/main/examples)

## Example

**Dependencies:**

```toml
[dependencies]
uncovr = "0.2"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

**Code:**

```rust
use uncovr::prelude::*;

#[derive(Clone)]
struct Hello;

impl Endpoint for Hello {
    fn route(&self) -> Route {
        Route::get("/hello/:name")
            .param("name", "Person's name")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Say hello")
            .tag("greetings")
    }
}

#[async_trait]
impl Handler for Hello {
    type Request = ();
    type Response = String;

    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
        let name = ctx.path.get("name").unwrap_or("World");
        format!("Hello, {}!", name)
    }
}

#[tokio::main]
async fn main() {
    let config = App::new("Hello API", "1.0.0");

    Server::new()
        .with_config(config)
        .register(Hello)
        .serve()
        .await
        .unwrap();
}
```

## More Examples

- [Hello World](https://github.com/erickweyunga/uncovr/tree/main/examples/hello-world)
- [JSON API with CRUD](https://github.com/erickweyunga/uncovr/tree/main/examples/api)
- [Path & Query Parameters](https://github.com/erickweyunga/uncovr/tree/main/examples/routes)
- [Authentication with JWT](https://github.com/erickweyunga/uncovr/tree/main/examples/auth-jwt)
- [URL Shortener](https://github.com/erickweyunga/uncovr/tree/main/examples/url-shortner)

You may consider checking out [the examples directory](https://github.com/erickweyunga/uncovr/tree/main/examples) for more examples.

## License

This project is licensed under the [MIT license](LICENSE).