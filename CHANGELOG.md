# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
