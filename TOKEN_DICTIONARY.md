# Uncovr Token Dictionary

> **Version:** 1.0.0  
> **For Framework Version:** 0.3.0+  
> **Status:** Official Standard

This document defines the official naming conventions, terminology, and API patterns for the Uncovr framework. All code, documentation, and examples must follow these conventions for consistency.

---

## Table of Contents

1. [Design Philosophy](#design-philosophy)
2. [Core Types](#core-types)
3. [Traits](#traits)
4. [Methods](#methods)
5. [HTTP Conventions](#http-conventions)
6. [Response & Error Handling](#response--error-handling)
7. [Builder Patterns](#builder-patterns)
8. [Naming Rules](#naming-rules)
9. [Migration from v0.2.x](#migration-from-v02x)

---

## Design Philosophy

### **Guiding Principles**

1. **Simplicity Over Cleverness** - Clear names beat short names
2. **Consistency** - Same patterns everywhere
3. **Rust Idioms** - Follow stdlib conventions (lowercase methods, Result types, etc.)
4. **No Redundancy** - Don't prefix everything with "Api" or "Http"
5. **Intuitive** - Names should be obvious to newcomers

### **Anti-Patterns to Avoid**

❌ **Cryptic abbreviations:** `ep()`, `ctx()`, `req()`  
✅ **Full words:** `route()`, `context()`, `request()`

❌ **Redundant prefixes:** `ApiResponse`, `ApiError`, `HttpRequest`  
✅ **Context-aware names:** `Response`, `Error`, `Request`

❌ **Uppercase methods:** `Route::GET()`, `Route::POST()`  
✅ **Lowercase methods:** `Route::get()`, `Route::post()`

❌ **Optional docs:** `fn docs(&self) -> Option<Docs>`  
✅ **Always provide:** `fn meta(&self) -> Meta`

---

## Core Types

### **Framework Types**

| Type | Purpose | Example Usage |
|------|---------|---------------|
| `Route` | HTTP route definition | `Route::get("/users/:id")` |
| `Meta` | Endpoint metadata (docs, tags, OpenAPI) | `Meta::new().summary("Get user")` |
| `Context<T>` | Request context with body, params, state | `ctx.path.get::<i64>("id")` |
| `Path` | Path parameter extractor | `ctx.path.get("id")` |
| `Query` | Query parameter extractor | `ctx.query.get("page")` |
| `Response<T>` | Success response wrapper | `Response::ok(user)` |
| `Error` | Error response | `Error::not_found("user_not_found", "User not found")` |
| `Server` | Server builder | `Server::new().register(endpoint)` |

### **Type Naming Conventions**

- **Singular form:** `Path` not `Paths`, `Query` not `Queries`
- **No prefixes:** `Response` not `ApiResponse`, `Error` not `HttpError`
- **Descriptive:** `Context` not `Ctx`, `Request` not `Req`
- **PascalCase:** All types use `PascalCase`

---

## Traits

### **Core Traits**

#### **`Endpoint`**

Defines the route and metadata for an endpoint.

```rust
pub trait Endpoint {
    /// Define the HTTP route
    fn route(&self) -> Route;
    
    /// Define metadata for OpenAPI documentation
    fn meta(&self) -> Meta;
}
```

**Usage:**
```rust
impl Endpoint for GetUser {
    fn route(&self) -> Route {
        Route::get("/users/:id")
    }
    
    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Get user by ID")
            .tag("users")
    }
}
```

#### **`Handler`**

Implements the request handling logic.

```rust
#[async_trait]
pub trait Handler {
    /// The request body type
    type Request: DeserializeOwned + Default + Send + 'static;
    
    /// The response type
    type Response: IntoResponse + Send + 'static;
    
    /// Handle the request
    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response;
}
```

**Usage:**
```rust
#[async_trait]
impl Handler for GetUser {
    type Request = ();
    type Response = Result<Json<User>, Error>;
    
    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
        let id = ctx.path.get::<i64>("id")?;
        // ... logic
        Ok(Json(user))
    }
}
```

### **Trait Naming Conventions**

- **Descriptive nouns:** `Handler`, `Endpoint`, `Validator`
- **Not verbs:** Not `Handle`, `Route`, `Validate`
- **No "able" suffix unless truly optional:** `Endpoint` not `Routable`

---

## Methods

### **Method Naming Conventions**

| Pattern | Example | When to Use |
|---------|---------|-------------|
| **Verbs for actions** | `.handle()`, `.register()`, `.serve()` | Methods that do something |
| **Nouns for getters** | `.route()`, `.meta()`, `.state()` | Methods that return something |
| **`get` prefix for extraction** | `.get()`, `.get_or_default()` | Extracting values with potential failure |
| **`with` prefix for builders** | `.with_state()`, `.with_config()` | Optional builder configuration |
| **No prefix for required** | `.route()`, `.summary()`, `.tag()` | Required builder methods |
| **snake_case always** | `.some_method()` | All methods |

### **Core Methods**

#### **Endpoint Definition**

```rust
.route()        // Define HTTP route (returns Route)
.meta()         // Define metadata (returns Meta)
```

#### **Handler**

```rust
.handle()       // Handle request (async, returns Response)
```

#### **Route Building**

```rust
.param()        // Document a parameter
.required()     // Mark parameter as required
.deprecated()   // Mark route as deprecated
```

#### **Meta Building**

```rust
.summary()      // Short description (one line)
.describe()     // Long description (multiple paragraphs)
.tag()          // Add OpenAPI tag
.example()      // Add request/response example
```

#### **Context Access**

```rust
.get()          // Get parameter (returns Option<T>)
.parse()        // Parse parameter (returns Result<T>)
.state()        // Get application state (returns &T)
```

#### **Server Building**

```rust
.register()     // Register endpoint
.with_state()   // Set application state
.with_config()  // Set configuration
.layer()        // Add middleware layer
.middleware()   // Add middleware (alias for layer)
.nest()         // Nest router at path
.serve()        // Start server (async)
```

### **Method Examples**

**Good:**
```rust
ctx.path.get::<i64>("id")?
ctx.query.get("page").unwrap_or(1)
ctx.state::<AppState>()
Route::get("/users/:id")
Meta::new().summary("Get user")
```

**Bad:**
```rust
ctx.getPath("id")              // Wrong case
ctx.path.extract_id()          // Too specific
Route::GET("/users/:id")       // Uppercase
Meta::new().setSummary("...")  // Wrong case
```

---

## HTTP Conventions

### **HTTP Methods**

All HTTP methods are **lowercase** following Rust naming conventions.

```rust
Route::get(path)      // GET
Route::post(path)     // POST
Route::put(path)      // PUT
Route::patch(path)    // PATCH
Route::delete(path)   // DELETE
Route::options(path)  // OPTIONS
Route::head(path)     // HEAD
```

**Rationale:** Rust methods are lowercase (e.g., `Result::ok()`, `Option::some()`). Framework should follow stdlib conventions.

### **Path Syntax**

```rust
"/users"              // Static path
"/users/:id"          // Path parameter
"/users/:id/posts"    // Multiple segments
"/users/:user_id/posts/:post_id"  // Multiple parameters
```

**Rules:**
- Use `:param_name` for parameters
- Use `snake_case` for parameter names
- Keep paths lowercase
- Use plural for collections: `/users` not `/user`
- Use singular for single resource: `/users/:id` not `/users/:ids`

### **Query Parameters**

```rust
ctx.query.get("page")
ctx.query.get("limit")
ctx.query.get("search")
```

**Conventions:**
- Use `snake_case`: `created_at` not `createdAt`
- Short names: `page` not `page_number`
- Boolean flags: `active`, `verified` (not `is_active`)

---

## Response & Error Handling

### **Success Responses**

The `Response<T>` enum provides semantic HTTP success responses:

```rust
pub enum Response<T> {
    Ok(T),              // 200 OK
    Created(T),         // 201 Created
    NoContent,          // 204 No Content
}
```

**Usage:**
```rust
// Return 200 OK
Ok(Response::ok(user))

// Return 201 Created
Ok(Response::created(user))

// Return 204 No Content
Ok(Response::no_content())
```

**Or use Result directly:**
```rust
// Most common: Result with implicit 200
Ok(Json(user))
```

### **Error Responses**

The `Error` type provides semantic HTTP error responses:

```rust
pub enum Error {
    BadRequest { code: String, message: String },
    Unauthorized { code: String, message: String },
    Forbidden { code: String, message: String },
    NotFound { code: String, message: String },
    Conflict { code: String, message: String },
    UnprocessableEntity { code: String, message: String },
    InternalError { code: String, message: String },
}
```

**Usage:**
```rust
// Return 400 Bad Request
Err(Error::bad_request("invalid_input", "Email is required"))

// Return 404 Not Found
Err(Error::not_found("user_not_found", "User not found"))

// Return 500 Internal Server Error
Err(Error::internal("db_error", "Database connection failed"))
```

**Helper Methods:**
```rust
Error::bad_request(code, msg)      // 400
Error::unauthorized(code, msg)     // 401
Error::forbidden(code, msg)        // 403
Error::not_found(code, msg)        // 404
Error::conflict(code, msg)         // 409
Error::unprocessable(code, msg)    // 422
Error::internal(code, msg)         // 500
```

### **Result Pattern**

**Standard pattern for handlers:**
```rust
type Response = Result<Json<T>, Error>;

async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
    let data = fetch_data().await?;  // Use ? operator
    Ok(Json(data))
}
```

**Benefits:**
- Use `?` operator for error propagation
- Standard Rust idioms
- Clean, readable code
- Automatic HTTP status mapping

---

## Builder Patterns

### **Fluent Builder Convention**

All builders use **method chaining** with consistent patterns:

```rust
Meta::new()
    .summary("Short description")
    .describe("Long detailed description")
    .tag("users")
    .tag("authentication")
    .deprecated()
```

### **Builder Method Types**

#### **1. Required Configuration**

No prefix, chainable:
```rust
.route(path)
.summary(text)
.tag(name)
```

#### **2. Optional Configuration**

Use `with_` prefix:
```rust
.with_state(state)
.with_config(config)
.with_timeout(duration)
```

#### **3. Boolean Flags**

No prefix, no arguments:
```rust
.required()
.deprecated()
.optional()
```

#### **4. Accumulative Methods**

Can be called multiple times:
```rust
.tag("users")
.tag("admin")

.middleware(cors)
.middleware(auth)
```

### **Builder Examples**

**Route Building:**
```rust
Route::get("/users/:id")
    .param("id", "User identifier")
    .deprecated()
```

**Meta Building:**
```rust
Meta::new()
    .summary("Create a new user")
    .describe("Creates a user account with the provided information...")
    .tag("users")
    .tag("registration")
```

**Server Building:**
```rust
Server::new()
    .with_config(config)
    .with_state(app_state)
    .register(GetUser)
    .register(CreateUser)
    .middleware(cors)
    .middleware(auth)
    .serve()
    .await
```

---

## Naming Rules

### **Variables**

```rust
// Good
let user = fetch_user().await?;
let user_id = ctx.path.get::<i64>("id")?;
let config = AppConfig::new();

// Bad
let u = fetch_user().await?;
let id = ctx.path.get::<i64>("id")?;  // Too generic
let cfg = AppConfig::new();
```

**Rules:**
- Descriptive names: `user` not `u`
- Context-aware: `user_id` not just `id` in broad scope
- No abbreviations unless common: `config` ok, `cfg` not ok

### **Constants**

```rust
// Good
const DEFAULT_PORT: u16 = 3000;
const MAX_CONNECTIONS: usize = 100;

// Bad
const Port: u16 = 3000;
const max_conn: usize = 100;
```

**Rules:**
- `SCREAMING_SNAKE_CASE`
- Descriptive names
- Use `DEFAULT_` prefix for defaults

### **Endpoint Structs**

```rust
// Good
pub struct GetUser;
pub struct CreateUser;
pub struct UpdateUserProfile;
pub struct DeleteUserAccount;

// Bad
pub struct GetUserEndpoint;     // Redundant suffix
pub struct UsersGet;            // Wrong order
pub struct user_get;            // Wrong case
```

**Rules:**
- `PascalCase`
- Verb + Noun: `GetUser`, `CreatePost`
- No "Endpoint" suffix (implied by Endpoint trait)
- Singular resource: `GetUser` not `GetUsers`

### **Module Names**

```rust
// Good
mod user;
mod auth;
mod middleware;

// Bad
mod User;
mod AUTH;
mod middle_ware;
```

**Rules:**
- `snake_case`
- Singular: `user` not `users` (module contains multiple items)
- Short but clear

---

## Migration from v0.2.x

### **Breaking Changes**

| v0.2.x | v0.3.0 | Migration |
|--------|--------|-----------|
| `API` trait | `Handler` | Rename trait implementation |
| `.ep()` method | `.route()` | Rename method |
| `.docs()` method | `.meta()` | Rename method |
| `.handler()` method | `.handle()` | Rename method |
| `ApiResponse<T>` | `Response<T>` | Rename type |
| `ApiError` | `Error` | Rename type |
| `PathParams` | `Path` | Rename type |
| `QueryParams` | `Query` | Rename type |
| `Route::GET()` | `Route::get()` | Lowercase method |
| `Route::POST()` | `Route::post()` | Lowercase method |
| Optional `docs()` | Required `meta()` | Always return Meta |

### **Migration Examples**

#### **Before (v0.2.x)**

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct GetUserEndpoint;

impl Endpoint for GetUserEndpoint {
    fn ep(&self) -> Route {
        Route::GET("/users/:id")
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
        let id = ctx.path.get("id").unwrap();
        ApiResponse::Ok(fetch_user(id).await)
    }
}
```

#### **After (v0.3.0)**

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct GetUser;

impl Endpoint for GetUser {
    fn route(&self) -> Route {
        Route::get("/users/:id")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Get user by ID")
            .tag("users")
    }
}

#[async_trait]
impl Handler for GetUser {
    type Request = ();
    type Response = Result<Json<User>, Error>;

    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
        let id = ctx.path.get::<i64>("id")?;
        let user = fetch_user(id).await?;
        Ok(Json(user))
    }
}
```

### **Key Improvements**

1. ✅ Clearer trait names: `Handler` instead of `API`
2. ✅ Explicit method names: `.route()` instead of `.ep()`
3. ✅ Always provide metadata: `meta()` returns `Meta`, not `Option<Meta>`
4. ✅ Lowercase HTTP methods: `Route::get()` follows Rust conventions
5. ✅ Result-based responses: Use `?` operator, clean error handling
6. ✅ Type-safe parameter extraction: `.get::<i64>("id")?`
7. ✅ Simpler type names: `Response`, `Error` without redundant prefixes

---

## Appendix: Quick Reference

### **Complete Example**

```rust
use uncovr::prelude::*;

// Application State
#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

// Request/Response Types
#[derive(Deserialize, JsonSchema, Default)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// Endpoint Definition
#[derive(Clone)]
pub struct CreateUser;

impl Endpoint for CreateUser {
    fn route(&self) -> Route {
        Route::post("/users")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Create a new user")
            .describe("Creates a user account with the provided information")
            .tag("users")
    }
}

#[async_trait]
impl Handler for CreateUser {
    type Request = CreateUserRequest;
    type Response = Result<Json<UserResponse>, Error>;

    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
        // Get state
        let state = ctx.state::<AppState>();
        
        // Validate (automatic with validation framework)
        // Business logic
        let user = sqlx::query_as!(
            UserResponse,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
            ctx.request.name,
            ctx.request.email
        )
        .fetch_one(&state.db)
        .await
        .map_err(|e| Error::internal("db_error", &e.to_string()))?;

        Ok(Json(user))
    }
}

// Server Setup
#[tokio::main]
async fn main() {
    let config = AppConfig::new("My API", "1.0.0")
        .environment(Environment::Development);

    let state = AppState {
        db: create_pool().await,
    };

    Server::new()
        .with_config(config)
        .with_state(state)
        .register(CreateUser)
        .serve()
        .await
        .expect("Server failed");
}
```

### **Terminology Glossary**

| Term | Definition |
|------|------------|
| **Endpoint** | A single API route with handler logic |
| **Route** | HTTP method + path definition |
| **Meta** | Metadata for OpenAPI documentation |
| **Handler** | Logic that processes requests |
| **Context** | Request data container |
| **State** | Shared application state |
| **Middleware** | Request/response interceptor |
| **Error** | HTTP error response |
| **Response** | HTTP success response |

---

## Contributing

To propose changes to this dictionary:

1. Open an issue with `[DICTIONARY]` prefix
2. Provide rationale for the change
3. Show examples of current vs proposed naming
4. Discuss impact on existing code

**Approval required from:** Framework maintainers

---

**Last Updated:** 2025-11-11  
**Next Review:** After v0.3.0 release  
**Maintained by:** Uncovr Core Team
