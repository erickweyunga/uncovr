# Uncovr Framework Improvement Plans

> **Version:** 0.3.0 Planning Document  
> **Created:** 2025-11-11  
> **Status:** In Progress  
> **Token Dictionary:** See [TOKEN_DICTIONARY.md](TOKEN_DICTIONARY.md)

## 🎯 API Design Decisions (APPROVED)

The following API design decisions have been approved and locked in for v0.3.0:

### **Core Naming Changes**
- ✅ `API` trait → `Handler` trait
- ✅ `.ep()` method → `.route()` method
- ✅ `.docs()` method → `.meta()` method (no longer Optional)
- ✅ `.handler()` method → `.handle()` method
- ✅ `ApiResponse<T>` → `Response<T>`
- ✅ `ApiError` → `Error`
- ✅ `PathParams` → `Path`
- ✅ `QueryParams` → `Query`
- ✅ `Route::GET()` → `Route::get()` (lowercase)
- ✅ `Route::POST()` → `Route::post()` (lowercase)

### **Design Patterns**
- ✅ **State Injection:** Context-based via `ctx.state::<T>()`
- ✅ **Error Handling:** Result-based with `Result<T, Error>`
- ✅ **Route Declaration:** Two methods (`.route()` + `.meta()`)
- ✅ **Macros:** Deferred to future version (keep explicit for now)
- ✅ **Parameter Inference:** Type-safe extraction with `.get::<T>()`

See [TOKEN_DICTIONARY.md](TOKEN_DICTIONARY.md) for complete naming conventions.

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Current State Assessment](#current-state-assessment)
3. [Priority-Ranked Improvements](#priority-ranked-improvements)
4. [Detailed Feature Proposals](#detailed-feature-proposals)
5. [Implementation Roadmap](#implementation-roadmap)
6. [Breaking Changes & Migration](#breaking-changes--migration)

---

## Executive Summary

Uncovr has a solid foundation with clean abstractions, type safety, and automatic OpenAPI documentation. Analysis of real-world examples (especially `auth-jwt` and `url-shortner`) reveals key pain points around:

- **State management boilerplate** - Every endpoint needs manual state injection
- **Verbose error handling** - Repetitive match statements for database/validation errors
- **Manual validation** - No framework support for input validation
- **Complex middleware API** - Nested tower layers are hard to understand
- **Parameter extraction verbosity** - Manual parsing with error handling
- **Limited testing support** - No test utilities or integration test examples

This document proposes improvements prioritized by **developer impact** and **implementation complexity**.

---

## Current State Assessment

### ✅ **Strengths**

1. **Type-safe endpoint definitions** - `Route::POST()` prevents typos
2. **Clean separation of concerns** - `Endpoint` (routing) + `API` (logic) + `Docs` (documentation)
3. **Automatic OpenAPI generation** - Interactive Scalar UI out of the box
4. **Built on stable foundation** - Axum, Tower, Tokio provide reliability
5. **Flexible state management** - Users control their own state pattern
6. **Good configuration system** - Environment-based presets (dev/prod)

### ⚠️ **Pain Points** (from examples analysis)

1. **State injection requires boilerplate** - Every endpoint: `pub struct E { state: AppState }`
2. **Path parameter parsing is verbose** - `ctx.path.get("id").and_then(|s| s.parse::<i64>().ok())`
3. **No validation framework** - Manual string checks everywhere
4. **Error handling is repetitive** - Every DB call wrapped in match
5. **Middleware API is complex** - `.layer(layer_fn(|inner| from_fn(middleware).layer(inner)))`
6. **No testing utilities** - Zero integration tests in examples
7. **Manual cloning** - `ctx.req.name.clone()` throughout handlers
8. **Documentation disconnect** - Route params defined separately from documentation

### 📊 **Framework Comparison**

| Feature | Uncovr (Current) | Actix-Web | Axum | Rocket |
|---------|------------------|-----------|------|--------|
| Type Safety | ✅ Excellent | ✅ Good | ✅ Excellent | ✅ Excellent |
| Auto OpenAPI | ✅ Built-in | ❌ External | ❌ External | ❌ External |
| State Injection | ⚠️ Manual | ✅ `Data<T>` | ✅ `State<T>` | ✅ `&State<T>` |
| Validation | ❌ None | ✅ actix-validator | ⚠️ Manual | ✅ Built-in |
| Error Handling | ⚠️ Verbose | ✅ `Result<T, E>` | ✅ `Result<T, E>` | ✅ `Result<T, E>` |
| Testing Utils | ❌ None | ✅ TestServer | ⚠️ Limited | ✅ Built-in |
| Middleware | ⚠️ Complex | ✅ Simple | ⚠️ Tower | ✅ Fairings |

**Key Insight:** Uncovr's OpenAPI strength is unique, but it lags in DX (developer experience) features that other frameworks have solved.

---

## Priority-Ranked Improvements

### 🔴 **P0: Critical (Next Release - v0.3.0)**

These features have the highest impact on developer experience and are blocking production adoption.

1. **[P0.1] Typed Path/Query Extractors** → Reduce parameter extraction verbosity by 80%
2. **[P0.2] State Injection Simplification** → Eliminate state boilerplate
3. **[P0.3] Result-based Error Handling** → Enable `?` operator, reduce error boilerplate
4. **[P0.4] Validation Framework** → Automatic input validation with descriptive errors

### 🟡 **P1: High Priority (v0.3.x)**

Significant quality-of-life improvements.

5. **[P1.1] Testing Utilities** → Test client and helpers for integration tests
6. **[P1.2] Simplified Middleware API** → Make middleware easier to compose
7. **[P1.3] Request/Response Ownership** → Reduce cloning, enable moves
8. **[P1.4] Built-in Middleware Collection** → Auth, CORS, rate limiting, etc.

### 🟢 **P2: Medium Priority (v0.4.0)**

Nice-to-have features that improve specific use cases.

9. **[P2.1] Database Integration Helpers** → Connection pools, transactions
10. **[P2.2] Advanced OpenAPI Features** → Security schemes, examples, webhooks
11. **[P2.3] Template Integration** → Built-in support for Tera/Handlebars
12. **[P2.4] Static File Serving** → Simpler API than Tower's ServeDir

### 🔵 **P3: Low Priority (Future)**

Long-term enhancements, not blocking.

13. **[P3.1] WebSocket Support** → Real-time communication
14. **[P3.2] Server-Sent Events** → Event streaming
15. **[P3.3] Code Generation** → Client SDKs from OpenAPI
16. **[P3.4] Admin Panel** → Auto-generated CRUD UI

---

## Detailed Feature Proposals

### **[P0.1] Typed Path/Query Extractors**

**Problem:**
```rust
// Current: 7 lines of boilerplate
let id = match ctx.path.get("id").and_then(|s| s.parse::<i64>().ok()) {
    Some(id) => id,
    None => {
        return ApiResponse::BadRequest {
            code: "invalid_id",
            message: "Invalid user ID format".to_string(),
        };
    }
};
```

**Proposed Solution:**
```rust
// New: Type-safe extraction in Context
let id: i64 = ctx.path.parse("id")?; // Returns early with 400 on error
// or
let id = ctx.path.get_or_400("id")?;
// or via newtype pattern
let UserId(id) = ctx.path.extract::<UserId>()?;
```

**Implementation Approach:**

1. **Add generic extraction methods to `PathParams`/`QueryParams`:**
```rust
impl PathParams {
    pub fn parse<T: FromStr>(&self, key: &str) -> Result<T, ParamError> {
        self.get(key)
            .ok_or(ParamError::Missing(key.to_string()))?
            .parse()
            .map_err(|_| ParamError::InvalidType(key.to_string()))
    }
    
    pub fn get_or_400<T: FromStr>(&self, key: &str) -> Result<T, ApiResponse<()>> {
        self.parse(key).map_err(|e| ApiResponse::BadRequest {
            code: "invalid_param",
            message: e.to_string(),
        })
    }
}
```

2. **Support newtype pattern:**
```rust
#[derive(Debug, Clone, Copy)]
pub struct UserId(pub i64);

impl FromParam for UserId {
    fn from_param(s: &str) -> Result<Self, ParamError> {
        s.parse::<i64>()
            .map(UserId)
            .map_err(|_| ParamError::InvalidType("UserId".into()))
    }
}

// Usage
let user_id = ctx.path.extract::<UserId>()?;
```

3. **Automatic OpenAPI documentation from types:**
```rust
impl JsonSchema for UserId {
    fn schema_name() -> String { "UserId".to_string() }
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        // Generates integer schema with description
    }
}
```

**Benefits:**
- ✅ Reduces boilerplate by ~80% for path/query extraction
- ✅ Type-safe with compile-time checking
- ✅ Automatic error responses with clear messages
- ✅ Better OpenAPI documentation

**Breaking Change:** None (additive API)

**Estimated Effort:** 2-3 days

---

### **[P0.2] State Injection Simplification**

**Problem:**
```rust
// Current: Repeated for every endpoint
#[derive(Clone)]
pub struct RegisterEndpoint {
    pub state: AppState, // Boilerplate
}

impl RegisterEndpoint {
    pub fn new(state: AppState) -> Self {
        Self { state } // Boilerplate
    }
}

// Registration
.register(RegisterEndpoint::new(state.clone())) // Clone everywhere
.register(LoginEndpoint::new(state.clone()))
.register(GetUserRouter::new(state.clone()))
```

**Proposed Solution A: Macro-based reduction**
```rust
// Define endpoint with automatic state injection
#[endpoint(state = AppState)]
pub struct RegisterEndpoint;

// Generated code handles state field and constructor
impl API for RegisterEndpoint {
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Access via self.state()
        let pool = self.state().db_pool;
    }
}

// Registration with automatic state injection
Server::new()
    .with_state(app_state)
    .register(RegisterEndpoint) // No manual state passing
    .register(LoginEndpoint)
    .serve()
```

**Proposed Solution B: Context-based state**
```rust
// State stored in Context
impl<Req> Context<Req> {
    pub fn state<S: Clone + Send + Sync + 'static>(&self) -> &S {
        self.extensions
            .get::<S>()
            .expect("State not found. Did you forget .with_state()?")
    }
}

// Usage in handler
async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    let state = ctx.state::<AppState>();
    let pool = &state.db_pool;
}

// Registration
Server::new()
    .with_state(app_state) // Stored in Extensions
    .register(RegisterEndpoint) // No state in constructor
```

**Recommended:** **Solution B** (Context-based)
- ✅ No macros (simpler, more explicit)
- ✅ Works with existing Extension pattern in Axum
- ✅ Familiar to Axum users
- ✅ Easy to test (inject mock state)

**Implementation:**
```rust
// In server/builder.rs
impl ServerBuilder {
    pub fn with_state<S: Clone + Send + Sync + 'static>(mut self, state: S) -> Self {
        self.router = self.router.layer(Extension(state));
        self
    }
}

// In context/context.rs
impl<Req> Context<Req> {
    pub fn state<S: Clone + Send + Sync + 'static>(&self) -> &S {
        self.extensions
            .get::<S>()
            .expect("State not registered. Call .with_state() before .register()")
    }
    
    pub fn try_state<S: Clone + Send + Sync + 'static>(&self) -> Option<&S> {
        self.extensions.get::<S>()
    }
}
```

**Benefits:**
- ✅ Eliminates state boilerplate in endpoint structs
- ✅ Single `.with_state()` call instead of multiple `.clone()`
- ✅ Easier to test with mock state
- ✅ Consistent with Axum patterns

**Breaking Change:** Minor (endpoints with state fields need refactoring)

**Migration Path:**
```rust
// Before
#[derive(Clone)]
pub struct MyEndpoint {
    state: AppState,
}

// After
#[derive(Clone)]
pub struct MyEndpoint;

impl API for MyEndpoint {
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let state = ctx.state::<AppState>(); // New
        // Rest stays the same
    }
}
```

**Estimated Effort:** 3-4 days

---

### **[P0.3] Result-based Error Handling**

**Problem:**
```rust
// Current: Verbose match everywhere
let user = match sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
    .fetch_one(&state.db_pool)
    .await
{
    Ok(Some(user)) => user,
    Ok(None) => {
        return ApiResponse::NotFound {
            code: "user_not_found",
            message: format!("User {} not found", id),
        };
    }
    Err(e) => {
        tracing::error!("Database error: {:?}", e);
        return ApiResponse::InternalError {
            code: "db_error",
            message: "Failed to fetch user".to_string(),
        };
    }
};
```

**Proposed Solution:**
```rust
// New: Use ? operator with automatic conversion
let user = fetch_user(&state.db_pool, id).await?;

// Response type supports Result
impl API for GetUserEndpoint {
    type Req = ();
    type Res = Result<Json<User>, ApiError>; // New
    
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let id = ctx.path.parse::<i64>("id")?; // Can use ?
        let user = fetch_user(&ctx.state::<AppState>().db_pool, id).await?; // Can use ?
        Ok(Json(user))
    }
}

// Automatic conversion to HTTP response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound { .. } => (StatusCode::NOT_FOUND, Json(self)),
            ApiError::BadRequest { .. } => (StatusCode::BAD_REQUEST, Json(self)),
            ApiError::InternalError { .. } => (StatusCode::INTERNAL_SERVER_ERROR, Json(self)),
        }.into_response()
    }
}
```

**Implementation:**

1. **Create `ApiError` type** (replaces `ApiResponse` for errors):
```rust
#[derive(Debug, Serialize, JsonSchema)]
#[serde(tag = "error", content = "details")]
pub enum ApiError {
    BadRequest { code: String, message: String },
    Unauthorized { code: String, message: String },
    Forbidden { code: String, message: String },
    NotFound { code: String, message: String },
    Conflict { code: String, message: String },
    InternalError { code: String, message: String },
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest { message, .. } => write!(f, "Bad Request: {}", message),
            Self::NotFound { message, .. } => write!(f, "Not Found: {}", message),
            // ...
        }
    }
}

impl std::error::Error for ApiError {}
```

2. **Implement `From` traits for common errors:**
```rust
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", e);
        ApiError::InternalError {
            code: "db_error".to_string(),
            message: "Database operation failed".to_string(),
        }
    }
}

impl From<ParamError> for ApiError {
    fn from(e: ParamError) -> Self {
        ApiError::BadRequest {
            code: "invalid_param".to_string(),
            message: e.to_string(),
        }
    }
}
```

3. **Update `API` trait to support Result responses:**
```rust
#[async_trait]
pub trait API {
    type Req: DeserializeOwned + Default + Send + 'static;
    type Res: IntoResponse + Send + 'static; // Changed: was more restrictive
    
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res;
}
```

4. **Keep `ApiResponse<T>` for success variants:**
```rust
pub enum ApiResponse<T: Serialize> {
    Ok(T),           // 200
    Created(T),      // 201
    NoContent,       // 204
}

// Usage: Return success
Ok(ApiResponse::Created(user))
// Or: Return error
Err(ApiError::NotFound { ... })
```

**Benefits:**
- ✅ Use `?` operator throughout handlers
- ✅ Reduces error handling boilerplate by ~60%
- ✅ Consistent error responses
- ✅ Better error propagation
- ✅ Works with existing Rust error ecosystem

**Breaking Change:** Yes (response types change)

**Migration Path:**
```rust
// Before
type Res = ApiResponse<User>;

async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    match db.fetch().await {
        Ok(user) => ApiResponse::Ok(user),
        Err(_) => ApiResponse::InternalError { ... }
    }
}

// After
type Res = Result<Json<User>, ApiError>;

async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    let user = db.fetch().await?; // Simple!
    Ok(Json(user))
}
```

**Estimated Effort:** 4-5 days

---

### **[P0.4] Validation Framework**

**Problem:**
```rust
// Current: Manual validation everywhere
if ctx.req.url.is_empty() {
    return Err(ApiError::BadRequest {
        code: "empty_url",
        message: "URL cannot be empty".to_string(),
    });
}

if !ctx.req.url.starts_with("http://") && !ctx.req.url.starts_with("https://") {
    return Err(ApiError::BadRequest {
        code: "invalid_url",
        message: "URL must start with http:// or https://".to_string(),
    });
}

if ctx.req.password.len() < 8 {
    return Err(ApiError::BadRequest {
        code: "weak_password",
        message: "Password must be at least 8 characters".to_string(),
    });
}
```

**Proposed Solution:**
```rust
use uncovr::validation::Validate;
use validator::Validate as ValidatorDerive;

#[derive(Deserialize, JsonSchema, ValidatorDerive)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    name: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8), custom = "validate_strong_password")]
    password: String,
}

// Automatic validation before handler
impl API for RegisterEndpoint {
    type Req = CreateUserRequest; // Auto-validated
    type Res = Result<Json<User>, ApiError>;
    
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // ctx.req is already validated!
        // If validation fails, 400 returned before this runs
        let user = create_user(ctx.req).await?;
        Ok(Json(user))
    }
}
```

**Implementation:**

1. **Add `validator` crate** to dependencies:
```toml
[dependencies]
validator = { version = "0.18", features = ["derive"] }
```

2. **Create validation integration** in `server/builder.rs`:
```rust
// Before calling handler, validate if type implements Validate
use validator::Validate;

// In the POST/PUT/PATCH handler wrapper
let payload: E::Req = Json::from_request(req, state).await?;

// Add validation check
if let Some(errors) = validate_request(&payload) {
    return Ok(ApiError::ValidationError(errors).into_response());
}

// Continue to handler
let ctx = Context { req: payload, ... };
endpoint.handler(ctx).await
```

3. **Add `ValidationError` to `ApiError`:**
```rust
pub enum ApiError {
    // ... existing variants
    
    ValidationError {
        code: String,
        message: String,
        fields: HashMap<String, Vec<String>>, // field -> [error messages]
    },
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let fields = errors
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
                let messages = errors
                    .iter()
                    .map(|e| e.message.clone().unwrap_or_default().to_string())
                    .collect();
                (field.to_string(), messages)
            })
            .collect();
        
        ApiError::ValidationError {
            code: "validation_failed".to_string(),
            message: "Request validation failed".to_string(),
            fields,
        }
    }
}
```

4. **Update OpenAPI to include validation constraints:**
```rust
// Schemars integration to show validation rules in docs
impl JsonSchema for CreateUserRequest {
    fn schema_name() -> String { "CreateUserRequest".to_string() }
    
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = gen.subschema_for::<String>();
        // Add minLength, maxLength, pattern from validator attributes
        schema
    }
}
```

**Benefits:**
- ✅ Eliminates manual validation boilerplate
- ✅ Declarative validation with attributes
- ✅ Consistent error responses
- ✅ OpenAPI docs show validation rules
- ✅ Works with standard `validator` crate

**Breaking Change:** None (opt-in feature)

**Migration Path:**
```rust
// Before
async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    if ctx.req.email.is_empty() { /* error */ }
    if !is_valid_email(&ctx.req.email) { /* error */ }
    // ... business logic
}

// After
#[derive(Deserialize, Validate)]
struct Request {
    #[validate(email)]
    email: String,
}

async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    // Validation already done!
    // ... business logic
}
```

**Estimated Effort:** 3-4 days

---

### **[P1.1] Testing Utilities**

**Problem:** No examples have integration tests. Testing requires manual setup:
```rust
// Currently no way to test endpoints
```

**Proposed Solution:**
```rust
use uncovr::testing::TestClient;

#[tokio::test]
async fn test_create_user() {
    // Test client with app state
    let state = AppState::test(); // Test database
    let client = TestClient::new()
        .with_state(state)
        .register(CreateUserEndpoint);
    
    // Make request
    let response = client
        .post("/users")
        .json(&CreateUserRequest {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        })
        .send()
        .await;
    
    // Assertions
    response.assert_status(201);
    response.assert_json::<User>(|user| {
        assert_eq!(user.name, "Alice");
    });
}

#[tokio::test]
async fn test_auth_required() {
    let client = TestClient::new()
        .with_state(AppState::test())
        .register(ProtectedEndpoint);
    
    // Test without auth
    let response = client.get("/protected").send().await;
    response.assert_status(401);
    
    // Test with auth
    let response = client
        .get("/protected")
        .bearer_token("valid_token")
        .send()
        .await;
    response.assert_status(200);
}
```

**Implementation:**

1. **Create `testing` module** in `src/testing/mod.rs`:
```rust
pub struct TestClient {
    router: Router,
    state: Option<Extension<AppState>>,
}

impl TestClient {
    pub fn new() -> Self { /* ... */ }
    
    pub fn with_state<S: Clone + Send + Sync + 'static>(self, state: S) -> Self { /* ... */ }
    
    pub fn register<E: Endpoint + API + Clone + 'static>(mut self, endpoint: E) -> Self {
        // Same logic as ServerBuilder
        self
    }
    
    pub fn get(&self, path: &str) -> RequestBuilder { /* ... */ }
    pub fn post(&self, path: &str) -> RequestBuilder { /* ... */ }
    // ... other HTTP methods
}

pub struct RequestBuilder {
    method: Method,
    path: String,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub fn json<T: Serialize>(mut self, body: &T) -> Self { /* ... */ }
    pub fn header(mut self, key: &str, value: &str) -> Self { /* ... */ }
    pub fn bearer_token(self, token: &str) -> Self {
        self.header("Authorization", &format!("Bearer {}", token))
    }
    
    pub async fn send(self) -> TestResponse { /* ... */ }
}

pub struct TestResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl TestResponse {
    pub fn assert_status(&self, expected: u16) {
        assert_eq!(self.status.as_u16(), expected, "Status mismatch");
    }
    
    pub fn assert_json<T: DeserializeOwned>(&self, f: impl FnOnce(T)) {
        let value: T = serde_json::from_slice(&self.body).expect("Invalid JSON");
        f(value);
    }
    
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.body).expect("Invalid JSON")
    }
}
```

2. **Add test database helpers:**
```rust
impl AppState {
    pub async fn test() -> Self {
        // Create temporary database
        let db_url = format!("sqlite::memory:");
        let pool = SqlitePool::connect(&db_url).await.unwrap();
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        Self { db_pool: pool }
    }
}
```

3. **Add to prelude for easy access:**
```rust
#[cfg(test)]
pub use crate::testing::*;
```

**Benefits:**
- ✅ Easy integration testing
- ✅ No need to start actual server
- ✅ Fluent assertion API
- ✅ Familiar to users of other frameworks

**Breaking Change:** None (new module)

**Estimated Effort:** 5-6 days

---

### **[P1.2] Simplified Middleware API**

**Problem:**
```rust
// Current: Confusing nested layers
.layer(layer_fn(|inner| from_fn(auth_middleware).layer(inner)))
```

**Proposed Solution:**
```rust
// New: Simple middleware registration
.middleware(auth_middleware)
.middleware_fn(|req, next| async {
    // Inline middleware
    tracing::info!("Before request");
    let res = next.run(req).await;
    tracing::info!("After request");
    res
})
```

**Implementation:**
```rust
impl ServerBuilder {
    pub fn middleware<M>(self, middleware: M) -> Self
    where
        M: tower::Layer<Route> + Clone + Send + 'static,
    {
        self.layer(middleware)
    }
    
    pub fn middleware_fn<F>(self, f: F) -> Self
    where
        F: Fn(Request, Next) -> Future<Response> + Clone + Send + 'static,
    {
        self.layer(from_fn(f))
    }
}
```

**Benefits:**
- ✅ Cleaner API
- ✅ Less Tower knowledge required
- ✅ Inline middleware support

**Breaking Change:** None (alias for `.layer()`)

**Estimated Effort:** 1-2 days

---

### **[P1.3] Request Ownership Improvements**

**Problem:**
```rust
// Current: Cloning everywhere
Json(User {
    name: ctx.req.name.clone(),
    email: ctx.req.email.clone(),
})
```

**Proposed Solution A: Allow consuming request**
```rust
// New: Consume request
Json(User {
    name: ctx.req.name,     // Moved
    email: ctx.req.email,   // Moved
})
```

**Implementation:** Change `Context` to allow consuming `req`:
```rust
impl<Req> Context<Req> {
    pub fn into_req(self) -> Req {
        self.req
    }
}

// Usage
async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    let req = ctx.into_req(); // Consume
    Json(User {
        name: req.name,   // No clone
        email: req.email, // No clone
    })
}
```

**Proposed Solution B: Make req generic over ownership**
```rust
// Advanced: Zero-copy where possible
pub struct Context<Req = ()> {
    pub req: Req,
    // ... other fields stay owned/Arc
}
```

**Benefits:**
- ✅ Reduces cloning
- ✅ More efficient
- ✅ Cleaner code

**Breaking Change:** Minor (handlers that use ctx after req may break)

**Estimated Effort:** 2-3 days

---

### **[P1.4] Built-in Middleware Collection**

**Problem:** Users must integrate Tower middleware manually.

**Proposed Solution:**
```rust
use uncovr::middleware::{Auth, RateLimit, RequestId, Cors};

Server::new()
    .middleware(RequestId::new()) // Adds X-Request-Id
    .middleware(RateLimit::new(100, Duration::from_secs(60))) // 100 req/min
    .middleware(Auth::bearer(validate_token)) // JWT validation
    .middleware(Cors::permissive()) // CORS
```

**Implementation:** Create built-in middleware in `src/middleware/`:
- `request_id.rs` - Generate request IDs
- `rate_limit.rs` - Rate limiting
- `auth.rs` - Authentication helpers
- `cors.rs` - Simpler CORS configuration

**Benefits:**
- ✅ Common features out of the box
- ✅ Better documentation
- ✅ Consistent patterns

**Estimated Effort:** 6-8 days

---

### **[P2.1] Database Integration Helpers**

**Problem:** Database setup is manual, migrations not integrated.

**Proposed Solution:**
```rust
use uncovr::database::{DatabaseConfig, ConnectionPool};

// Configuration
let db_config = DatabaseConfig::postgres("postgres://localhost/mydb")
    .max_connections(10)
    .min_connections(2)
    .acquire_timeout(Duration::from_secs(5))
    .migrations("./migrations");

// Connect with automatic migrations
let pool = db_config.connect().await?;

// Use in state
let state = AppState {
    db: pool,
};

Server::new()
    .with_state(state)
    .serve()
    .await
```

**Implementation:**
- Wrapper around SQLx with better defaults
- Automatic migration runner
- Health check endpoints
- Connection pool monitoring

**Benefits:**
- ✅ Easier database setup
- ✅ Built-in migrations
- ✅ Health checks

**Estimated Effort:** 4-5 days

---

### **[P2.2] Advanced OpenAPI Features**

**Current:** Basic OpenAPI with tags, summaries, descriptions.

**Proposed Additions:**
- Security schemes (Bearer, API Key, OAuth2)
- Request/response examples
- Webhook definitions
- Deprecation warnings
- External documentation links

```rust
fn docs(&self) -> Option<Docs> {
    Some(Docs::new()
        .summary("Create user")
        .security("bearerAuth") // New
        .example(CreateUserRequest { // New
            name: "Alice".into(),
            email: "alice@example.com".into(),
        })
        .response_example::<200>(User { // New
            id: 1,
            name: "Alice".into(),
        })
        .deprecated(true) // New
        .external_docs("https://docs.example.com/users") // New
    )
}
```

**Estimated Effort:** 5-6 days

---

### **[P2.3] Template Integration**

**Problem:** Template rendering requires manual setup (see `live-reload` example).

**Proposed Solution:**
```rust
use uncovr::templates::{Tera, Template};

// Configure templates
let templates = Tera::new("templates/**/*")?;

// Use in handler
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    message: String,
}

async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    IndexTemplate {
        title: "Home".into(),
        message: "Welcome".into(),
    }
}
```

**Implementation:** Built-in support for Tera/Handlebars/Askama.

**Estimated Effort:** 3-4 days

---

### **[P2.4] Static File Serving**

**Problem:** Requires Tower's `ServeDir` manually.

**Proposed Solution:**
```rust
Server::new()
    .static_files("/public", "public/") // Simple!
    .static_files("/assets", "assets/")
    .spa("/app", "dist/") // SPA with fallback to index.html
```

**Estimated Effort:** 2-3 days

---

## Implementation Roadmap

### **Phase 1: Foundation (v0.3.0) - 3-4 weeks**

**Focus:** Core DX improvements that reduce boilerplate.

| Feature | Priority | Effort | Status |
|---------|----------|--------|--------|
| Typed Path/Query Extractors | P0.1 | 2-3 days | ⏸️ Not Started |
| State Injection (Context-based) | P0.2 | 3-4 days | ⏸️ Not Started |
| Result-based Error Handling | P0.3 | 4-5 days | ⏸️ Not Started |
| Validation Framework | P0.4 | 3-4 days | ⏸️ Not Started |

**Deliverables:**
- Updated examples with new patterns
- Migration guide
- Updated documentation
- Blog post announcing improvements

**Success Metrics:**
- Boilerplate reduced by 50%+ (measured by LOC in examples)
- Positive feedback from early adopters
- Zero P0 bugs reported

---

### **Phase 2: Testing & Middleware (v0.3.x) - 2-3 weeks**

**Focus:** Quality of life and production readiness.

| Feature | Priority | Effort | Status |
|---------|----------|--------|--------|
| Testing Utilities | P1.1 | 5-6 days | ⏸️ Not Started |
| Simplified Middleware API | P1.2 | 1-2 days | ⏸️ Not Started |
| Request Ownership | P1.3 | 2-3 days | ⏸️ Not Started |
| Built-in Middleware | P1.4 | 6-8 days | ⏸️ Not Started |

**Deliverables:**
- Integration tests for all examples
- Middleware guide
- Production deployment guide

---

### **Phase 3: Database & Advanced Features (v0.4.0) - 4-5 weeks**

**Focus:** Common integrations and advanced use cases.

| Feature | Priority | Effort | Status |
|---------|----------|--------|--------|
| Database Helpers | P2.1 | 4-5 days | ⏸️ Not Started |
| Advanced OpenAPI | P2.2 | 5-6 days | ⏸️ Not Started |
| Template Integration | P2.3 | 3-4 days | ⏸️ Not Started |
| Static File Serving | P2.4 | 2-3 days | ⏸️ Not Started |

---

### **Phase 4: Future Enhancements (v0.5.0+) - TBD**

| Feature | Priority | Effort | Status |
|---------|----------|--------|--------|
| WebSocket Support | P3.1 | 7-10 days | ⏸️ Not Started |
| Server-Sent Events | P3.2 | 3-4 days | ⏸️ Not Started |
| Code Generation | P3.3 | 10-14 days | ⏸️ Not Started |
| Admin Panel | P3.4 | 14-21 days | ⏸️ Not Started |

---

## Breaking Changes & Migration

### **v0.3.0 Breaking Changes**

1. **Error Handling:** `ApiResponse` split into `ApiResponse<T>` (success) and `ApiError` (errors)
   - **Migration:** Replace error variants with `Err(ApiError::...)`
   
2. **State Management:** Endpoints no longer need state fields
   - **Migration:** Remove state field, use `ctx.state::<T>()` instead

3. **Response Types:** `API::Res` must implement `IntoResponse`
   - **Migration:** Most types already do, use `Result<T, E>` for fallible handlers

### **Migration Tool**

Create a migration tool to automate common changes:
```bash
cargo install uncovr-migrate
uncovr-migrate --from 0.2 --to 0.3
```

### **Compatibility Layer**

Provide a `uncovr-compat` crate for gradual migration:
```rust
// Old code still works
use uncovr_compat::prelude::*;
```

---

## Success Criteria

### **Framework Maturity Indicators**

- [ ] Used in production by 10+ teams
- [ ] 1000+ GitHub stars
- [ ] 50+ community contributions
- [ ] Featured in "Are We Web Yet?"
- [ ] Comparison benchmarks vs Actix/Axum/Rocket

### **Developer Experience Metrics**

- [ ] Time to "Hello World": < 5 minutes
- [ ] Time to production app: < 1 day
- [ ] Boilerplate per endpoint: < 20 LOC
- [ ] Compile time for 100 endpoints: < 30s
- [ ] Documentation coverage: 100%

### **Community Health**

- [ ] Active Discord/Discussions
- [ ] Weekly/monthly releases
- [ ] Response time to issues: < 48h
- [ ] Contributor guide
- [ ] Code of conduct

---

## Questions & Decisions Needed

### **Open Design Questions**

1. **State Injection:** Context-based vs Macro-based? (Recommendation: Context)
2. **Validation:** Required or optional feature? (Recommendation: Optional)
3. **Error Handling:** Keep `ApiResponse` or switch to `Result` entirely? (Recommendation: Both)
4. **Breaking Changes:** v0.3 or v1.0? (Recommendation: v0.3, save v1.0 for stable API)
5. **Testing:** Built-in test client or separate crate? (Recommendation: Built-in)

### **Community Input Needed**

- Survey existing users about pain points
- RFC process for major changes
- Beta testing program for v0.3.0

---

## Resources & References

### **Similar Frameworks**
- [Actix-Web](https://actix.rs/) - Popular Rust web framework
- [Axum](https://github.com/tokio-rs/axum) - Ergonomic Tokio-based framework
- [Rocket](https://rocket.rs/) - Type-safe framework with code generation
- [Loco](https://loco.rs/) - Rails-like framework for Rust

### **Inspiration**
- [FastAPI](https://fastapi.tiangolo.com/) - Python framework with auto docs
- [NestJS](https://nestjs.com/) - TypeScript framework with DI
- [Spring Boot](https://spring.io/projects/spring-boot) - Java framework

### **Learning Resources**
- [Aide Documentation](https://github.com/tamasfe/aide) - OpenAPI integration
- [Tower Guide](https://github.com/tower-rs/tower) - Middleware patterns
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)

---

## Appendix

### **Example: Full Before/After**

**Before (v0.2.x):**
```rust
#[derive(Clone)]
pub struct GetUserEndpoint {
    pub state: AppState,
}

impl GetUserEndpoint {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Endpoint for GetUserEndpoint {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/users/:id");
        route.path_param("id").desc("User ID");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new()
            .summary("Get user by ID")
            .tag("users"))
    }
}

#[async_trait]
impl API for GetUserEndpoint {
    type Req = ();
    type Res = ApiResponse<User>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let id = match ctx.path.get("id").and_then(|s| s.parse::<i64>().ok()) {
            Some(id) => id,
            None => {
                return ApiResponse::BadRequest {
                    code: "invalid_id",
                    message: "Invalid user ID".to_string(),
                };
            }
        };

        match sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&self.state.db_pool)
            .await
        {
            Ok(user) => ApiResponse::Ok(user),
            Err(sqlx::Error::RowNotFound) => ApiResponse::NotFound {
                code: "user_not_found",
                message: format!("User {} not found", id),
            },
            Err(e) => {
                tracing::error!("Database error: {:?}", e);
                ApiResponse::InternalError {
                    code: "db_error",
                    message: "Failed to fetch user".to_string(),
                }
            }
        }
    }
}

// Registration
Server::new()
    .register(GetUserEndpoint::new(state.clone()))
```

**After (v0.3.0):**
```rust
#[derive(Clone)]
pub struct GetUserEndpoint;

impl Endpoint for GetUserEndpoint {
    fn ep(&self) -> Route {
        Route::GET("/users/:id")
            .path_param("id").desc("User ID")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new()
            .summary("Get user by ID")
            .tag("users"))
    }
}

#[async_trait]
impl API for GetUserEndpoint {
    type Req = ();
    type Res = Result<Json<User>, ApiError>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let id = ctx.path.parse::<i64>("id")?;
        let state = ctx.state::<AppState>();
        
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&state.db_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::not_found("user_not_found", "User not found"),
                _ => ApiError::internal("db_error", "Database error"),
            })?;
        
        Ok(Json(user))
    }
}

// Registration
Server::new()
    .with_state(state)
    .register(GetUserEndpoint)
```

**Lines of Code:**
- Before: 58 LOC
- After: 35 LOC
- **Reduction: 40%**

---

## Contributing to This Plan

This is a living document. To propose changes:

1. Open an issue with `[PLAN]` prefix
2. Discuss in GitHub Discussions
3. Submit PR to update this document
4. Get approval from maintainers

**Questions?** Open a discussion at https://github.com/erickweyunga/uncovr/discussions

---

**Last Updated:** 2025-11-11  
**Next Review:** After v0.3.0 release
