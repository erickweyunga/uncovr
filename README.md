# Uncover

Uncover is a web framework that focuses on type safety and automatic API documentation.

More information about this crate can be found in the [crate documentation](https://docs.rs/uncover).

## High level features

- Type-safe request and response handling with compile-time validation.
- Automatic OpenAPI documentation generation with interactive UI.
- Built-in structured logging and request tracing.
- Environment-based configuration presets for development and production.
- Built on top of Axum and Tower, giving you access to the entire ecosystem.

In particular the last point is what makes Uncover flexible. Since it's built on Axum and Tower, you get access to middleware for timeouts, tracing, compression, authorization, and more. It also enables you to share middleware with applications written using Axum, Hyper, or Tonic.

## Usage example

```rust
use uncover::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct HelloWorld;

impl Metadata for HelloWorld {
    fn metadata(&self) -> EndpointMetadata {
        EndpointMetadata::new("/", "get")
            .summary("Say hello")
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
    let config = AppConfig::new("Hello API", "1.0.0")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(HelloWorld)
        .serve()
        .await
        .expect("Server failed to start");
}
```

You can find this example as well as other example projects in the [examples directory](examples/).

## Configuration

Uncover uses environment-based configuration to make it easy to switch between development and production settings:

```rust
use uncover::prelude::*;

pub fn config() -> AppConfig {
    AppConfig::new("My API", "1.0.0")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development)
        .cors(CorsConfig::development())
        .logging(LoggingConfig::development())
        .docs(true)
}
```

## Examples

The [examples](examples/) folder contains various examples of how to use Uncover. The docs also provide code snippets and examples.

## Getting Help

You're welcome to open a [discussion](https://github.com/erickweyunga/uncover/discussions) with your question or create an [issue](https://github.com/erickweyunga/uncover/issues) if you encounter problems.

## Contributing

Thanks for your help improving the project! We have a [contributing guide](CONTRIBUTING.md) to help you get involved in the Uncover project.

## License

This project is licensed under the [MIT license](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Uncover by you, shall be licensed as MIT, without any additional terms or conditions.
