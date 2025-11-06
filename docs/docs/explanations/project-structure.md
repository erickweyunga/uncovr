# Project Structure

How to organize your Uncovr application for clarity and scalability.

## The Core Principle

Good project structure answers one question: "Where do I find the code that does X?" 

When someone looks at your project, they should immediately understand where to look for users logic, posts logic, or authentication. The structure should guide them naturally.

## The Recommended Pattern

Uncovr works best when you organize by **feature** (resource), with a clear separation between **what** (API definitions) and **how** (business logic).

```
src/
├── main.rs              # Server setup and configuration
├── users/               # Everything related to users
│   ├── mod.rs
│   ├── apis.rs          # Routes, types, documentation
│   └── handlers.rs      # Business logic
└── posts/               # Everything related to posts
    ├── mod.rs
    ├── apis.rs
    └── handlers.rs
```

Each feature lives in its own module. Each module splits into two files: definitions and implementations.

## The Two-File Split

### apis.rs - Definitions

This file declares **what** your API looks like:
- Request and response types
- Endpoint structs
- Route paths and HTTP methods
- OpenAPI documentation

Think of this as your API's contract. Someone reading `apis.rs` should understand your entire API surface without seeing any implementation details.

### handlers.rs - Implementation  

This file contains **how** your API works:
- Business logic
- Data validation
- Database queries
- External API calls

This is where the actual work happens. It implements the contract defined in `apis.rs`.

## Why This Works

**For Reading**: When you need to understand an endpoint, you know exactly where to look. Route definition? `apis.rs`. Business logic? `handlers.rs`.

**For Changing**: Want to change how something works? Edit `handlers.rs`. Need to add a query parameter? Edit `apis.rs`. Clear boundaries prevent changes from spreading.

**For Testing**: Test API contracts separately from business logic. Mock handlers to test API definitions. Mock data sources to test handlers.

**For Teams**: Different developers can work on different features without touching the same files. Code reviews stay focused on one feature at a time.

## Starting Small

If your entire API is 2-3 endpoints, don't over-engineer. Put everything in one file:

```
src/
├── main.rs
└── api.rs               # All endpoints here
```

But once you have more than one resource (users AND posts, for example), split into feature modules.

## Growing Larger

As your API grows, the pattern scales naturally:

```
src/
├── main.rs
├── users/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
├── posts/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
├── comments/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
└── auth/
    ├── mod.rs
    ├── apis.rs
    └── handlers.rs
```

Each new feature follows the same pattern. No need to rethink your structure as you grow.

## What About Shared Code?

Create dedicated modules for code used across features:

```
src/
├── main.rs
├── database.rs          # Database connection and utilities
├── middleware.rs        # Custom middleware
├── models.rs            # Shared data types
├── users/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
└── posts/
    ├── mod.rs
    ├── apis.rs
    └── handlers.rs
```

Keep shared modules at the root level. Feature modules stay isolated.

## Nested Resources

For resources that belong to other resources (like user posts), you have two choices:

**Option 1 - Flat Structure** (Recommended for most cases):
```
src/
├── users/
│   ├── apis.rs          # /users, /users/:id
│   └── handlers.rs
└── posts/
    ├── apis.rs          # /posts, /users/:user_id/posts
    └── handlers.rs
```

Posts are in their own module even though they can be accessed through users. This keeps modules independent.

**Option 2 - Nested Modules** (For tightly coupled resources):
```
src/
└── users/
    ├── mod.rs
    ├── apis.rs          # /users, /users/:id
    ├── handlers.rs
    └── posts/
        ├── mod.rs
        ├── apis.rs      # /users/:user_id/posts
        └── handlers.rs
```

Use this only when posts make no sense without users. In most cases, flat is better.

## The Main File

Keep `main.rs` simple and declarative:

```rust
use uncovr::{prelude::*, server::Server};

mod users;
mod posts;

#[tokio::main]
async fn main() {
    let config = AppConfig::new("My API", "1.0.0")
        .bind("0.0.0.0:3000");

    Server::new()
        .with_config(config)
        .register(users::apis::CreateUserApi)
        .register(users::apis::GetUserApi)
        .register(posts::apis::CreatePostApi)
        .register(posts::apis::GetPostApi)
        .serve()
        .await
        .expect("Server failed to start")
}
```

Reading `main.rs` should give you a clear picture of what your API offers.

## Common Mistakes

### Mistake 1: Too Many Layers

Don't create unnecessary depth:
```
src/
└── api/
    └── endpoints/
        └── v1/
            └── resources/
                └── users/
                    └── handlers/
                        └── get.rs
```

This is over-engineered. Keep it flat:
```
src/
└── users/
    ├── apis.rs
    └── handlers.rs
```

### Mistake 2: Mixing Concerns

Don't put everything in `apis.rs`:
```rust
// apis.rs - BAD: Has business logic
impl API for CreateUser {
    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Database queries here
        // Validation here
        // Complex logic here
    }
}
```

Keep `apis.rs` focused on definitions. Move implementation to `handlers.rs`.

### Mistake 3: No Clear Boundaries

Don't let modules depend on each other's internal details:
```rust
// posts/handlers.rs - BAD: Reaches into users internals
use crate::users::handlers::internal_user_validation;
```

Share through well-defined interfaces, not internal functions.

## Key Takeaways

1. **Organize by feature** - Each resource gets its own module
2. **Split definitions from implementation** - `apis.rs` vs `handlers.rs`
3. **Keep it flat** - Avoid unnecessary nesting
4. **Start simple** - Add structure as you grow
5. **Be consistent** - Every feature follows the same pattern

Good structure makes your code easy to understand, easy to change, and easy to maintain. The pattern is simple, but it scales.