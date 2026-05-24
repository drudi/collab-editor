# P1-T10 — Auth: Registration

**Status:** Done

## Goal

Implement the user registration endpoint.

## Description

Create `src/auth/register.rs` with a `POST /api/auth/register` handler. This endpoint allows new users to create an account.

The handler flow:
1. Extract and validate JSON body (username + password)
2. Check username doesn't already exist
3. Hash password with argon2id
4. Insert user and session into the database
5. Set HttpOnly, Secure, SameSite=Lax cookie
6. Return 201 with user data

Input validation:
- Username: 3-32 characters, alphanumeric + underscore
- Password: minimum 6 characters

## Acceptance Criteria

- AC1: `POST /api/auth/register` handler exists in `src/auth/register.rs`
- AC2: JSON body extraction with `axum::Json<RegisterRequest>`
- AC3: Username validation: 3-32 chars, alphanumeric + underscore
- AC4: Password validation: minimum 6 characters
- AC5: Username uniqueness check against database
- AC6: Password hashed with argon2id using default params
- AC7: Session created with token, `expires_at` set to 7 days from now
- AC8: Cookie set with `HttpOnly`, `Secure`, `SameSite=Lax`, `Path=/`
- AC9: Returns `201 Created` with `UserResponse` on success
- AC10: Returns `409 Conflict` if username already exists
- AC11: Returns `400 Bad Request` for validation errors
- AC12: Handler wired into `main.rs` router

## Technical Hints

- Use `argon2::Argon2::default()` for argon2id with recommended params
- Generate token with `uuid::Uuid::new_v4()` or `rand::thread_rng().gen::<[u8; 32]>()`
- Cookie header: `Set-Cookie: session={token}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=604800`
- Use `axum::response::WithStatus` for typed responses
- Validation pattern: collect errors, return first in `RegisterRequest` struct
- Refer to: https://docs.rs/argon2/latest/argon2/
