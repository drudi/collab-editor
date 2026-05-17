# P1-T13 — Auth: Logout

**Status:** TODO

## Goal

Implement the user logout endpoint.

## Description

Create `src/auth/logout.rs` with a `POST /api/auth/logout` handler. This endpoint destroys the current user session and clears the session cookie.

The handler flow:
1. Read session token from cookie (using the same extraction as session middleware)
2. Delete the session from the database by token
3. Set `Set-Cookie` with `Max-Age=0` to clear the cookie
4. Return 200

## Acceptance Criteria

- AC1: `POST /api/auth/logout` handler exists in `src/auth/logout.rs`
- AC2: Session token extracted from cookie
- AC3: Session deleted from database by token
- AC4: Cookie cleared with `Max-Age=0`, `Path=/`, `HttpOnly`, `Secure`, `SameSite=Lax`
- AC5: Returns `200 OK` on success
- AC6: Returns `401 Unauthorized` if no valid session cookie
- AC7: Handler wired into `main.rs` router

## Technical Hints

- Cookie clearing: `Set-Cookie: session=; Max-Age=0; Path=/; HttpOnly; Secure; SameSite=Lax`
- The session deletion is best-effort — even if DB delete fails, cookie clearing is sufficient
- Use `chrono::TimeDelta::zero()` or `Max-Age=0` for immediate expiry
- Refer to: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
