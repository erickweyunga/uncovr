# Routes

Understanding how routing works in Uncovr.

## What are Routes?

Routes define how your API responds to HTTP requests. Each route maps a URL path and HTTP method to a specific handler function.

In Uncovr, routes are defined through the `Metadata` trait, which specifies:

- **Path**: The URL path (e.g., `/users`, `/posts/:id`)
- **Method**: The HTTP method (e.g., `get`, `post`, `put`, `delete`, `patch`)
- **Summary**: Optional description for API documentation

## Basic Route Definition

Every endpoint in Uncovr implements the `Metadata` trait:

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
```

This creates a route that:
- Responds to GET requests
- At the path `/`
- With the description "Say hello" in the API docs

## HTTP Methods

Uncovr supports all standard HTTP methods:

### GET - Retrieve Data

```rust
impl Metadata for ListUsers {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")
            .summary("List all users")
    }
}
```

### POST - Create Data

```rust
impl Metadata for CreateUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")
    }
}
```

### PUT - Update/Replace Data

```rust
impl Metadata for UpdateUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "put")
            .summary("Update a user")
    }
}
```

### PATCH - Partially Update Data

```rust
impl Metadata for PatchUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "patch")
            .summary("Partially update a user")
    }
}
```

### DELETE - Remove Data

```rust
impl Metadata for DeleteUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "delete")
            .summary("Delete a user")
    }
}
```

## Path Parameters

Use `:param` syntax to capture values from the URL:

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

**Request:**
```bash
curl http://localhost:3000/users/42
```

The value `42` is automatically extracted and available as `ctx.req.id`.

### Multiple Path Parameters

You can have multiple parameters in a path:

```rust
impl Metadata for GetPostComment {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/posts/:post_id/comments/:comment_id", "get")
            .summary("Get a specific comment on a post")
    }
}

#[derive(Default, Deserialize, JsonSchema)]
pub struct GetCommentRequest {
    post_id: u64,
    comment_id: u64,
}
```

## Route Organization

### Single File per Endpoint

Organize related routes in separate files:

```
src/
└── endpoints/
    ├── mod.rs
    ├── list_users.rs
    ├── create_user.rs
    ├── get_user.rs
    ├── update_user.rs
    └── delete_user.rs
```

### Grouping by Resource

Group related endpoints together:

```
src/
└── endpoints/
    ├── mod.rs
    ├── users/
    │   ├── mod.rs
    │   ├── list.rs
    │   ├── create.rs
    │   ├── get.rs
    │   └── delete.rs
    └── posts/
        ├── mod.rs
        ├── list.rs
        └── create.rs
```

## Registering Routes

Register routes with the server in your `main.rs`:

```rust
use uncovr::prelude::*;

mod endpoints;

#[tokio::main]
async fn main() {
    let config = AppConfig::new("My API", "1.0.0");

    Server::new()
        .with_config(config)
        .register(endpoints::ListUsers)
        .register(endpoints::CreateUser)
        .register(endpoints::GetUser)
        .register(endpoints::UpdateUser)
        .register(endpoints::DeleteUser)
        .serve()
        .await
        .expect("Server failed");
}
```

The order of registration doesn't matter - Uncovr will route requests correctly based on the paths and methods.

## Route Conflicts

### Same Path, Different Methods

This is valid and common:

```rust
// GET /users - List users
impl Metadata for ListUsers {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")
    }
}

// POST /users - Create user
impl Metadata for CreateUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
    }
}
```

### Same Path and Method

Avoid registering the same path and method twice:

```rust
// Bad: Conflict!
impl Metadata for Endpoint1 {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")  // Conflict
    }
}

impl Metadata for Endpoint2 {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")  // Conflict
    }
}
```

The last registered route will handle the requests.

## Path Matching

Routes are matched in this order:

1. **Exact matches**: `/users/new`
2. **Path parameters**: `/users/:id`
3. **Wildcards**: `/users/*path`

Example:

```rust
// This will match /users/new exactly
Endpoint::new("/users/new", "get")

// This will match /users/123 but not /users/new
Endpoint::new("/users/:id", "get")
```

## Query Parameters

Query parameters are not part of the route path. Handle them in your request type:

```rust
#[derive(Default, Deserialize, JsonSchema)]
pub struct ListUsersRequest {
    page: Option<u32>,
    limit: Option<u32>,
}

impl Metadata for ListUsers {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")
            .summary("List users with pagination")
    }
}
```

**Request:**
```bash
curl http://localhost:3000/users?page=2&limit=10
```

Query parameters are automatically parsed into your request type.

## Nested Routes

You can nest routers under a common prefix:

```rust
// Create a router for user endpoints
let users_router = Server::new()
    .register(ListUsers)
    .register(CreateUser)
    .register(GetUser)
    .build()
    .into_router();

// Nest it under /api/v1
Server::new()
    .nest("/api/v1", users_router)
    .serve()
    .await;
```

Now your routes are:
- `/api/v1/users` (GET) - List users
- `/api/v1/users` (POST) - Create user
- `/api/v1/users/:id` (GET) - Get user

## Root Path

The root path `/` is just another route:

```rust
impl Metadata for Home {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/", "get")
            .summary("API home")
    }
}
```

## Best Practices

### 1. RESTful Conventions

Follow REST conventions for intuitive APIs:

```rust
GET    /users       // List all users
POST   /users       // Create a user
GET    /users/:id   // Get a specific user
PUT    /users/:id   // Update a user
DELETE /users/:id   // Delete a user
```

### 2. Resource Hierarchy

Represent relationships in paths:

```rust
GET /users/:user_id/posts           // User's posts
GET /users/:user_id/posts/:post_id  // Specific post
```

### 3. Clear Summaries

Write descriptive summaries for documentation:

```rust
impl Metadata for GetUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "get")
            .summary("Get a user by their unique ID")
    }
}
```

### 4. Versioning

Include version in the path when needed:

```rust
Endpoint::new("/v1/users", "get")
Endpoint::new("/v2/users", "get")
```

Or use nesting:

```rust
let v1 = Server::new()
    .register(V1Users)
    .build()
    .into_router();

let v2 = Server::new()
    .register(V2Users)
    .build()
    .into_router();

Server::new()
    .nest("/v1", v1)
    .nest("/v2", v2)
    .serve()
    .await;
```

## Technical Details

Under the hood, Uncovr uses:

- **Axum** for routing and HTTP handling
- **Aide** for OpenAPI integration
- **Tower** for middleware

Routes are compiled at application startup into an efficient routing tree. Path parameters are extracted automatically and passed to your handlers through the `Context`.
