# P1-T12 — Auth: Session Middleware

**Status:** TODO

## Goal

Implement the session validation extract for protected routes.

## Description

Create `src/auth/session.rs` with an Axum `extract::FromRequest` middleware that validates user sessions. This is used by all protected endpoints to ensure the requester is authenticated.

The middleware:
1. Reads the `session` cookie from the request
2. Calls `get_session_by_token` to look up the session
3. Validates the session has not expired
4. Fetches the associated user from the database
5. Returns `ValidSession(User)` on success or `AuthError::Unauthorized` on failure

## Acceptance Criteria

- AC1: `ValidSession` struct defined in `src/auth/session.rs`
- AC2: `impl FromRequest` for `ValidSession` on Axum `Request`
- AC3: Cookie name read is `session`
- AC4: Token lookup via `get_session_by_token` from `db/queries`
- AC5: Expired sessions return `AuthError::Unauthorized` (401)
- AC6: Returns `ValidSession(user)` with the authenticated user
- AC7: Missing cookie returns `AuthError::Unauthorized` (401)
- AC8: Invalid token returns `AuthError::Unauthorized` (401)
- AC9: Handler wired into `main.rs` router as a tower layer or middleware

## Technical Hints

- Use `axum::extract::FromRequest` trait:
  ```rust
  impl<S> FromRequest<S> for ValidSession
  where
      S: Send + Sync,
  {
      type Rejection = AppError;
      async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> { ... }
  }
  ```
- Cookie parsing: use `cookie::Cookie::parse("session", cookie_str)` or manual splitting
- Session expiry: `session.expires_at < chrono::Utc::now()`
- Consider using `tower_http::auth::RequireAuthorizationLayer` for a simpler approach
- Refer to: https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html
