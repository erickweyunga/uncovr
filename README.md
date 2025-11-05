# Uncovr

A modular microbackend framework for building type-safe REST APIs with automatic documentation and minimal boilerplate.

Uncovr enables you to build composable, self-contained API modules (microbackends) where each endpoint is an independent, testable unit. Perfect for building scalable APIs, microservices, and modular backend systems.

More information about this crate can be found in the [crate documentation](https://docs.rs/uncovr).

## Why Uncovr?

**Microbackend Architecture** - Build APIs as a collection of independent, composable modules. Each endpoint is self-contained with its own types, validation, and documentation.

**Type-Safe from the Ground Up** - Full compile-time validation of requests and responses. Catch errors before runtime.

**Zero-Config Documentation** - OpenAPI documentation generated automatically from your types. Interactive UI included out of the box.

**Modular by Design** - Each endpoint is an independent module that can be developed, tested, and maintained separately.

**Production-Ready** - Built-in logging, CORS, error handling, and environment-based configuration.

## Key Features

- **Microbackend Architecture** - Composable, self-contained API modules
- **Auto-generated OpenAPI Docs** - Interactive Scalar UI included
- **Type-Safe Endpoints** - Compile-time request/response validation
- **Modular Design** - Independent, testable endpoint modules
- **Built-in Middleware** - Logging, CORS, compression, tracing
- **Environment Config** - Development and production presets
- **Minimal Boilerplate** - Focus on business logic, not framework code

## Usage example

```rust
use uncovr::prelude::*;
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

Uncovr uses environment-based configuration to make it easy to switch between development and production settings:

```rust
use uncovr::prelude::*;

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

The [examples](examples/) folder contains various examples of how to use Uncovr. The docs also provide code snippets and examples.

## Getting Help

You're welcome to open a [discussion](https://github.com/erickweyunga/uncovr/discussions) with your question or create an [issue](https://github.com/erickweyunga/uncovr/issues) if you encounter problems.

## Contributing

Thanks for your help improving the project! We have a [contributing guide](CONTRIBUTING.md) to help you get involved in the Uncovr project.

## License

This project is licensed under the [MIT license](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Uncovr by you, shall be licensed as MIT, without any additional terms or conditions.
