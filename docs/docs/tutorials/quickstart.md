# API Dev Quickstart

Get up and running with Uncovr in minutes.

## Prerequisites

Before you begin, make sure you have:

- Rust installed (1.70 or later)
- Cargo (comes with Rust)
- A text editor or IDE

Install Rust from [rustup.rs](https://rustup.rs/) if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Create a New Project

Create a new Rust project:

```bash
cargo new my-api
cd my-api
```

## Install Uncovr

Add Uncovr to your `Cargo.toml`:

```toml
[dependencies]
uncovr = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

## Write Your First Endpoint

Replace the contents of `src/main.rs` with:

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct HelloWorld;

impl Metadata for HelloWorld {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/", "get")
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

## Run Your API

Start the server:

```bash
cargo run
```

You should see output like:

```
Compiling my-api v0.1.0
Finished dev [unoptimized + debuginfo] target(s) in 2.34s
Running `target/debug/my-api`
Server running at http://127.0.0.1:3000
```

## Test Your API

Open another terminal and test your endpoint:

```bash
curl http://127.0.0.1:3000/
```

You should see:

```
Hello, World!
```

## View API Documentation

Uncovr automatically generates interactive API documentation. Open your browser and visit:

```
http://127.0.0.1:3000/docs
```

You'll see an interactive UI where you can explore your API, view request/response schemas, and test endpoints directly.

## Add a POST Endpoint

Let's add a more interesting endpoint that accepts JSON. Add this to your `main.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, JsonSchema)]
pub struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize, JsonSchema)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Clone)]
pub struct CreateUserEndpoint;

impl Metadata for CreateUserEndpoint {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")
            .description("Creates a new user with the provided name and email")
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

Update your main function to register the new endpoint:

```rust
#[tokio::main]
async fn main() {
    let config = AppConfig::new("Hello API", "1.0.0")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(HelloWorld)
        .register(CreateUserEndpoint)
        .serve()
        .await
        .expect("Server failed to start");
}
```

Restart your server (Ctrl+C and `cargo run` again).

Test the new endpoint:

```bash
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

Response:

```json
{"id":1,"name":"Alice","email":"alice@example.com"}
```

Check the updated documentation at `http://127.0.0.1:3000/docs` to see your new endpoint.

## Project Structure

For larger projects, organize your code like this:

```
my-api/
├── Cargo.toml
├── src/
│   ├── main.rs          # Server setup
│   ├── config.rs        # Configuration
│   └── endpoints/       # Your endpoints
│       ├── mod.rs
│       ├── hello.rs
│       └── users.rs
```

Example `src/endpoints/hello.rs`:

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct HelloWorld;

impl Metadata for HelloWorld {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/", "get")
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
```

Example `src/endpoints/mod.rs`:

```rust
pub mod hello;
pub mod users;

pub use hello::HelloWorld;
pub use users::CreateUserEndpoint;
```

Example `src/main.rs`:

```rust
use uncovr::prelude::*;

mod endpoints;

use endpoints::{HelloWorld, CreateUserEndpoint};

#[tokio::main]
async fn main() {
    let config = AppConfig::new("My API", "1.0.0")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(HelloWorld)
        .register(CreateUserEndpoint)
        .serve()
        .await
        .expect("Server failed");
}
```

## Configuration

Create a separate `src/config.rs` for your configuration:

```rust
use uncovr::prelude::*;

pub fn development() -> AppConfig {
    AppConfig::new("My API", "1.0.0")
        .description("My awesome API")
        .bind("127.0.0.1:3000")
        .environment(Environment::Development)
        .logging(LoggingConfig::development())
        .cors(CorsConfig::development())
        .docs(true)
}

pub fn production() -> AppConfig {
    AppConfig::new("My API", "1.0.0")
        .description("My awesome API")
        .bind("0.0.0.0:8080")
        .environment(Environment::Production)
        .logging(LoggingConfig::production())
        .cors(CorsConfig::production(vec![
            "https://myapp.com".to_string()
        ]))
        .docs(false)
}
```

Use it in `main.rs`:

```rust
mod config;

#[tokio::main]
async fn main() {
    Server::new()
        .with_config(config::development())
        .register(HelloWorld)
        .serve()
        .await
        .expect("Server failed");
}
```

## Development Tips

### Hot Reload

Install `cargo-watch` for automatic recompilation:

```bash
cargo install cargo-watch
```

Run with:

```bash
cargo watch -x run
```

### Pretty Logging

In development, enable pretty logging:

```rust
.logging(LoggingConfig::development())
```

### CORS for Frontend Development

Allow your frontend to access the API:

```rust
.cors(CorsConfig::development())  // Allows all origins
```

## Next Steps

You now have a working Uncovr API! Continue learning:

- [Uncovr By Example](/tutorials/by-example) - More practical examples
- [Routes](/explanations/routes) - Understand routing in depth
- [GitHub Examples](https://github.com/erickweyunga/uncovr/tree/main/examples) - Complete example projects

## Common Issues

### Port Already in Use

If port 3000 is in use, change the bind address:

```rust
.bind("127.0.0.1:8080")
```

### Compilation Errors

Make sure you have the required derives:

```rust
// Request types need:
#[derive(Default, Deserialize, JsonSchema)]

// Response types need:
#[derive(Serialize, JsonSchema)]
```

### Missing JsonSchema

Add schemars to your dependencies:

```toml
[dependencies]
schemars = "0.8"
```

Then derive it:

```rust
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct MyType { }
```
