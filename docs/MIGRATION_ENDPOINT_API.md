# Migration Guide: New Endpoint API

This guide explains how to migrate from the old `Metadata` trait to the new `Endpoint` trait API.

## Overview

The new Endpoint API separates route definition from documentation, making your code cleaner and more maintainable.

**Key Benefits:**
- ✅ Cleaner separation of concerns (routing vs documentation)
- ✅ Optional documentation for rapid prototyping
- ✅ Type-safe HTTP methods (uppercase constructors)
- ✅ Better extensibility for future features
- ✅ More intuitive API design

## Old API (Metadata trait)

```rust
use uncovr::prelude::*;

#[derive(Clone)]
struct GetUsers;

impl Metadata for GetUsers {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")  // ❌ String-based method
            .summary("Get all users")
            .description("Returns a paginated list of users")
            .query("page")
            .query("limit").required()
    }
}

#[async_trait]
impl API for GetUsers {
    type Req = ();
    type Res = Json<Vec<User>>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // handler logic
    }
}
```

## New API (Endpoint trait)

```rust
use uncovr::prelude::*;
use uncovr::server::endpoint::{Endpoint, Route, Docs};

#[derive(Clone)]
struct GetUsers;

impl Endpoint for GetUsers {
    fn ep(&self) -> Route {
        Route::GET("/users")  // ✅ Type-safe uppercase method
            .query("page")
            .query("limit").required()
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new()
            .summary("Get all users")
            .description("Returns a paginated list of users")
            .tag("users"))
    }
}

#[async_trait]
impl API for GetUsers {
    type Req = ();
    type Res = Json<Vec<User>>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // handler logic
    }
}
```

## Key Differences

### 1. HTTP Methods are Uppercase

**Old:**
```rust
Endpoint::new("/users", "get")   // String-based
Endpoint::new("/users", "post")
```

**New:**
```rust
Route::GET("/users")   // Type-safe
Route::POST("/users")
Route::PUT("/users/:id")
Route::DELETE("/users/:id")
Route::PATCH("/users/:id")
```

### 2. Separate Route and Documentation

**Old:** Everything in one method
```rust
fn metadata(&self) -> Endpoint {
    Endpoint::new("/users", "post")
        .summary("Create user")          // Documentation
        .description("...")              // Documentation
        .query("notify")                 // Routing
        .required()                      // Routing
}
```

**New:** Clean separation
```rust
fn ep(&self) -> Route {
    // Only routing concerns
    Route::POST("/users")
        .query("notify").required()
}

fn docs(&self) -> Option<Docs> {
    // Only documentation concerns
    Some(Docs::new()
        .summary("Create user")
        .description("..."))
}
```

### 3. Optional Documentation

The new API makes documentation optional, perfect for rapid prototyping:

```rust
impl Endpoint for QuickTest {
    fn ep(&self) -> Route {
        Route::GET("/test")
    }
    // No docs() implementation needed - defaults to None
}
```

## Complete Migration Examples

### Example 1: Simple GET Endpoint

**Before:**
```rust
impl Metadata for GetHealth {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/health", "get")
            .summary("Health check")
    }
}
```

**After:**
```rust
impl Endpoint for GetHealth {
    fn ep(&self) -> Route {
        Route::GET("/health")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new().summary("Health check"))
    }
}
```

### Example 2: POST with Query Parameters

**Before:**
```rust
impl Metadata for CreateUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")
            .description("Creates a user with the provided information")
            .query("notify").desc("Send notification email")
    }
}
```

**After:**
```rust
impl Endpoint for CreateUser {
    fn ep(&self) -> Route {
        Route::POST("/users")
            .query("notify").desc("Send notification email")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new()
            .summary("Create a new user")
            .description("Creates a user with the provided information")
            .tag("users"))
    }
}
```

### Example 3: DELETE with Path Parameters

**Before:**
```rust
impl Metadata for DeleteUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "delete")
            .summary("Delete a user")
            .path_param("id").required().desc("User ID")
            .query("force").desc("Force deletion")
    }
}
```

**After:**
```rust
impl Endpoint for DeleteUser {
    fn ep(&self) -> Route {
        Route::DELETE("/users/:id")
            .path_param("id").required().desc("User ID")
            .query("force").desc("Force deletion")
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new()
            .summary("Delete a user")
            .tag("users"))
    }
}
```

### Example 4: Multiple Tags and Rich Documentation

**Before:**
```rust
impl Metadata for GetUserProfile {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id/profile", "get")
            .summary("Get user profile")
            .description("Retrieves detailed profile information for a specific user")
            .path_param("id").required()
    }
}
```

**After:**
```rust
impl Endpoint for GetUserProfile {
    fn ep(&self) -> Route {
        Route::GET("/users/:id/profile")
            .path_param("id").required()
    }

    fn docs(&self) -> Option<Docs> {
        Some(Docs::new()
            .summary("Get user profile")
            .description("Retrieves detailed profile information for a specific user")
            .tag("users")
            .tag("profiles"))
    }
}
```

## Server Registration

Use the new `register_endpoint()` method:

**Old:**
```rust
Server::new()
    .with_config(config)
    .register(GetUsers)  // Old API
    .serve()
    .await
```

**New:**
```rust
Server::new()
    .with_config(config)
    .register_endpoint(GetUsers)  // New API
    .serve()
    .await
```

## Backward Compatibility

The old `Metadata` trait and `register()` method are still supported. You can mix both APIs during migration:

```rust
Server::new()
    .with_config(config)
    .register(OldEndpoint)           // Old API
    .register_endpoint(NewEndpoint)  // New API
    .serve()
    .await
```

## Migration Checklist

- [ ] Replace `impl Metadata` with `impl Endpoint`
- [ ] Change `fn metadata()` to `fn ep()`
- [ ] Replace `Endpoint::new("/path", "method")` with `Route::METHOD("/path")`
- [ ] Move documentation to `fn docs() -> Option<Docs>`
- [ ] Update `register()` to `register_endpoint()`
- [ ] Add tags to organize your API documentation
- [ ] Test your endpoints

## Benefits Summary

| Feature | Old API | New API |
|---------|---------|---------|
| HTTP Methods | String-based (`"get"`) | Type-safe (`Route::GET()`) |
| Documentation | Mixed with routing | Separate (`docs()`) |
| Optional Docs | No | Yes |
| Tags | No | Yes |
| Separation of Concerns | Poor | Excellent |
| Type Safety | Good | Excellent |
| Extensibility | Limited | High |

## Questions?

If you encounter any issues during migration, please check the examples in the `examples/` directory or open an issue on GitHub.