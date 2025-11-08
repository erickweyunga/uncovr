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
- **Separation of Concerns** - Route definition separate from documentation
- **Modular Design** - Independent, testable endpoint modules
- **Built-in Middleware** - Logging, CORS, compression, tracing
- **Environment Config** - Development and production presets
- **Minimal Boilerplate** - Focus on business logic, not framework code

## Quick Start

```rust
use uncovr::prelude::*;
use uncovr::server::endpoint::{Endpoint, Route, Docs};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CreateUser;

#[derive(Default, Deserialize, JsonSchema)]
pub struct UserRequest {
    name: String,
    email: String,
}

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    id: u64,
    name: String,
    email: String,
}

impl Endpoint for CreateUser {
    fn ep(&self) -> Route {
        Route::POST("/users")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Create a new user")
                .description("Creates a user account with the provided information")
                .tag("users")
                .responses(|op| {
                    op.response::<200, Json<UserResponse>>()
                      .response::<400, Json<ErrorResponse>>()
                })
        )
    }
}

#[async_trait]
impl API for CreateUser {
    type Req = UserRequest;
    type Res = Json<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(UserResponse {
            id: 1,
            name: ctx.req.name,
            email: ctx.req.email,
        })
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::new("My API", "1.0.0")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(CreateUser)
        .serve()
        .await
        .expect("Server failed to start");
}
```

## Endpoint Definition

Uncovr uses a clean separation of concerns for endpoint definition:

### 1. Route Definition (`ep()`)

Define the HTTP method, path, and parameters:

```rust
fn ep(&self) -> Route {
    Route::POST("/users/:id")
        .path_param("id").desc("User ID")
        .query("notify").desc("Send notification")
}
```

**Available HTTP Methods:**
- `Route::GET(path)` - GET requests
- `Route::POST(path)` - POST requests  
- `Route::PUT(path)` - PUT requests
- `Route::PATCH(path)` - PATCH requests
- `Route::DELETE(path)` - DELETE requests
- `Route::OPTIONS(path)` - OPTIONS requests
- `Route::HEAD(path)` - HEAD requests

### 2. Documentation (`docs()`)

Optional API documentation for OpenAPI:

```rust
fn docs(&self) -> Option<Docs> {
    Some(
        Docs::new()
            .summary("Create user")
            .description("Creates a new user account")
            .tag("users")
            .tag("authentication")
            .responses(|op| {
                op.response::<200, Json<UserResponse>>()
                  .response::<400, Json<ErrorResponse>>()
                  .response::<500, Json<ErrorResponse>>()
            })
    )
}
```

### 3. Handler (`handler()`)

The business logic:

```rust
#[async_trait]
impl API for CreateUser {
    type Req = UserRequest;
    type Res = Json<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Your logic here
        Json(UserResponse { /* ... */ })
    }
}
```

## Configuration

Uncovr uses environment-based configuration to make it easy to switch between development and production settings:

```rust
use uncovr::prelude::*;

pub fn config() -> AppConfig {
    AppConfig::new("My API", "1.0.0")
        .bind("127.0.0.1:3000")
        .description("My awesome API")
        .environment(Environment::Development)
        .cors(CorsConfig::development())
        .logging(LoggingConfig::development())
        .docs(true)
}
```

### Environment Presets

**Development:**
```rust
AppConfig::new("My API", "1.0.0")
    .environment(Environment::Development)
    .logging(LoggingConfig::development()) // Verbose, pretty logs
    .cors(CorsConfig::development())       // Permissive CORS
```

**Production:**
```rust
AppConfig::new("My API", "1.0.0")
    .environment(Environment::Production)
    .logging(LoggingConfig::production())  // JSON logs, info level
    .cors(CorsConfig::production(vec![     // Restricted CORS
        "https://yourdomain.com".to_string()
    ]))
```

## Interactive Documentation

Once your server is running, visit `http://localhost:3000/docs` for interactive API documentation powered by Scalar.

The documentation is automatically generated from your types and endpoint definitions!

## Examples

The [examples](examples/) folder contains various real-world examples:

- **[api](examples/api/)** - Basic API with multiple endpoints
- **[auth-jwt](examples/auth-jwt/)** - JWT authentication with middleware
- **[routes](examples/routes/)** - Path and query parameter examples
- **[url-shortner](examples/url-shortner/)** - Complete URL shortener service

Each example demonstrates different features and patterns.

## Why Separate `ep()` and `docs()`?

The separation of route definition and documentation provides several benefits:

1. **Cleaner Code** - Each method has a single responsibility
2. **Optional Docs** - Skip documentation during rapid prototyping
3. **Better Organization** - Routing logic separate from API docs
4. **Type Safety** - `Route::POST()` instead of `"post"` strings
5. **Extensibility** - Easy to add more endpoint metadata in the future

## Performance

Uncovr is built on Axum and Tokio, providing excellent performance:

- **~23,000 requests/sec** for typical JSON endpoints
- **Sub-5ms latency** for most operations
- Efficient async I/O with Tokio runtime
- Zero-cost abstractions

Run benchmarks with:
```bash
cargo bench
```

## Getting Help

You're welcome to:
- Open a [discussion](https://github.com/erickweyunga/uncovr/discussions) with your question
- Create an [issue](https://github.com/erickweyunga/uncovr/issues) if you encounter problems
- Check out the [examples](examples/) for common patterns

## Contributing

Thanks for your help improving the project! We have a [contributing guide](CONTRIBUTING.md) to help you get involved in the Uncovr project.

## License

This project is licensed under the [MIT license](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Uncovr by you, shall be licensed as MIT, without any additional terms or conditions.