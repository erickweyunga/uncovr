# Nested Services Example

This example demonstrates how to nest different services in Uncovr, including:

1. **API Routes** - Standard Uncovr endpoints
2. **Static File Serving** - Using `tower_http::services::ServeDir`
3. **Custom Services** - Any Tower-compatible service
4. **Version Prefixes** - Organizing APIs with `/v1`, `/v2` prefixes

## What You'll Learn

- How to use `nest()` for nesting Uncovr routers
- How to use `nest_service()` for external services
- Organizing APIs by version or feature
- Serving static files alongside your API
- Combining multiple services in one application

## Project Structure

```
src/
├── main.rs           # Server setup with nested services
├── v1/
│   ├── mod.rs
│   └── users.rs      # V1 user endpoints
└── v2/
    ├── mod.rs
    └── users.rs      # V2 user endpoints (improved)
```

## Running the Example

```bash
cargo run
```

Then visit:
- `http://localhost:8000/v1/users` - V1 API
- `http://localhost:8000/v2/users` - V2 API
- `http://localhost:8000/static/` - Static files
- `http://localhost:8000/docs` - Interactive API documentation

## Key Concepts

### Nesting Routers with `nest()`

Use `nest()` to nest Uncovr routers under a path prefix:

```rust
let v1_routes = Server::new()
    .register(GetUserV1)
    .register(CreateUserV1)
    .build()
    .into_router();

Server::new()
    .nest("/v1", v1_routes)  // All routes get /v1 prefix
    .serve()
    .await
```

### Nesting External Services with `nest_service()`

Use `nest_service()` for non-Uncovr services like static file servers:

```rust
use tower_http::services::ServeDir;

Server::new()
    .nest_service("/static", ServeDir::new("public"))
    .serve()
    .await
```

### Combining Both

You can mix and match routers and services:

```rust
Server::new()
    .with_config(config)
    // API routes
    .nest("/v1", v1_routes)
    .nest("/v2", v2_routes)
    // Static files
    .nest_service("/static", ServeDir::new("public"))
    // Health check
    .register(HealthCheck)
    .serve()
    .await
```

## Use Cases

### API Versioning

```rust
// Keep old version working
let v1 = Server::new()
    .register(OldUserAPI)
    .build()
    .into_router();

// New improved version
let v2 = Server::new()
    .register(NewUserAPI)
    .build()
    .into_router();

Server::new()
    .nest("/v1", v1)
    .nest("/v2", v2)
    .serve()
    .await
```

### Feature-Based Organization

```rust
// User management
let users = Server::new()
    .register(GetUser)
    .register(CreateUser)
    .build()
    .into_router();

// Post management  
let posts = Server::new()
    .register(GetPost)
    .register(CreatePost)
    .build()
    .into_router();

Server::new()
    .nest("/users", users)
    .nest("/posts", posts)
    .serve()
    .await
```

### Static Files + API

```rust
use tower_http::services::ServeDir;

Server::new()
    // Serve your frontend
    .nest_service("/", ServeDir::new("dist"))
    // API under /api prefix
    .nest("/api", api_routes)
    .serve()
    .await
```

### Protected Routes

```rust
use uncovr::tower::Layer;
use uncovr::axum_middleware::from_fn;

// Public routes
let public = Server::new()
    .register(Login)
    .register(Register)
    .build()
    .into_router();

// Protected routes with auth middleware
let protected = Server::new()
    .register(GetProfile)
    .register(UpdateProfile)
    .layer(from_fn(auth_middleware))
    .build()
    .into_router();

Server::new()
    .nest("/auth", public)
    .nest("/api", protected)
    .serve()
    .await
```

## Benefits of Nesting

1. **Clean URLs** - Logical grouping with prefixes
2. **Version Management** - Support multiple API versions
3. **Separation of Concerns** - Feature-based organization
4. **Middleware per Section** - Different middleware for different routes
5. **Static + Dynamic** - Serve files and API together
6. **Incremental Migration** - Add new versions without breaking old ones

## Advanced: Custom Tower Services

You can nest any Tower-compatible service:

```rust
use tower::ServiceBuilder;

let custom_service = tower::service_fn(|req| async {
    // Custom logic
    Ok::<_, Infallible>(Response::new("Custom response"))
});

Server::new()
    .nest_service("/custom", custom_service)
    .serve()
    .await
```

## Next Steps

- Add middleware to specific nested routes
- Implement API versioning in your application
- Combine multiple microservices
- Serve a frontend and API together