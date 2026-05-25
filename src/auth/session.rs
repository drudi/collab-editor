//! Session validation middleware — Axum extractor for authenticated routes.
//!
//! Parses the `session` cookie, looks up the token in the database,
//! validates expiry, and returns the authenticated `User`.
//!
//! Usage in a handler:
//! ```ignore
//! pub async fn protected_handler(
//!     ValidSession(user): ValidSession,
//! ) -> Result<Json<UserResponse>, AppError> {
//!     Ok(Json(UserResponse::from(user)))
//! }
//! ```

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sqlx::SqlitePool;

use crate::db;
use crate::error::AppError;
use crate::models::User;

/// Extractor that represents a valid, authenticated session.
///
/// Contains the `User` associated with the validated session cookie.
#[derive(Debug, Clone)]
pub struct ValidSession(pub User);

impl FromRequestParts<SqlitePool> for ValidSession {
    type Rejection = AppError;

    async fn from_request_parts(
        req: &mut Parts,
        pool: &SqlitePool,
    ) -> Result<Self, Self::Rejection> {
        // ── 1. Read the Cookie header and find the `session` token ──
        let cookie_header = req
            .headers
            .get(axum::http::header::COOKIE)
            .ok_or_else(|| AppError::AuthError("Missing Cookie header".into()))?;

        let cookie_str = cookie_header.to_str().map_err(|_| {
            AppError::AuthError("Cookie header contains invalid characters".into())
        })?;

        let session_token = extract_cookie_value(cookie_str, "session").ok_or_else(|| {
            AppError::AuthError("Missing session cookie".into())
        })?;

        // ── 2. Look up the session in the database ──
        let session = db::queries::get_session_by_token(pool, &session_token)
            .await
            .map_err(|e| AppError::DbError(e))?;

        let session = session.ok_or_else(|| {
            AppError::AuthError("Invalid session token".into())
        })?;

        // ── 3. Validate the session has not expired ──
        let now = chrono::Utc::now().naive_utc();
        if session.expires_at < now {
            return Err(AppError::AuthError("Session has expired".into()));
        }

        // ── 4. Fetch the associated user ──
        let user = db::queries::get_user_by_id(pool, session.user_id)
            .await
            .map_err(|e| AppError::DbError(e))?
            .ok_or_else(|| AppError::AuthError("Associated user not found".into()))?;

        Ok(ValidSession(user))
    }
}

/// Extract the value of a cookie named `name` from a `Cookie` header string.
///
/// Format: `session=abc123; path=/; HttpOnly`
/// Returns the value part only (no leading `=`).
fn extract_cookie_value(cookie_str: &str, name: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        let part = part.trim();
        if let Some((key, value)) = part.split_once('=') {
            if key.trim() == name {
                return Some(value.trim().to_string());
            }
        }
    }
    None
}

/// Get the session cookie string for a given token.
///
/// Matches the format used in register/login handlers for consistency.
pub fn session_cookie_string(token: &str) -> String {
    format!(
        "session={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}",
        token,
        7 * 24 * 60 * 60
    )
}

/// Optional session extractor — returns `None` for unauthenticated requests instead of rejecting.
#[derive(Debug, Clone)]
pub struct OptionalSession(pub Option<User>);

impl FromRequestParts<SqlitePool> for OptionalSession {
    type Rejection = ();

    async fn from_request_parts(
        req: &mut Parts,
        pool: &SqlitePool,
    ) -> Result<Self, Self::Rejection> {
        // Read the Cookie header and find the `session` token
        let cookie_str = req
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let session_token = extract_cookie_value(cookie_str, "session");

        let user = if let Some(token) = session_token {
            let session = db::queries::get_session_by_token(pool, &token)
                .await
                .ok()
                .flatten();

            if let Some(session) = session {
                let now = chrono::Utc::now().naive_utc();
                if session.expires_at >= now {
                    db::queries::get_user_by_id(pool, session.user_id)
                        .await
                        .ok()
                        .flatten()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(OptionalSession(user))
    }
}

/// GET /api/auth/me — Returns the authenticated user's profile, or 200 OK with null.
///
/// Unlike other protected routes that reject unauthenticated requests with 401,
/// this endpoint returns HTTP 200 with `null` for unauthenticated users so the
/// frontend can distinguish between "not logged in" and "server error".
pub async fn me_handler(
    OptionalSession(user): OptionalSession,
) -> axum::Json<Option<crate::models::UserResponse>> {
    axum::Json(user.map(crate::models::UserResponse::from))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_cookie_value_finds_session() {
        let cookie = "session=abc123; HttpOnly; Secure; SameSite=Lax; Path=/";
        assert_eq!(extract_cookie_value(cookie, "session"), Some("abc123".to_string()));
    }

    #[test]
    fn extract_cookie_value_returns_none_when_missing() {
        let cookie = "other=value; HttpOnly";
        assert_eq!(extract_cookie_value(cookie, "session"), None);
    }

    #[test]
    fn extract_cookie_value_handles_empty_string() {
        assert_eq!(extract_cookie_value("", "session"), None);
    }

    #[test]
    fn extract_cookie_value_handles_multiple_cookies() {
        let cookie = "session=token123; other=xyz; path=/";
        assert_eq!(extract_cookie_value(cookie, "session"), Some("token123".to_string()));
        assert_eq!(extract_cookie_value(cookie, "other"), Some("xyz".to_string()));
        assert_eq!(extract_cookie_value(cookie, "missing"), None);
    }

    #[test]
    fn session_cookie_string_format() {
        let cookie = session_cookie_string("tok123");
        assert!(cookie.starts_with("session=tok123;"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("Secure"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("Max-Age="));
    }
}
