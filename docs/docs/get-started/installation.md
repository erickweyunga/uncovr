# Installation

Get Uncovr up and running in minutes.

## Prerequisites

- Rust 1.70 or later
- Cargo (included with Rust)

Install Rust from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Create a New Project

```bash
cargo new my-api
cd my-api
```

## Add Dependencies

Add Uncovr to your project `my-api`:

```bash
cargo add uncovr@0.1.1 tokio --features full serde --features derive
```

## Create Your First Endpoint

Replace `src/main.rs` with:

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct HelloWorld;

impl Metadata for HelloWorld {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/", "get").summary("Say hello")
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
        .expect("Server failed");
}
```

## Run Your API

```bash
cargo run
```

Visit `http://127.0.0.1:3000` to see your API in action.

View auto-generated docs at `http://127.0.0.1:3000/docs`.
