# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2025-11-08

### Added
- **New Endpoint API with separation of concerns**: Complete redesign of endpoint definition
  - New `Endpoint` trait replaces old `Metadata` trait
  - Separate `ep()` method for route definition (path, method, parameters)
  - Separate `docs()` method for optional API documentation
  - Type-safe HTTP method constructors: `Route::GET()`, `Route::POST()`, `Route::PUT()`, `Route::PATCH()`, `Route::DELETE()`, `Route::OPTIONS()`, `Route::HEAD()`
  - New `Route` struct for clean route definitions
  - New `Docs` struct for comprehensive API documentation
  - Support for `responses()` method in `Docs` for OpenAPI response documentation
  - Tag support for organizing endpoints in documentation
  - `HttpMethod` enum for type-safe method handling
- **Response documentation in Docs**: Moved response configuration into `Docs` struct
  - `.responses()` method for documenting HTTP status codes and response types
  - Integrates seamlessly with OpenAPI generation
- **Re-exported middleware modules**: Added `tower` and `axum_middleware` re-exports to main library
  - No need for separate dependencies in user code
  - Access via `uncovr::tower` and `uncovr::axum_middleware`
- **Auto-detect server URL**: OpenAPI spec now automatically derives server URL from bind address
  - No hardcoded localhost defaults
  - Use `add_server()` to explicitly configure custom domains
  - Supports multiple server environments
- **Comprehensive migration guide**: Added `docs/MIGRATION_ENDPOINT_API.md` with full migration examples

### Changed
- **BREAKING**: Removed old `Metadata` trait - use new `Endpoint` trait instead
- **BREAKING**: Removed old `Endpoint` struct - replaced with new `Route` struct
- **BREAKING**: HTTP methods now use uppercase constructors (`Route::GET()` instead of `Endpoint::new("/path", "get")`)
- **BREAKING**: Documentation is now optional via `Option<Docs>` return type
- **BREAKING**: Response configuration moved from separate callback to `responses()` in `Docs`
- Updated all 4 examples to use new Endpoint API
  - `examples/api` - 5 endpoints updated
  - `examples/auth-jwt` - 4 endpoints with full response docs
  - `examples/routes` - 3 routing examples
  - `examples/url-shortner` - 2 endpoints updated
- Benchmark suite redesigned and reduced by 52% (762 → 364 lines)
  - 5 focused benchmarks instead of 20+ redundant ones
  - All benchmarks updated to new API
  - Clear documentation and structure
- Complete documentation overhaul
  - Updated README.md with new API examples
  - Updated routing tutorial and explanations
  - Updated by-example guide
  - All code examples use new Endpoint API

### Performance
- Maintained excellent performance: ~23,000 req/sec across all benchmarks
- Sub-5ms average latency
- No performance regression from API redesign

### Migration
- See `docs/MIGRATION_ENDPOINT_API.md` for complete migration guide
- Main changes:
  - `impl Metadata` → `impl Endpoint`
  - `fn metadata()` → `fn ep()` and `fn docs()`
  - `Endpoint::new("/path", "get")` → `Route::GET("/path")`
  - `.with_responses()` → `.responses()` in `Docs`

## [0.2.1] - 2025-01-8

### Added
- **Middleware support via Extensions**: Added `extensions` field to `Context` struct for middleware data access
  - Middleware can now inject data into requests that handlers can access via `ctx.extensions.get::<T>()`
  - Created custom `ExtractExtensions` extractor compatible with aide/OpenAPI
  - All HTTP method handlers (GET, POST, PUT, PATCH, DELETE) now extract and pass extensions
- **Middleware modules**: Exported `axum_middleware` and `tower` modules for middleware utilities
  - `axum_middleware` provides access to Axum's native middleware functions
  - `tower` provides access to Tower's composable middleware layers
  - Enables authentication, logging, rate limiting, and other cross-cutting concerns
- **JWT Authentication Example**: Added comprehensive `auth-jwt` example demonstrating:
  - User registration and login with JWT tokens
  - Password hashing with bcrypt
  - JWT token generation and validation
  - Protected routes using authentication middleware
  - Extensions-based user context passing
  - SQLite database with migrations

### Changed
- **BREAKING**: Error messages now use `String` instead of `&'static str`
  - Allows dynamic error messages with `format!()` macro
  - Static messages require `.to_string()` conversion
  - Example: `ApiResponse::BadRequest { code: "error", message: "Error message".to_string() }`
  - Example with dynamic data: `message: format!("User {} not found", id)`

### Removed
- Removed obsolete `middleware` module (replaced by native Axum middleware via `axum_middleware`)

## [0.2.0] - 2025-01-08

### Added
- **Redirect response support**: Added 5 new redirect response variants to `ApiResponse`
  - `MovedPermanently` (301) - Permanent redirect
  - `Found` (302) - Temporary redirect
  - `SeeOther` (303) - Redirect to different resource
  - `TemporaryRedirect` (307) - Temporary redirect preserving method
  - `PermanentRedirect` (308) - Permanent redirect preserving method
- **Response documentation callback**: New `with_responses()` method on `Endpoint` for flexible OpenAPI response configuration
  - Provides direct access to aide's `TransformOperation` for maximum flexibility
  - Allows declarative response documentation: `.with_responses(|op| op.response::<200, T>().response::<400, E>())`
  - Applied across all HTTP method handlers (GET, POST, PUT, PATCH, DELETE)
- **Custom error responses**: Added `ClientError` and `ServerError` variants for custom HTTP status codes with error details

### Changed
- **BREAKING**: Restructured error response format to use structured `code` and `message` fields
  - All error variants now use `{ code: &'static str, message: &'static str }` format
  - Example: `ApiResponse::BadRequest { code: "invalid_id", message: "ID must be greater than 0" }`
  - Provides consistent, machine-readable error responses
- **BREAKING**: Renamed `ErrorDetails` to `ErrorResponse` for better clarity
- **BREAKING**: Removed simple string-based error variants (e.g., `BadRequest(&'static str)`)
- Updated `ApiResponse` implementation to support all new response types
- Improved error response documentation with clearer examples

### Migration Guide
To upgrade from 0.1.x to 0.2.0:

1. Update error responses to use structured format:
   ```rust
   // Old (0.1.x)
   ApiResponse::BadRequest("Invalid input")

   // New (0.2.0)
   ApiResponse::BadRequest {
       code: "invalid_input",
       message: "Invalid input"
   }
   ```

2. Rename `ErrorDetails` to `ErrorResponse`:
   ```rust
   // Old (0.1.x)
   use uncovr::prelude::ErrorDetails;

   // New (0.2.0)
   use uncovr::prelude::ErrorResponse;
   ```

3. Add response documentation using `with_responses()`:
   ```rust
   Endpoint::new("/users", "get")
       .summary("Get users")
       .with_responses(|op| {
           op.response::<200, Json<Vec<User>>>()
             .response::<400, Json<ErrorResponse>>()
       })
   ```

## [0.1.2] - 2025-11-06

### Added
- **Path and query parameter support**: Routes can now extract path parameters (e.g., `/users/:id`) and query parameters (e.g., `?page=1&limit=10`)
  - New `PathParams` and `QueryParams` types with type-safe conversion methods (`get_u64()`, `get_string()`, etc.)
  - Added `path` and `query` fields to `Context` struct
  - All HTTP method handlers (GET, POST, PUT, PATCH, DELETE) now extract and provide access to parameters
- **Parameter declaration API**: Endpoints can declare query and path parameters for documentation
  - New `ParamInfo` struct for parameter metadata
  - Chainable API: `.query("name").desc("description").required()`
  - `.path_param()` method for declaring path parameters
- **OpenAPI integration for query parameters**: Query parameters now appear in OpenAPI specification
  - Parameters show in `/openapi.json` with correct schema and descriptions
  - Interactive documentation in Scalar UI displays all query parameters
  - Helper function converts parameter metadata to OpenAPI format

### Fixed
- **Port configuration synchronization**: `api_servers` now automatically updates when `bind()` is called
  - OpenAPI documentation correctly shows the actual server port instead of always showing :3000
  - Bind address and API server URLs stay in sync

### Changed
- Documentation website structure simplified following FastHTML pattern

## [0.1.1] - 2025-11-05

### Fixed
- Fixed inline documentation to use 'Uncovr' instead of 'Uncover'
- Updated all module documentation headers
- Updated AppConfig default values to use 'Uncovr'
- Fixed inline code examples in documentation

## [0.1.0] - 2025-11-04

### Initial Release
- First working version of Uncovr framework
