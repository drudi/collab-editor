# P1-T11 — Auth: Login

**Status:** TODO

## Goal

Implement the user login endpoint.

## Description

Create `src/auth/login.rs` with a `POST /api/auth/login` handler. This endpoint authenticates existing users and creates a new session.

The handler flow:
1. Extract and validate JSON body (username + password)
2. Look up user by username
3. Verify password with argon2 `verify()`
4. Generate session token, insert session into database
5. Set HttpOnly, Secure, SameSite=Lax cookie
6. Return 200 with user data

## Acceptance Criteria

- AC1: `POST /api/auth/login` handler exists in `src/auth/login.rs`
- AC2: JSON body extraction with `axum::Json<LoginRequest>`
- AC3: Username format validation: 3-32 chars, alphanumeric + underscore
- AC4: Password validation: minimum 6 characters
- AC5: User lookup by username returns `None` for non-existent users
- AC6: Password verified with `argon2::verify()`
- AC7: Session created with random token, `expires_at` = 7 days from now
- AC8: Cookie set with `HttpOnly`, `Secure`, `SameSite=Lax`, `Path=/`
- AC9: Returns `200 OK` with `UserResponse` on success
- AC10: Returns `401 Unauthorized` on invalid credentials
- AC11: Uses constant-time comparison (argon2 handles this internally)
- AC12: Handler wired into `main.rs` router

## Technical Hints

- `argon2::Argon2::default().verify_password(password.as_bytes(), &password_hash)` returns `Result`
- Never reveal whether username or password was wrong — always return same error message
- Use `chrono::Utc::now() + chrono::TimeDelta::days(7)` for expiry calculation
- Same login request struct as register can be reused with `#[serde(deny_unknown_fields)]`
- Refer to: https://docs.rs/argon2/latest/argon2/struct.Argon2.html#method.verify_password
