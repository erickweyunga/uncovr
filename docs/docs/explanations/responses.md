# Responses

How to return structured, type-safe responses from your API handlers.

## The Response Problem

Every API endpoint needs to communicate two things: success and failure. The challenge is doing this consistently across your entire API while keeping your code clean and type-safe.

Uncovr solves this with `ApiResponse<T>` - a single type that handles all HTTP responses with proper status codes, error messages, and OpenAPI documentation.

## The Basics

Every handler returns an `ApiResponse<T>` where `T` is your success type:

```rust
use uncovr::prelude::*;

#[async_trait]
impl API for GetUser {
    type Req = ();
    type Res = ApiResponse<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> ApiResponse<UserResponse> {
        // Return success
        ApiResponse::Ok(UserResponse {
            id: 1,
            name: "John".to_string(),
        })
    }
}
```

That's it. `ApiResponse::Ok(data)` returns a 200 status with your data as JSON.

## Success Responses

Three variants handle successful operations:

**Ok (200)** - Standard success:
```rust
ApiResponse::Ok(user)
```

**Created (201)** - Use when creating resources:
```rust
ApiResponse::Created(new_user)
```

**NoContent (204)** - Use when there's nothing to return:
```rust
ApiResponse::NoContent
```

Choose based on HTTP semantics, not preference. Created means "I made something new." NoContent means "I succeeded but have nothing to show you."

## Error Responses

All error responses use the same structured format with a code and message:

```rust
ApiResponse::BadRequest {
    code: "invalid_email",
    message: "Email format is invalid",
}
```

The `code` is machine-readable (for client error handling). The `message` is human-readable (for displaying to users).

### Common Error Variants

**BadRequest (400)** - Invalid input from client:
```rust
if ctx.req.email.is_empty() {
    return ApiResponse::BadRequest {
        code: "empty_email",
        message: "Email is required",
    };
}
```

**Unauthorized (401)** - Authentication required:
```rust
if !is_authenticated(&ctx) {
    return ApiResponse::Unauthorized {
        code: "auth_required",
        message: "Authentication required",
    };
}
```

**Forbidden (403)** - Authenticated but lacks permission:
```rust
if !has_permission(&user, "delete") {
    return ApiResponse::Forbidden {
        code: "insufficient_permissions",
        message: "You cannot delete this resource",
    };
}
```

**NotFound (404)** - Resource doesn't exist:
```rust
ApiResponse::NotFound {
    code: "user_not_found",
    message: "User not found",
}
```

**Conflict (409)** - Resource already exists:
```rust
if email_exists(&ctx.req.email) {
    return ApiResponse::Conflict {
        code: "email_taken",
        message: "Email already registered",
    };
}
```

**UnprocessableEntity (422)** - Validation failed:
```rust
ApiResponse::UnprocessableEntity {
    code: "validation_failed",
    message: "Password must be at least 8 characters",
}
```

**InternalError (500)** - Something went wrong on the server:
```rust
ApiResponse::InternalError {
    code: "database_error",
    message: "Failed to process request",
}
```

**ServiceUnavailable (503)** - Service temporarily down:
```rust
ApiResponse::ServiceUnavailable {
    code: "maintenance",
    message: "Service under maintenance",
}
```

## Redirect Responses

Use redirects to send clients to different URLs:

**MovedPermanently (301)** - Resource permanently moved:
```rust
ApiResponse::MovedPermanently("/users/new-location".to_string())
```

**Found (302)** - Temporary redirect:
```rust
ApiResponse::Found("/temporary-page".to_string())
```

**SeeOther (303)** - Redirect after POST:
```rust
ApiResponse::SeeOther(format!("/users/{}", user.id))
```

Use `SeeOther` after creating/updating resources to redirect clients to the new resource.

## Error Codes: The Pattern

Error codes should be consistent and descriptive. Use this pattern:

- **Lowercase with underscores**: `user_not_found`, not `UserNotFound`
- **Descriptive but concise**: `invalid_email` is better than `invalid` or `email_format_validation_error`
- **Action-oriented when relevant**: `email_taken`, `quota_exceeded`

Good codes make client-side error handling easier:

```typescript
// Client can handle specific errors
if (error.code === "email_taken") {
  showError("Try a different email address");
} else if (error.code === "invalid_email") {
  showError("Check your email format");
}
```

## When to Use Which Response

**BadRequest vs UnprocessableEntity**:
- BadRequest: Malformed request, wrong type, missing required field
- UnprocessableEntity: Valid format but fails business rules

**Unauthorized vs Forbidden**:
- Unauthorized: Not logged in, invalid token
- Forbidden: Logged in but can't access this resource

**NotFound vs Gone**:
- NotFound: Never existed or you don't want to reveal it exists
- Gone: Existed before but was deleted (rarely used)

## Response Documentation

Tell OpenAPI what responses your endpoint returns:

```rust
impl Metadata for CreateUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")
            .with_responses(|op| {
                op.response::<201, Json<UserResponse>>()
                    .response::<400, Json<ErrorResponse>>()
                    .response::<409, Json<ErrorResponse>>()
            })
    }
}
```

Only document the responses you actually return. This keeps your API docs accurate and useful.

## Complete Example

Here's a complete handler showing multiple response types:

```rust
#[async_trait]
impl API for CreateUserApi {
    type Req = CreateUserRequest;
    type Res = ApiResponse<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> ApiResponse<UserResponse> {
        // Validate input
        if ctx.req.email.is_empty() {
            return ApiResponse::BadRequest {
                code: "empty_email",
                message: "Email is required",
            };
        }

        if !is_valid_email(&ctx.req.email) {
            return ApiResponse::UnprocessableEntity {
                code: "invalid_email",
                message: "Email format is invalid",
            };
        }

        // Check for conflicts
        if email_exists(&ctx.req.email) {
            return ApiResponse::Conflict {
                code: "email_taken",
                message: "Email already registered",
            };
        }

        // Create user
        match create_user(&ctx.req) {
            Ok(user) => ApiResponse::Created(user),
            Err(_) => ApiResponse::InternalError {
                code: "create_failed",
                message: "Failed to create user",
            },
        }
    }
}
```

## Error Response Format

All errors return the same JSON structure:

```json
{
  "code": "email_taken",
  "message": "Email already registered",
  "details": null
}
```

The `details` field is optional and can contain additional error information. For most cases, `code` and `message` are sufficient.

## Common Patterns

**Early returns for validation**:
```rust
if invalid_condition {
    return ApiResponse::BadRequest { code: "...", message: "..." };
}
if another_issue {
    return ApiResponse::UnprocessableEntity { code: "...", message: "..." };
}
// Continue with happy path
```

**Match for Result handling**:
```rust
match database_operation() {
    Ok(data) => ApiResponse::Ok(data),
    Err(_) => ApiResponse::InternalError {
        code: "db_error",
        message: "Database operation failed",
    },
}
```

**Option handling**:
```rust
match find_user(id) {
    Some(user) => ApiResponse::Ok(user),
    None => ApiResponse::NotFound {
        code: "user_not_found",
        message: "User not found",
    },
}
```

## Key Takeaways

1. **Use ApiResponse for all handlers** - Consistent response handling across your API
2. **Choose the right status code** - HTTP status codes have meaning, use them correctly
3. **Structure your errors** - Always include both code and message
4. **Document your responses** - Tell clients what to expect with `.with_responses()`
5. **Be consistent** - Same patterns across all endpoints makes your API predictable

Good response handling makes your API professional, predictable, and easy to integrate with.