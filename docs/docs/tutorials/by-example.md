# Uncovr By Example

Learn Uncovr by building a complete URL shortener application from scratch.

::: tip Full Source Code
The complete example is available on GitHub: [url-shortener example](https://github.com/erickweyunga/uncovr/tree/main/examples/url-shortner)
:::

## What We're Building

A URL shortener service that:
- Takes long URLs and creates short codes
- Redirects short codes to original URLs
- Validates input and handles errors properly
- Provides OpenAPI documentation

By the end, you'll understand how to structure real Uncovr applications.

## Project Setup

Create a new project and add dependencies:

```bash
cargo new url-shortener
cd url-shortener
cargo add uncovr tokio serde --features derive
cargo add nanoid once_cell
```

## Project Structure

We'll organize our code following Uncovr's recommended pattern:

```
src/
├── main.rs          # Server setup
├── fun.rs           # Helper functions
└── url/             # URL feature module
    ├── mod.rs       # Module exports
    ├── apis.rs      # API definitions
    └── handlers.rs  # Business logic
```

This separates concerns: definitions in `apis.rs`, implementation in `handlers.rs`.

## Step 1: Helper Functions

First, create `src/fun.rs` with our URL storage logic:

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// In-memory storage (use a database in production)
static URL_MAP: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn shorten_url(base_url: &str, original_url: &str) -> String {
    let short_code = nanoid::nanoid!(6);

    URL_MAP
        .lock()
        .unwrap()
        .insert(short_code.clone(), original_url.to_string());

    format!("{}/{}", base_url, short_code)
}

pub fn get_original_url(short_code: &str) -> Option<String> {
    URL_MAP
        .lock()
        .unwrap()
        .get(short_code)
        .cloned()
}
```

::: info Why This Structure?
We use a global HashMap for simplicity. In production, replace this with a database like PostgreSQL or Redis.
:::

## Step 2: API Definitions

Create `src/url/apis.rs` to define our API surface:

```rust
use uncovr::prelude::*;
use uncovr::server::endpoint::{Endpoint, Route, Docs};

// Request type for shortening URLs
#[derive(Default, Deserialize, JsonSchema)]
pub struct UrlRequest {
    /// The long URL to be shortened
    pub url: String,
}

// Response type with the shortened URL
#[derive(Serialize, JsonSchema)]
pub struct UrlResponse {
    /// The short URL generated for the provided long URL
    pub short_url: String,
}

// Empty response type for redirects
#[derive(Serialize, JsonSchema)]
pub struct Redirect;

// API endpoint for shortening URLs
#[derive(Clone)]
pub struct ShortenUrlApi;

// API endpoint for redirecting to original URLs
#[derive(Clone)]
pub struct RedirectUrlApi;

impl Endpoint for ShortenUrlApi {
    fn ep(&self) -> Route {
        Route::POST("/url")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Shorten a URL")
                .description("Takes a long URL and returns a shortened version")
                .tag("urls")
                .responses(|op| {
                    op.response::<200, Json<UrlResponse>>()
                        .response::<400, Json<ErrorResponse>>()
                        .response::<500, Json<ErrorResponse>>()
                })
        )
    }
}

impl Endpoint for RedirectUrlApi {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/:id");
        route.path_param("id").desc("The short URL identifier");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Redirect to original URL")
                .description("Redirects to the original URL associated with the given short URL ID")
                .tag("urls")
                .responses(|op| {
                    op.response::<301, Json<Redirect>>()
                        .response::<404, Json<ErrorResponse>>()
                })
        )
    }
}
```

::: details Understanding the Code

**Request/Response Types**: Define the shape of data flowing through your API. These automatically generate OpenAPI schemas.

**API Structs**: Each endpoint is a struct. This keeps endpoints independent and testable.

**New Endpoint Trait**: The endpoint definition is split into two parts:

1. **`ep()` - Route Definition**: Technical routing information
   - HTTP method via `Route::POST()`, `Route::GET()`, etc.
   - Path with parameters
   - Query and path parameter definitions

2. **`docs()` - Documentation**: Human-readable API documentation
   - Summary and description
   - Tags for grouping endpoints
   - Response status codes via `responses()`

This separation makes your code cleaner and documentation optional for rapid prototyping.

**Type-Safe HTTP Methods**: Notice we use `Route::POST()` and `Route::GET()` instead of strings. This prevents typos at compile time!

**Response Documentation**: The `responses()` method documents what your API returns, making your OpenAPI docs complete and accurate.
:::

## Step 3: Business Logic

Create `src/url/handlers.rs` with the actual implementation:

```rust
use crate::{
    fun,
    url::apis::{RedirectUrlApi, ShortenUrlApi, UrlRequest, UrlResponse},
};
use uncovr::prelude::*;

#[async_trait]
impl API for ShortenUrlApi {
    type Req = UrlRequest;
    type Res = ApiResponse<UrlResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Validate URL is not empty
        if ctx.req.url.is_empty() {
            return ApiResponse::BadRequest {
                code: "empty_url",
                message: "URL cannot be empty",
            };
        }

        // Validate URL format
        if !ctx.req.url.starts_with("http://") && !ctx.req.url.starts_with("https://") {
            return ApiResponse::BadRequest {
                code: "invalid_url_format",
                message: "URL must start with http:// or https://",
            };
        }

        let base_url = "http://localhost:8000";
        let original_url = ctx.req.url.clone();

        // Shorten the URL
        let short_url = fun::shorten_url(base_url, &original_url);

        ApiResponse::Ok(UrlResponse { short_url })
    }
}

#[async_trait]
impl API for RedirectUrlApi {
    type Req = ();
    type Res = ApiResponse<UrlResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let id = ctx.path.get("id").unwrap_or_default();
        let url = fun::get_original_url(id);

        match url {
            Some(url) => ApiResponse::MovedPermanently(url),
            None => ApiResponse::NotFound {
                code: "url_not_found",
                message: "URL not found",
            },
        }
    }
}
```

::: details Understanding the Handler

**Validation**: Always validate input early. Return `BadRequest` for invalid data.

**Error Codes**: Use descriptive codes like `empty_url` and `invalid_url_format`. These help clients handle specific errors.

**Path Parameters**: Extract with `ctx.path.get("id")`. The router automatically captures values from the URL.

**Response Types**:
- `ApiResponse::Ok` - Success with data (200)
- `ApiResponse::BadRequest` - Client error (400)
- `ApiResponse::NotFound` - Resource not found (404)
- `ApiResponse::MovedPermanently` - Redirect (301)

Each response type maps to the correct HTTP status code automatically.
:::

## Step 4: Module Exports

Create `src/url/mod.rs` to export the module:

```rust
pub mod apis;
pub mod handlers;
```

## Step 5: Server Setup

Finally, wire everything together in `src/main.rs`:

```rust
use uncovr::{prelude::*, server::Server};

use crate::url::apis::{RedirectUrlApi, ShortenUrlApi};

mod fun;
mod url;

#[tokio::main]
async fn main() {
    let config = AppConfig::new("URL SHORTENER API", "0.1.0")
        .bind("0.0.0.0:8000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(ShortenUrlApi)
        .register(RedirectUrlApi)
        .serve()
        .await
        .expect("Something went wrong while starting Url Shortener Server")
}
```

::: tip Configuration
- `bind()` - Sets the address and port
- `environment()` - Development mode enables detailed logging
- `register()` - Adds endpoints to the server

The order of `register()` calls doesn't matter. The router handles all routing automatically.
:::

## Running the Application

Start the server:

```bash
cargo run
```

Visit `http://localhost:8000/docs` to see your interactive API documentation.

## Testing the API

**Shorten a URL:**

```bash
curl -X POST http://localhost:8000/url \
  -H "Content-Type: application/json" \
  -d '{"url":"https://github.com/erickweyunga/uncovr"}'
```

Response:
```json
{
  "short_url": "http://localhost:8000/abc123"
}
```

**Use the short URL:**

```bash
curl -L http://localhost:8000/abc123
```

The redirect happens automatically, taking you to the original URL.

**Test error handling:**

```bash
curl -X POST http://localhost:8000/url \
  -H "Content-Type: application/json" \
  -d '{"url":""}'
```

Response:
```json
{
  "code": "empty_url",
  "message": "URL cannot be empty"
}
```

## What You Learned

**Separation of Concerns**: The new Endpoint API separates routing (`ep()`) from documentation (`docs()`). This makes code cleaner and more maintainable.

**Type-Safe HTTP Methods**: Using `Route::GET()`, `Route::POST()`, etc. prevents typos and improves IDE support.

**Project Structure**: Organize by feature with separated concerns (`apis.rs` vs `handlers.rs`).

**Type Safety**: Request and response types catch errors at compile time.

**Error Handling**: Structured errors with codes and messages for better client integration.

**Response Documentation**: `responses()` generates complete OpenAPI specs.

**Path Parameters**: Extract dynamic values from URLs with `ctx.path`.

**Redirects**: Use `ApiResponse::MovedPermanently` for URL redirects.

**Optional Documentation**: You can skip `docs()` during prototyping and add it later.

## Key Differences from Old API

If you're familiar with the previous version of Uncovr, here are the key changes:

**Before:**
```rust
impl Metadata for ShortenUrlApi {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/url", "post")  // String-based method
            .summary("Shorten a URL")
            .with_responses(|op| { /* ... */ })
    }
}
```

**After:**
```rust
impl Endpoint for ShortenUrlApi {
    fn ep(&self) -> Route {
        Route::POST("/url")  // Type-safe method
    }
    
    fn docs(&self) -> Option<Docs> {
        Some(Docs::new()
            .summary("Shorten a URL")
            .responses(|op| { /* ... */ }))
    }
}
```

Benefits:
- ✅ Cleaner separation of routing and documentation
- ✅ Type-safe HTTP methods
- ✅ Optional documentation
- ✅ Better code organization

## Next Steps

- Add a database instead of in-memory storage
- Implement user authentication
- Add URL analytics (click tracking)
- Set expiration dates for short URLs
- Create a web interface

Explore the [complete source code](https://github.com/erickweyunga/uncovr/tree/main/examples/url-shortner) to see the full implementation with the new Endpoint API.