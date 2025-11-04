# Uncover

A modular Rust API framework built on top of Axum, designed for building type-safe, production-ready HTTP APIs with minimal boilerplate.

## Features

- 🚀 **Built on Axum** - Leverages the performance and ecosystem of Axum
- 📝 **Auto-generated OpenAPI** - Automatic API documentation with Scalar UI
- 🔒 **Type-safe** - Full compile-time type checking for requests and responses
- 🎯 **Minimal Boilerplate** - Focus on business logic, not plumbing
- 📊 **Built-in Logging** - Structured logging with tracing
- 🌐 **CORS Support** - Configurable CORS with environment-based presets
- ⚙️ **Configuration Management** - Centralized config via `meta.rs`
- 🔧 **Flexible** - Easy to customize and extend

## Quick Start

### Add to your `Cargo.toml`

```toml
[dependencies]
uncover = "0.1.0"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### Create your first endpoint

```rust
use uncover::prelude::*;
use serde::{Deserialize, Serialize};

// Define request and response types
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

// Define your endpoint
#[derive(Clone)]
pub struct CreateUserEndpoint;

impl Metadata for CreateUserEndpoint {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")
    }
}

#[async_trait]
impl API for CreateUserEndpoint {
    type Req = CreateUser;
    type Res = Json<User>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(User {
            id: 1,
            name: ctx.req.name,
            email: ctx.req.email,
        })
    }
}
```

### Configure your server (`meta.rs`)

```rust
use uncover::prelude::*;

pub fn config() -> AppConfig {
    AppConfig::new("My API", "1.0.0")
        .description("My awesome API built with Uncover")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development)
        .cors(CorsConfig::development())
        .logging(LoggingConfig::development())
        .docs(true)
        .add_server("http://localhost:3000", "Local development")
}
```

### Start your server

```rust
mod meta;

#[tokio::main]
async fn main() {
    let config = meta::config();

    uncover::server::Server::new()
        .with_config(config)
        .register(CreateUserEndpoint)
        .serve()
        .await
        .expect("Server failed");
}
```

### Run it!

```bash
cargo run
```

Your API is now running at `http://127.0.0.1:3000`!

- **Interactive API Docs**: http://127.0.0.1:3000/docs
- **OpenAPI JSON**: http://127.0.0.1:3000/openapi.json

## Configuration

### Environment-based Configuration

```rust
use uncover::prelude::*;

// Development
let dev_config = AppConfig::new("My API", "1.0.0")
    .environment(Environment::Development)
    .logging(LoggingConfig::development())  // Debug level, pretty format
    .cors(CorsConfig::development());       // Allow all origins

// Production
let prod_config = AppConfig::new("My API", "1.0.0")
    .environment(Environment::Production)
    .logging(LoggingConfig::production())   // Info level, JSON format
    .cors(CorsConfig::production(vec![
        "https://yourdomain.com".to_string()
    ]));
```

### Logging

Uncover includes built-in structured logging powered by `tracing`:

```rust
// Configure logging
.logging(LoggingConfig::development()
    .level(LogLevel::Debug)
    .log_requests(true)
    .log_responses(false)
    .format(LogFormat::Pretty))
```

**Log output:**
```
2025-11-04T18:23:57.899Z  INFO Request completed, status: 200, latency_ms: 2, method: POST, uri: /users
2025-11-04T18:23:57.899Z  INFO Server running, address: 127.0.0.1:3000
```

### CORS

```rust
// Development - allow all
.cors(CorsConfig::development())

// Production - specific origins
.cors(CorsConfig::production(vec![
    "https://example.com".to_string(),
    "https://www.example.com".to_string(),
]))

// Custom
.cors(CorsConfig {
    allowed_origins: vec!["https://myapp.com".to_string()],
    allowed_methods: vec!["GET".to_string(), "POST".to_string()],
    allowed_headers: vec!["content-type".to_string()],
    allow_credentials: true,
    max_age: Some(3600),
})
```

## HTTP Methods

Uncover supports all standard HTTP methods:

```rust
Endpoint::new("/users", "get")      // GET
Endpoint::new("/users", "post")     // POST
Endpoint::new("/users/{id}", "put") // PUT
Endpoint::new("/users/{id}", "delete") // DELETE
Endpoint::new("/users/{id}", "patch")  // PATCH
```

## Request/Response Types

All request and response types must implement the required traits:

```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Request type
#[derive(Default, Deserialize, JsonSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// Response type
#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
}
```

## OpenAPI Documentation

OpenAPI docs are automatically generated from your types and metadata:

```rust
impl Metadata for MyEndpoint {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")  // Shows in API docs
    }
}
```

Add doc comments to your types for better documentation:

```rust
#[derive(Deserialize, JsonSchema)]
pub struct CreateUser {
    /// User's full name
    #[schemars(example = "John Doe")]
    pub name: String,

    /// User's email address
    #[schemars(example = "john@example.com")]
    pub email: String,
}
```

## Project Structure

```
my-api/
├── src/
│   ├── main.rs       # Server setup and entry point
│   ├── meta.rs       # Configuration
│   └── endpoints/    # Your API endpoints
│       ├── mod.rs
│       ├── users.rs
│       └── posts.rs
├── Cargo.toml
└── README.md
```

## Advanced Usage

### Multiple Endpoints

```rust
Server::new()
    .with_config(config)
    .register(CreateUserEndpoint)
    .register(GetUserEndpoint)
    .register(ListUsersEndpoint)
    .register(UpdateUserEndpoint)
    .register(DeleteUserEndpoint)
    .serve()
    .await
```

### Custom Middleware

Uncover is built on Axum, so you can use any Axum middleware:

```rust
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;

let app = Server::new()
    .with_config(config)
    .register(MyEndpoint)
    .build();

// Add custom middleware
let app_with_timeout = app.layer(TimeoutLayer::new(Duration::from_secs(30)));
```

### Environment Variables

Override log level at runtime:

```bash
RUST_LOG=debug cargo run
RUST_LOG=my_api=trace,tower_http=info cargo run
```

## Examples

See the [`examples/`](examples/) directory for complete working examples:

- [`hello-api`](examples/hello-api/) - Basic API with user creation

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
