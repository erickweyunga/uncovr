# Hello API Example

The simplest possible Uncovr API - returns "Hello, World!" from a single GET endpoint.

## Getting Started

```bash
# Clone the repository
git clone https://github.com/erickweyunga/uncovr.git
cd uncover/examples/api

# Run the example
cargo run
```

Server starts at `http://127.0.0.1:3000`

## Try It

```bash
# Get hello message
curl http://localhost:3000/

# View API documentation
open http://localhost:3000/docs
```

## What This Example Shows

- Minimal endpoint definition
- Basic configuration setup
- Simple text response (no JSON needed)

## Code Overview

```rust
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
```

That's it! No complex types, no database, just a simple endpoint.
