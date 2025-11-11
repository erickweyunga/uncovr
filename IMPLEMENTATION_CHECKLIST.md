# v0.3.0 Implementation Checklist

> **Status:** Ready to Start  
> **Target Release:** v0.3.0  
> **Breaking Changes:** Yes (Major API Refactor)

## Overview

This checklist tracks the implementation of v0.3.0, which includes major API simplification and naming improvements based on the [TOKEN_DICTIONARY.md](TOKEN_DICTIONARY.md).

---

## Phase 1: Core Type Renames

### 1.1 Response & Error Types (src/api/)

- [ ] Rename `ApiResponse<T>` → `Response<T>` in `src/api/response.rs`
- [ ] Rename `ApiError` → `Error` in `src/api/response.rs`
- [ ] Update `Error` variants with helper constructors:
  - [ ] `Error::bad_request(code, msg)`
  - [ ] `Error::unauthorized(code, msg)`
  - [ ] `Error::forbidden(code, msg)`
  - [ ] `Error::not_found(code, msg)`
  - [ ] `Error::conflict(code, msg)`
  - [ ] `Error::unprocessable(code, msg)`
  - [ ] `Error::internal(code, msg)`
- [ ] Implement `IntoResponse` for `Error`
- [ ] Implement `std::error::Error` trait for `Error`
- [ ] Update exports in `src/api/mod.rs`

### 1.2 Parameter Types (src/server/)

- [ ] Rename `PathParams` → `Path` in `src/server/params.rs`
- [ ] Rename `QueryParams` → `Query` in `src/server/params.rs`
- [ ] Add generic `.get<T>()` method to `Path`
- [ ] Add generic `.get<T>()` method to `Query`
- [ ] Add `.parse<T>()` method that returns `Result<T, Error>`
- [ ] Update exports in `src/server/mod.rs`

### 1.3 Documentation Types (src/server/)

- [ ] Rename `Docs` → `Meta` in `src/server/endpoint.rs`
- [ ] Update all references to use `Meta`
- [ ] Change `.docs()` to return `Meta` instead of `Option<Docs>`

---

## Phase 2: Trait Renames

### 2.1 Handler Trait (src/api/)

- [ ] Rename `API` trait → `Handler` in `src/api/api.rs`
- [ ] Rename associated types:
  - [ ] `Req` → `Request`
  - [ ] `Res` → `Response`
- [ ] Rename method: `.handler()` → `.handle()`
- [ ] Update trait bounds for `IntoResponse`
- [ ] Update exports in `src/api/mod.rs`

### 2.2 Endpoint Trait (src/server/)

- [ ] Rename `.ep()` → `.route()` in `src/server/endpoint.rs`
- [ ] Rename `.docs()` → `.meta()` in `src/server/endpoint.rs`
- [ ] Remove `Option<>` from `.meta()` return type
- [ ] Update all internal usages

---

## Phase 3: Route Methods

### 3.1 Lowercase HTTP Methods (src/server/)

- [ ] `Route::GET()` → `Route::get()` in `src/server/endpoint.rs`
- [ ] `Route::POST()` → `Route::post()`
- [ ] `Route::PUT()` → `Route::put()`
- [ ] `Route::PATCH()` → `Route::patch()`
- [ ] `Route::DELETE()` → `Route::delete()`
- [ ] `Route::OPTIONS()` → `Route::options()`
- [ ] `Route::HEAD()` → `Route::head()`
- [ ] Keep uppercase as deprecated aliases for compatibility (optional)

### 3.2 Route Builder Methods

- [ ] Rename `.path_param()` → `.param()` for consistency
- [ ] Keep `.query()` as-is
- [ ] Add `.required()` method for parameters
- [ ] Add `.deprecated()` method for routes

---

## Phase 4: Context & State Management

### 4.1 Context Updates (src/context/)

- [ ] Add `.state<T>()` method to `Context` in `src/context/context.rs`
- [ ] Add `.try_state<T>()` method (returns Option)
- [ ] Update `Path` and `Query` field types
- [ ] Add better error messages for missing state

### 4.2 Server Builder (src/server/)

- [ ] Add `.with_state<S>()` method to `ServerBuilder` in `src/server/builder.rs`
- [ ] Store state in `Extension` layer
- [ ] Update registration to work without state in endpoint structs
- [ ] Ensure state is accessible in handlers via `ctx.state()`

---

## Phase 5: Error Handling Integration

### 5.1 Error Conversions (src/api/)

- [ ] Implement `From<ParamError>` for `Error`
- [ ] Add `ParamError` type for parameter extraction failures
- [ ] Update parameter extraction to return `Result<T, ParamError>`
- [ ] Add examples of `?` operator usage

### 5.2 Response Types (src/api/)

- [ ] Ensure `Result<T, Error>` implements `IntoResponse`
- [ ] Add helper for automatic status code mapping
- [ ] Update OpenAPI integration to handle Result types
- [ ] Add response examples in documentation

---

## Phase 6: Update Internal Usages

### 6.1 Server Builder (src/server/builder.rs)

- [ ] Update all trait bounds: `API` → `Handler`
- [ ] Update method calls: `.handler()` → `.handle()`
- [ ] Update method calls: `.ep()` → `.route()`
- [ ] Update method calls: `.docs()` → `.meta()`
- [ ] Remove `Option` handling for `.meta()`
- [ ] Update HTTP method constructors to lowercase

### 6.2 OpenAPI Integration (src/openapi/)

- [ ] Update to use `Meta` instead of `Docs`
- [ ] Update to use `Response` instead of `ApiResponse`
- [ ] Update to use `Error` instead of `ApiError`
- [ ] Ensure schema generation works with new types

---

## Phase 7: Update Prelude

### 7.1 Exports (src/prelude.rs)

- [ ] Export `Handler` (remove `API`)
- [ ] Export `Response` (remove `ApiResponse`)
- [ ] Export `Error` (remove `ApiError`)
- [ ] Export `Meta` (remove `Docs`)
- [ ] Export `Path` (remove `PathParams`)
- [ ] Export `Query` (remove `QueryParams`)
- [ ] Add deprecation warnings for old names (optional)

---

## Phase 8: Update Examples

### 8.1 examples/api

- [ ] Update trait implementations: `API` → `Handler`
- [ ] Update methods: `.ep()` → `.route()`, `.handler()` → `.handle()`
- [ ] Update types: `ApiResponse` → `Response`
- [ ] Update HTTP methods: `Route::GET()` → `Route::get()`
- [ ] Update `.docs()` → `.meta()` and remove `Some()`

### 8.2 examples/auth-jwt

- [ ] Remove state from endpoint struct definitions
- [ ] Update to use `ctx.state::<AppState>()`
- [ ] Update error handling to use `Result<T, Error>`
- [ ] Update to use `?` operator
- [ ] Update all trait and method names
- [ ] Update parameter extraction to use `.get::<T>()`

### 8.3 examples/routes

- [ ] Update parameter extraction examples
- [ ] Show new `.get::<T>()` syntax
- [ ] Update all naming conventions
- [ ] Add error handling examples

### 8.4 examples/url-shortner

- [ ] Refactor to use proper state injection
- [ ] Remove global state pattern
- [ ] Update all naming conventions
- [ ] Add validation examples

### 8.5 examples/nested-services

- [ ] Update all naming conventions
- [ ] Ensure compatibility with Tower services

### 8.6 examples/live-reload

- [ ] Update state management pattern
- [ ] Update all naming conventions
- [ ] Ensure template rendering works

---

## Phase 9: Documentation Updates

### 9.1 README.md

- [ ] Update all code examples with new API
- [ ] Update feature descriptions
- [ ] Add migration guide link
- [ ] Update quick start example

### 9.2 lib.rs Documentation

- [ ] Update module-level documentation
- [ ] Update all code examples
- [ ] Update feature descriptions
- [ ] Add migration notes

### 9.3 Migration Guide

- [ ] Create MIGRATION_v0.3.md
- [ ] Document all breaking changes
- [ ] Provide before/after examples
- [ ] Add search-and-replace guide
- [ ] Include common patterns

### 9.4 Contributing Guide

- [ ] Update with Token Dictionary reference
- [ ] Add naming convention requirements
- [ ] Update code style guidelines

---

## Phase 10: Testing

### 10.1 Unit Tests

- [ ] Update all unit tests with new API
- [ ] Add tests for new `.state()` functionality
- [ ] Add tests for new parameter extraction
- [ ] Add tests for error conversions

### 10.2 Integration Tests

- [ ] Create integration tests for each example
- [ ] Test state management
- [ ] Test error handling
- [ ] Test parameter extraction

### 10.3 Documentation Tests

- [ ] Ensure all doc examples compile
- [ ] Update doc tests with new API
- [ ] Add more comprehensive examples

---

## Phase 11: Validation Framework (Optional for v0.3.0)

If time permits, add validation support:

- [ ] Add `validator` crate dependency (optional feature)
- [ ] Integrate automatic validation in request handling
- [ ] Add `ValidationError` variant to `Error`
- [ ] Update examples to show validation
- [ ] Add validation to OpenAPI schemas

---

## Phase 12: Release Preparation

### 12.1 Version Bump

- [ ] Update Cargo.toml version to 0.3.0
- [ ] Update all workspace member versions
- [ ] Update dependency versions if needed

### 12.2 Changelog

- [ ] Create CHANGELOG.md entry for v0.3.0
- [ ] List all breaking changes
- [ ] List all new features
- [ ] List all improvements
- [ ] Add migration guide link

### 12.3 Final Review

- [ ] Run `cargo fmt` on all code
- [ ] Run `cargo clippy` and fix warnings
- [ ] Run `cargo test` and ensure all pass
- [ ] Build all examples successfully
- [ ] Test examples manually
- [ ] Review all documentation
- [ ] Spell check documentation

### 12.4 Release

- [ ] Create git tag v0.3.0
- [ ] Push to GitHub
- [ ] Publish to crates.io
- [ ] Create GitHub release with notes
- [ ] Announce on social media / community

---

## Estimated Timeline

| Phase | Estimated Time | Status |
|-------|---------------|--------|
| Phase 1: Core Type Renames | 2-3 hours | ⏸️ Not Started |
| Phase 2: Trait Renames | 1-2 hours | ⏸️ Not Started |
| Phase 3: Route Methods | 1-2 hours | ⏸️ Not Started |
| Phase 4: Context & State | 3-4 hours | ⏸️ Not Started |
| Phase 5: Error Handling | 2-3 hours | ⏸️ Not Started |
| Phase 6: Internal Updates | 2-3 hours | ⏸️ Not Started |
| Phase 7: Prelude Updates | 30 mins | ⏸️ Not Started |
| Phase 8: Update Examples | 4-6 hours | ⏸️ Not Started |
| Phase 9: Documentation | 3-4 hours | ⏸️ Not Started |
| Phase 10: Testing | 2-3 hours | ⏸️ Not Started |
| Phase 11: Validation (Optional) | 4-5 hours | ⏸️ Optional |
| Phase 12: Release Prep | 2-3 hours | ⏸️ Not Started |
| **Total** | **27-38 hours** | **⏸️ Not Started** |

**Expected Duration:** 4-5 working days (full-time) or 1-2 weeks (part-time)

---

## Notes

- Keep backward compatibility where possible using `#[deprecated]` attributes
- Consider creating a `uncovr-0.2-compat` crate for gradual migration
- Document every breaking change thoroughly
- Test with real-world applications if available
- Get community feedback on breaking changes before finalizing

---

## Progress Tracking

**Started:** Not yet  
**Current Phase:** Not started  
**Completion:** 0%

**Last Updated:** 2025-11-11
