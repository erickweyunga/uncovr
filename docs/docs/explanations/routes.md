# Routes

Understanding how routing works in Uncovr and how to build structured APIs with the new Endpoint API.

## The Concept of Routing

When a client makes an HTTP request to your API, the router's job is to determine which piece of code should handle that request. This decision is based on two factors: the URL path and the HTTP method.

Think of routing like a postal system. The URL path is the address, and the HTTP method (GET, POST, etc.) is the type of mail service. The router looks at both and delivers the request to the correct handler.

## How Routes Work in Uncovr

In Uncovr, you define routes through the `Endpoint` trait. This trait separates routing concerns from documentation:

1. **`ep()` method**: Defines the route (path, method, parameters)
2. **`docs()` method**: Provides optional API documentation

Here's a complete example:

```rust
use uncovr::prelude::*;
use uncovr::server::endpoint::{Endpoint, Route, Docs};

#[derive(Clone)]
pub struct GetUser;

impl Endpoint for GetUser {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/users/:id");
        route.path_param("id").desc("User ID");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Retrieve a user by ID")
                .description("Fetches user information from the database")
                .tag("users")
        )
    }
}
```

When you register this endpoint, Uncovr knows: "When someone makes a GET request to `/users/:id`, call this handler."

## HTTP Methods - Type-Safe Constructors

Uncovr uses type-safe HTTP method constructors instead of strings. This prevents typos and provides better IDE support.

### GET - Retrieving Data

GET requests fetch data without modifying anything on the server. They should be safe to call multiple times with the same result.

```rust
impl Endpoint for ListUsers {
    fn ep(&self) -> Route {
        Route::GET("/users")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("List all users"))
    }
}
```

**Key characteristic**: Idempotent and safe. Calling it 100 times has the same effect as calling it once.

**Common use cases**:
- Fetching a single resource (`/users/:id`)
- Listing multiple resources (`/users`)
- Searching or filtering (`/users?role=admin`)

### POST - Creating New Resources

POST creates a new resource. The server typically assigns a new ID and returns the created resource.

```rust
impl Endpoint for CreateUser {
    fn ep(&self) -> Route {
        Route::POST("/users")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Create a new user")
                .tag("users")
                .responses(|op| {
                    op.response::<201, Json<UserResponse>>()
                      .response::<400, Json<ErrorResponse>>()
                })
        )
    }
}
```

**Key characteristic**: Not idempotent. Calling it twice creates two resources.

**Common use cases**:
- Creating new resources
- Submitting forms
- Starting new processes
- Uploading files

### PUT - Replacing a Resource

PUT replaces an entire resource. You're saying "make this resource look exactly like what I'm sending."

```rust
impl Endpoint for ReplaceUser {
    fn ep(&self) -> Route {
        let mut route = Route::PUT("/users/:id");
        route.path_param("id").required().desc("User ID");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Replace user data completely"))
    }
}
```

**Key characteristic**: Idempotent. Sending the same PUT twice results in the same final state.

**When to use PUT vs PATCH**:
- Use PUT when you want to replace the entire resource
- Use PATCH when you want to update specific fields

### PATCH - Partial Updates

PATCH modifies part of a resource without replacing the whole thing.

```rust
impl Endpoint for UpdateUser {
    fn ep(&self) -> Route {
        let mut route = Route::PATCH("/users/:id");
        route.path_param("id").required();
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Update specific user fields"))
    }
}
```

**Key characteristic**: Not necessarily idempotent, depends on the patch operations.

**Example request body**:
```json
{
  "name": "New Name"
}
```

Only the `name` field is updated; other fields remain unchanged.

### DELETE - Removing Resources

DELETE removes a resource from the system.

```rust
impl Endpoint for DeleteUser {
    fn ep(&self) -> Route {
        let mut route = Route::DELETE("/users/:id");
        route.path_param("id").required().desc("User ID to delete");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Remove a user")
                .responses(|op| {
                    op.response::<204, ()>()
                      .response::<404, Json<ErrorResponse>>()
                })
        )
    }
}
```

**Key characteristic**: Idempotent. Deleting twice has the same effect as deleting once (the resource is gone).

**Response patterns**:
- `204 No Content` - Successfully deleted, no response body
- `404 Not Found` - Resource doesn't exist
- `200 OK` - Successfully deleted, returns the deleted resource

### OPTIONS and HEAD

```rust
// OPTIONS - Describe available operations
Route::OPTIONS("/users")

// HEAD - Like GET but returns only headers
Route::HEAD("/users")
```

These are less commonly used but available when needed.

## Working with Path Parameters

Path parameters let you capture values directly from the URL. They're declared in the path using the `:name` syntax.

### Basic Path Parameters

```rust
impl Endpoint for GetUserPost {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/users/:user_id/posts/:post_id");
        route.path_param("user_id").desc("The user's ID");
        route.path_param("post_id").desc("The post's ID");
        route
    }
}
```

**Extracting path parameters in handlers**:

```rust
#[async_trait]
impl API for GetUserPost {
    type Req = ();
    type Res = Json<PostResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let user_id = ctx.path.get_u64("user_id").unwrap_or(0);
        let post_id = ctx.path.get_u64("post_id").unwrap_or(0);
        
        // Fetch the post...
        Json(post)
    }
}
```

### Path Parameter Types

The `PathParams` API provides type-safe extraction:

```rust
// Extract as different types
let id = ctx.path.get_u64("id");          // Option<u64>
let name = ctx.path.get("name");          // Option<&str>
let is_active = ctx.path.get_bool("active"); // Option<bool>
```

### Required vs Optional Parameters

Path parameters are typically required (they're part of the route), but you can document this:

```rust
fn ep(&self) -> Route {
    let mut route = Route::GET("/users/:id");
    route.path_param("id").required().desc("User ID");
    route
}
```

## Working with Query Parameters

Query parameters are key-value pairs in the URL after the `?` symbol: `/users?page=1&limit=10`

### Defining Query Parameters

```rust
impl Endpoint for ListUsers {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/users");
        route.query("page").desc("Page number (default: 1)");
        route.query("limit").desc("Items per page (default: 10)");
        route.query("role").desc("Filter by role");
        route
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("List users with pagination"))
    }
}
```

### Extracting Query Parameters

```rust
#[async_trait]
impl API for ListUsers {
    type Req = ();
    type Res = Json<Vec<User>>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let page = ctx.query.get_u32("page").unwrap_or(1);
        let limit = ctx.query.get_u32("limit").unwrap_or(10);
        let role = ctx.query.get("role"); // Option<&str>
        
        // Apply filters...
        Json(users)
    }
}
```

### Required Query Parameters

Mark query parameters as required for better documentation:

```rust
fn ep(&self) -> Route {
    let mut route = Route::GET("/search");
    route.query("q").required().desc("Search query");
    route.query("limit").desc("Max results (optional)");
    route
}
```

### Query Parameter Types

```rust
// Different extraction methods
let page = ctx.query.get_u32("page");      // Option<u32>
let limit = ctx.query.get_u64("limit");    // Option<u64>
let active = ctx.query.get_bool("active"); // Option<bool>
let name = ctx.query.get("name");          // Option<&str>
```

## Route Patterns and Best Practices

### RESTful Resource Naming

Follow REST conventions for consistent APIs:

```rust
// Collection operations
Route::GET("/users")      // List all users
Route::POST("/users")     // Create a user

// Single resource operations
Route::GET("/users/:id")     // Get one user
Route::PUT("/users/:id")     // Replace user
Route::PATCH("/users/:id")   // Update user
Route::DELETE("/users/:id")  // Delete user

// Nested resources
Route::GET("/users/:user_id/posts")           // User's posts
Route::POST("/users/:user_id/posts")          // Create post for user
Route::GET("/users/:user_id/posts/:post_id")  // Specific post
```

### Versioning

Version your API in the path:

```rust
Route::GET("/v1/users")
Route::GET("/v2/users")
```

Or use a version prefix when registering:

```rust
Server::new()
    .with_config(config)
    .nest("/v1", v1_routes)
    .nest("/v2", v2_routes)
```

### Action Routes

Sometimes you need actions beyond CRUD:

```rust
// Prefer POST for actions
Route::POST("/users/:id/activate")
Route::POST("/users/:id/reset-password")
Route::POST("/orders/:id/cancel")
```

### Search and Filtering

Use query parameters for search and filtering:

```rust
impl Endpoint for SearchUsers {
    fn ep(&self) -> Route {
        let mut route = Route::GET("/users/search");
        route.query("q").required().desc("Search query");
        route.query("role").desc("Filter by role");
        route.query("status").desc("Filter by status");
        route
    }
}
```

**Example requests**:
```
GET /users/search?q=john&role=admin
GET /users/search?q=doe&status=active
```

## Route Organization

### Feature-Based Structure

Organize routes by feature for better maintainability:

```
src/
├── users/
│   ├── mod.rs
│   ├── apis.rs       # Route definitions
│   └── handlers.rs   # Business logic
├── posts/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
└── main.rs
```

### Grouping with Tags

Use tags in documentation to group related endpoints:

```rust
impl Endpoint for CreateUser {
    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("Create user")
                .tag("users")
                .tag("admin")
        )
    }
}
```

Tags appear in the OpenAPI documentation, making it easier to navigate large APIs.

## Advanced Routing Patterns

### Nested Routers

Split your API into logical sections:

```rust
// User routes
let user_routes = Server::new()
    .register(GetUser)
    .register(CreateUser)
    .build()
    .into_router();

// Post routes
let post_routes = Server::new()
    .register(GetPost)
    .register(CreatePost)
    .build()
    .into_router();

// Combine
Server::new()
    .with_config(config)
    .nest("/users", user_routes)
    .nest("/posts", post_routes)
    .serve()
    .await
```

Results in:
- `/users/*` - User endpoints
- `/posts/*` - Post endpoints

### Middleware Per Route Group

Apply middleware to specific route groups:

```rust
use uncovr::tower::Layer;
use uncovr::axum_middleware::from_fn;

// Public routes (no auth)
let public_routes = Server::new()
    .register(Login)
    .register(Register)
    .build()
    .into_router();

// Protected routes (with auth)
let protected_routes = Server::new()
    .register(GetProfile)
    .register(UpdateProfile)
    .layer(from_fn(auth_middleware))
    .build()
    .into_router();

Server::new()
    .with_config(config)
    .nest("/auth", public_routes)
    .nest("/api", protected_routes)
    .serve()
    .await
```

## Common Routing Mistakes to Avoid

### 1. Mixing Plural and Singular

**Bad**:
```rust
Route::GET("/user/:id")    // Singular
Route::GET("/posts")       // Plural
```

**Good**:
```rust
Route::GET("/users/:id")   // Consistent plural
Route::GET("/posts")
```

### 2. Using Verbs in URLs

**Bad**:
```rust
Route::POST("/users/create")
Route::POST("/users/delete/:id")
```

**Good**:
```rust
Route::POST("/users")
Route::DELETE("/users/:id")
```

The HTTP method provides the verb!

### 3. Deep Nesting

**Bad**:
```rust
Route::GET("/users/:user_id/posts/:post_id/comments/:comment_id/likes")
```

**Good**:
```rust
Route::GET("/comments/:comment_id/likes")
```

Keep nesting to 2-3 levels maximum.

### 4. Forgetting Parameter Descriptions

**Bad**:
```rust
fn ep(&self) -> Route {
    let mut route = Route::GET("/users/:id");
    route.path_param("id");  // No description
    route
}
```

**Good**:
```rust
fn ep(&self) -> Route {
    let mut route = Route::GET("/users/:id");
    route.path_param("id").desc("User's unique identifier");
    route
}
```

Descriptions appear in OpenAPI documentation!

## Testing Routes

Test your routes with standard HTTP clients:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_user() {
        let app = Server::new()
            .register(GetUser)
            .build()
            .into_router();
            
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/123")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();
            
        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

## Summary

**Type-Safe Methods**: Use `Route::GET()`, `Route::POST()`, etc. instead of strings.

**Separation of Concerns**: Define routes in `ep()`, documentation in `docs()`.

**Path Parameters**: Capture dynamic values with `:name` syntax.

**Query Parameters**: Use for filtering, pagination, and optional data.

**RESTful Conventions**: Follow standard patterns for intuitive APIs.

**Organization**: Group routes by feature and use tags for documentation.

**Parameter Documentation**: Always describe parameters for better OpenAPI docs.

The new Endpoint API makes routing cleaner, more type-safe, and easier to maintain while keeping your documentation separate and optional.