# P1-T03 — Error Types Module

**Status:** Done

## Goal

Define a unified `AppError` type that covers all error domains with Axum integration.

## Description

Create `src/error/mod.rs` with a centralized error type that the entire application uses. This prevents scattered `anyhow!()` or ad-hoc error handling and provides consistent HTTP responses.

The `AppError` enum has variants for each domain:
- `AuthError(String)` — authentication failures
- `RoomError(String)` — room-related errors
- `CollabError(String)` — collaboration/CRDT errors
- `LintError(String)` — linting errors
- `DbError(String)` — database errors
- `IoError(String)` — I/O errors

Each variant maps to an appropriate HTTP status code (401, 404, 500, etc.). The `IntoResponse` impl converts the error into an Axum response with both a status code and a JSON error body.

Use `thiserror` for the `Error` trait derive and `fmt::Display` for each variant. Use `serde::Serialize` on the response wrapper for JSON output.

## Acceptance Criteria

- AC1: `AppError` enum with all six domain variants defined
- AC2: `thiserror::Error` derive on `AppError`
- AC3: `fmt::Display` implementation for `AppError`
- AC4: `impl IntoResponse for AppError` that returns correct HTTP status codes
- AC5: `AuthError` returns 401, `RoomError(NotFound)` returns 404, others return 500
- AC6: JSON response body includes `error` and `message` fields
- AC7: `cargo check` passes

## Technical Hints

- Use `thiserror` crate: `#[derive(thiserror::Error)]`
- Implement `IntoResponse` manually: `fn into_response(self) -> axum::response::Response`
- Use `axum::Json` for the JSON response body
- Status code mapping:
  - `AuthError` → `StatusCode::UNAUTHORIZED` (401)
  - `RoomError(NotFound)` → `StatusCode::NOT_FOUND` (404)
  - `RoomError(Conflict)` → `StatusCode::CONFLICT` (409)
  - Others → `StatusCode::INTERNAL_SERVER_ERROR` (500)
- Refer to: https://docs.rs/axum/latest/axum/extract/struct.FromRequest.html
- Pattern: `Error` enum with domain-specific subtypes for precise error handling
