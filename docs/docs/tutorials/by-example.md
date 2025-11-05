# Uncovr By Example

Learn Uncovr through practical examples, from simple to complex.

## Hello World

The simplest possible Uncovr application:

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
    let config = AppConfig::new("Hello API", "1.0.0");

    Server::new()
        .with_config(config)
        .register(HelloWorld)
        .serve()
        .await
        .expect("Server failed");
}
```

**What's happening:**
- We define an endpoint struct `HelloWorld`
- Implement `Metadata` to specify the route (`/`) and HTTP method (`get`)
- Implement `API` to define request/response types and handler logic
- Register the endpoint with the server and start it

## JSON Request and Response

Handle JSON data with automatic serialization:

```rust
use uncovr::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, JsonSchema)]
pub struct GreetRequest {
    name: String,
}

#[derive(Serialize, JsonSchema)]
pub struct GreetResponse {
    message: String,
}

#[derive(Clone)]
pub struct Greet;

impl Metadata for Greet {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/greet", "post")
            .summary("Greet a user")
    }
}

#[async_trait]
impl API for Greet {
    type Req = GreetRequest;
    type Res = Json<GreetResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(GreetResponse {
            message: format!("Hello, {}!", ctx.req.name),
        })
    }
}
```

**Test it:**
```bash
curl -X POST http://localhost:3000/greet \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice"}'
```

**Response:**
```json
{"message":"Hello, Alice!"}
```

## Path Parameters

Extract parameters from the URL path:

```rust
use uncovr::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, JsonSchema)]
pub struct GetUserRequest {
    id: u64,
}

#[derive(Serialize, JsonSchema)]
pub struct User {
    id: u64,
    name: String,
}

#[derive(Clone)]
pub struct GetUser;

impl Metadata for GetUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "get")
            .summary("Get a user by ID")
    }
}

#[async_trait]
impl API for GetUser {
    type Req = GetUserRequest;
    type Res = Json<User>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(User {
            id: ctx.req.id,
            name: format!("User {}", ctx.req.id),
        })
    }
}
```

**Test it:**
```bash
curl http://localhost:3000/users/42
```

## Multiple Endpoints

Register multiple endpoints to build a complete API:

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct ListUsers;

impl Metadata for ListUsers {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")
            .summary("List all users")
    }
}

#[async_trait]
impl API for ListUsers {
    type Req = ();
    type Res = Json<Vec<String>>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        Json(vec!["Alice".into(), "Bob".into()])
    }
}

#[derive(Clone)]
pub struct CreateUser;

impl Metadata for CreateUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")
    }
}

#[async_trait]
impl API for CreateUser {
    type Req = CreateUserRequest;
    type Res = Json<User>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        Json(User {
            id: 1,
            name: ctx.req.name,
        })
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::new("Users API", "1.0.0");

    Server::new()
        .with_config(config)
        .register(ListUsers)
        .register(CreateUser)
        .serve()
        .await
        .expect("Server failed");
}
```

## Shared State

Share state (like database connections) across endpoints:

```rust
use uncovr::prelude::*;
use std::sync::Arc;

// Your shared state
#[derive(Clone)]
pub struct AppState {
    pub counter: Arc<std::sync::atomic::AtomicU64>,
}

#[derive(Clone)]
pub struct GetCount {
    pub state: AppState,
}

impl Metadata for GetCount {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/count", "get")
            .summary("Get current count")
    }
}

#[async_trait]
impl API for GetCount {
    type Req = ();
    type Res = String;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        let count = self.state.counter.load(std::sync::atomic::Ordering::Relaxed);
        format!("Count: {}", count)
    }
}

#[derive(Clone)]
pub struct Increment {
    pub state: AppState,
}

impl Metadata for Increment {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/increment", "post")
            .summary("Increment counter")
    }
}

#[async_trait]
impl API for Increment {
    type Req = ();
    type Res = String;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        let count = self.state.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("New count: {}", count + 1)
    }
}

#[tokio::main]
async fn main() {
    let state = AppState {
        counter: Arc::new(std::sync::atomic::AtomicU64::new(0)),
    };

    let config = AppConfig::new("Counter API", "1.0.0");

    Server::new()
        .with_config(config)
        .register(GetCount { state: state.clone() })
        .register(Increment { state })
        .serve()
        .await
        .expect("Server failed");
}
```

## Configuration

Configure your app for different environments:

```rust
use uncovr::prelude::*;

#[tokio::main]
async fn main() {
    let config = AppConfig::new("My API", "1.0.0")
        .description("A production-ready API")
        .bind("0.0.0.0:8080")
        .environment(Environment::Production)
        .logging(LoggingConfig::production())
        .cors(CorsConfig::production(vec![
            "https://myapp.com".to_string()
        ]))
        .docs(true);

    Server::new()
        .with_config(config)
        .register(HelloWorld)
        .serve()
        .await
        .expect("Server failed");
}
```

**Development configuration:**
```rust
let config = AppConfig::new("My API", "1.0.0")
    .bind("127.0.0.1:3000")
    .environment(Environment::Development)
    .logging(LoggingConfig::development())
    .cors(CorsConfig::development());
```

## Error Handling

Return custom errors with proper HTTP status codes:

```rust
use uncovr::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, JsonSchema)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Default, Deserialize, JsonSchema)]
pub struct DivideRequest {
    a: f64,
    b: f64,
}

#[derive(Clone)]
pub struct Divide;

impl Metadata for Divide {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/divide", "post")
            .summary("Divide two numbers")
    }
}

#[async_trait]
impl API for Divide {
    type Req = DivideRequest;
    type Res = Result<Json<f64>, (StatusCode, Json<ErrorResponse>)>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        if ctx.req.b == 0.0 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Cannot divide by zero".into(),
                }),
            ));
        }

        Ok(Json(ctx.req.a / ctx.req.b))
    }
}
```

## Accessing Headers

Read request headers in your handler:

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct GetAuthInfo;

impl Metadata for GetAuthInfo {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/auth-info", "get")
            .summary("Get authentication info")
    }
}

#[async_trait]
impl API for GetAuthInfo {
    type Req = ();
    type Res = String;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        if let Some(auth) = ctx.headers.get("authorization") {
            format!("Auth header: {:?}", auth)
        } else {
            "No auth header found".to_string()
        }
    }
}
```
