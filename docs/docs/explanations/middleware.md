# Middleware

How to intercept requests and responses to add cross-cutting functionality to your API.

## The Middleware Problem

Every real-world API needs functionality that applies across multiple endpoints: authentication, logging, rate limiting, request validation, and more. Without middleware, you'd duplicate this code in every handler.

Middleware solves this by letting you intercept requests before they reach your handlers and responses before they're sent to clients. Uncovr uses Axum's native middleware system, giving you the full power of Tower's composable middleware layers.

## How Middleware Works

Think of middleware as a chain of functions that wrap your handlers:

```
Request → Middleware 1 → Middleware 2 → Handler → Middleware 2 → Middleware 1 → Response
```

Each middleware can:
- Inspect and modify the request before it reaches the handler
- Inject data into the request (via extensions)
- Short-circuit the chain and return early
- Inspect and modify the response after the handler runs
- Handle errors and perform cleanup

The key insight: middleware runs **before and after** your handler, giving you control over the entire request-response cycle.

## Middleware vs Handlers

**Handlers** contain your business logic. They receive a `Context` and return a response.

**Middleware** contains cross-cutting concerns. They receive a `Request`, can pass data to handlers via extensions, and transform responses.

Don't put business logic in middleware. Keep middleware focused on infrastructure concerns that apply broadly across your API.

## Basic Middleware Pattern

Here's the fundamental pattern for custom middleware:

```rust
async fn my_middleware(
    request: Request,
    next: Next,
) -> Response {
    // Do something with the request
    println!("Before handler");
    
    // Call the next middleware/handler in the chain
    let response = next.run(request).await;
    
    // Do something with the response
    println!("After handler");
    
    response
}
```

The `next.run(request)` call is crucial - it passes control to the next middleware or your handler. Everything before this line runs before your handler. Everything after runs after.

## Passing Data to Handlers

The most powerful middleware pattern is injecting data into requests that handlers can access. This is how authentication, user context, and other request-scoped data flows through your API.

Uncovr uses **extensions** - a type-safe way to attach arbitrary data to requests:

```rust
#[derive(Clone)]
struct AuthUser {
    user_id: i64,
    email: String,
}

async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // Extract and validate auth token
    let token = extract_token(&request);
    let user = validate_token(token);
    
    // Inject user into request
    request.extensions_mut().insert(user);
    
    // Continue to handler
    next.run(request).await
}
```

In your handler, extract the data from context:

```rust
async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    // Get the user injected by middleware
    let user = ctx.extensions.get::<AuthUser>();
    
    match user {
        Some(user) => {
            // User is authenticated, proceed
            ApiResponse::Ok(format!("Hello, {}", user.email))
        }
        None => {
            // No user found, reject
            ApiResponse::Unauthorized {
                code: "not_authenticated",
                message: "Authentication required",
            }
        }
    }
}
```

This pattern keeps authentication logic out of your handlers. Your handlers simply check if a user exists in extensions - the middleware handles all the token validation complexity.

## Applying Middleware

Use the `.layer()` method to add middleware to your server or specific routes:

```rust
use uncovr::axum_middleware::from_fn;

// Apply to specific routes
let protected_routes = Server::new()
    .register(GetUserEndpoint::new(state.clone()))
    .register(UpdateUserEndpoint::new(state.clone()))
    .layer(from_fn(auth_middleware))
    .build()
    .into_router();

// Public routes without middleware
let public_routes = Server::new()
    .register(LoginEndpoint::new(state.clone()))
    .register(RegisterEndpoint::new(state.clone()))
    .build()
    .into_router();

// Combine them
Server::new()
    .with_config(config)
    .nest("/api", protected_routes)
    .nest("/auth", public_routes)
    .serve()
    .await
```

This creates two separate route groups: protected routes with authentication middleware, and public routes without it.

## Authentication Middleware Pattern

Authentication is the most common middleware use case. Here's the pattern:

**1. Extract credentials from the request** (headers, cookies, etc.)

**2. Validate credentials** (check token, verify signature, query database)

**3. On success:** inject user data into extensions and continue

**4. On failure:** return an error response immediately

```rust
async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract token from Authorization header
    let token = match headers.get("authorization") {
        Some(header) => header.to_str().ok(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "missing_token",
                    "message": "Authorization header required"
                }))
            ).into_response();
        }
    };
    
    // Validate token
    let user = match validate_token(token) {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "invalid_token",
                    "message": "Invalid or expired token"
                }))
            ).into_response();
        }
    };
    
    // Inject authenticated user
    request.extensions_mut().insert(user);
    
    // Continue to handler
    next.run(request).await
}
```

The key: middleware can short-circuit the request chain by returning a response without calling `next.run()`.

## Request Transformation

Middleware can modify requests before they reach handlers:

```rust
async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // Add a unique request ID
    let request_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(RequestId(request_id.clone()));
    
    // Add to headers for the handler to access
    request.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap()
    );
    
    next.run(request).await
}
```

Now every request has a unique ID that can be used for logging, tracing, and error correlation.

## Response Transformation

Middleware can modify responses after handlers return:

```rust
async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    // Add security headers to every response
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    
    response
}
```

This ensures security headers are added to all responses without touching individual handlers.

## Error Handling in Middleware

Middleware can catch errors and transform them:

```rust
async fn error_handler_middleware(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    // Check if the response is an error
    if response.status().is_server_error() {
        // Log the error
        tracing::error!("Server error: {:?}", response.status());
        
        // Return a standardized error response
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "code": "internal_error",
                "message": "An unexpected error occurred"
            }))
        ).into_response();
    }
    
    response
}
```

This creates a safety net that catches unexpected errors and returns consistent error responses.

## Timing and Logging

Measure request duration and log request details:

```rust
async fn timing_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    // Process request
    let response = next.run(request).await;
    
    // Log timing
    let duration = start.elapsed();
    tracing::info!(
        "{} {} - {} - {:?}",
        method,
        uri,
        response.status(),
        duration
    );
    
    response
}
```

Every request gets logged with its method, path, status code, and duration.

## State Access in Middleware

If you need to access application state in middleware, use extractors:

```rust
async fn rate_limit_middleware(
    Extension(state): Extension<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let user_ip = get_user_ip(&request);
    
    // Check rate limit using state
    if state.rate_limiter.is_exceeded(user_ip) {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({
                "code": "rate_limit_exceeded",
                "message": "Too many requests"
            }))
        ).into_response();
    }
    
    next.run(request).await
}
```

State is shared across all middleware and handlers via Axum's Extension layer.

## Middleware Ordering

Order matters. Middleware executes in the order you add it:

```rust
Server::new()
    .register(endpoint)
    .layer(from_fn(timing_middleware))      // Runs first (outer)
    .layer(from_fn(auth_middleware))        // Runs second
    .layer(from_fn(rate_limit_middleware))  // Runs third (inner)
```

For a request:
1. Timing middleware runs (starts timer)
2. Auth middleware runs (validates token)
3. Rate limit middleware runs (checks quota)
4. Handler runs
5. Rate limit middleware completes
6. Auth middleware completes
7. Timing middleware completes (logs duration)

Think of it like nested function calls - the first middleware added is the outermost wrapper.

## Conditional Middleware

Apply middleware only to certain routes by creating separate route groups:

```rust
// Routes that need auth
let protected = Server::new()
    .register(GetUserEndpoint::new())
    .register(UpdateUserEndpoint::new())
    .layer(from_fn(auth_middleware));

// Routes that don't need auth
let public = Server::new()
    .register(LoginEndpoint::new())
    .register(RegisterEndpoint::new());

// Admin routes with extra middleware
let admin = Server::new()
    .register(AdminEndpoint::new())
    .layer(from_fn(auth_middleware))
    .layer(from_fn(admin_check_middleware));

// Combine with different paths
Server::new()
    .with_config(config)
    .nest("/api", protected.build().into_router())
    .nest("/auth", public.build().into_router())
    .nest("/admin", admin.build().into_router())
    .serve()
    .await
```

This gives you fine-grained control over which routes get which middleware.

## Tower Middleware

Uncovr exposes Tower middleware for advanced use cases:

```rust
use uncovr::tower::{ServiceBuilder, Layer};
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;

Server::new()
    .register(endpoint)
    .layer(
        ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
            .layer(from_fn(auth_middleware))
    )
```

Tower provides battle-tested middleware for timeouts, rate limiting, compression, and more.

## Common Middleware Patterns

**Authentication**: Validate tokens, inject user data
```rust
Extract token → Validate → Insert user → Continue
```

**Authorization**: Check permissions after authentication
```rust
Get user from extensions → Check permissions → Allow or reject
```

**Request ID**: Track requests across your system
```rust
Generate UUID → Add to extensions → Add to headers → Continue
```

**Logging**: Record request details
```rust
Start timer → Continue → Log method/path/status/duration
```

**Rate Limiting**: Prevent abuse
```rust
Get identifier → Check quota → Allow or reject
```

**Error Handling**: Standardize error responses
```rust
Continue → Catch errors → Transform to standard format
```

## Best Practices

**Keep middleware focused**: Each middleware should do one thing well. Don't combine authentication, logging, and rate limiting in a single middleware.

**Order carefully**: Authentication before authorization. Logging as the outermost layer to capture everything. Rate limiting early to reject bad requests fast.

**Use extensions for data flow**: Don't use global state or thread locals. Extensions give you type-safe, request-scoped data.

**Fail fast**: Reject invalid requests in middleware before they reach handlers. Save handler code for business logic.

**Log appropriately**: Middleware is perfect for access logs and timing. Don't log sensitive data like tokens or passwords.

**Test middleware separately**: Write tests for your middleware independent of your handlers. Middleware is infrastructure - test it like infrastructure.

## When NOT to Use Middleware

**Business logic**: Middleware is for cross-cutting concerns, not business rules. If it's specific to one endpoint, put it in the handler.

**Complex transformations**: If you need to deeply transform request bodies based on content, do it in the handler where you have full type information.

**Response building**: Don't build complex responses in middleware. Middleware should wrap responses, not create them from scratch.

**State mutations**: Avoid complex state changes in middleware. Keep middleware simple and side-effect free when possible.

## Complete Example

Here's a complete authentication middleware with all the pieces:

```rust
// The user data structure
#[derive(Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub email: String,
}

// Middleware function
pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract Authorization header
    let auth_header = match headers.get("authorization") {
        Some(header) => match header.to_str() {
            Ok(h) => h,
            Err(_) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "code": "invalid_header",
                        "message": "Invalid authorization header"
                    })),
                ).into_response();
            }
        },
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "missing_token",
                    "message": "Authorization header required"
                })),
            ).into_response();
        }
    };

    // Extract token from "Bearer <token>"
    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "invalid_format",
                    "message": "Authorization header must be 'Bearer <token>'"
                })),
            ).into_response();
        }
    };

    // Validate token (your validation logic here)
    let claims = match validate_jwt_token(token) {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "code": "invalid_token",
                    "message": "Invalid or expired token"
                })),
            ).into_response();
        }
    };

    // Create user from claims
    let user = AuthUser {
        user_id: claims.user_id,
        email: claims.email,
    };

    // Insert user into request extensions
    request.extensions_mut().insert(user);

    // Continue to handler
    next.run(request).await
}

// In your route setup
let protected_routes = Server::new()
    .register(GetUserEndpoint::new(state.clone()))
    .register(WhoAmIEndpoint::new(state.clone()))
    .layer(from_fn(auth_middleware))
    .build()
    .into_router();

// In your handler
async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    let user = match ctx.extensions.get::<AuthUser>() {
        Some(user) => user.clone(),
        None => {
            return ApiResponse::Unauthorized {
                code: "not_authenticated",
                message: "User not authenticated",
            };
        }
    };

    // Use the authenticated user
    ApiResponse::Ok(format!("Hello, {}", user.email))
}
```

This pattern - middleware injecting data that handlers extract from extensions - is the foundation for building secure, maintainable APIs with Uncovr.