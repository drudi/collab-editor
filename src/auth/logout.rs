//! Logout handler — POST /api/auth/logout
//!
//! Destroys the current user session and clears the session cookie.

use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
};
use sqlx::SqlitePool;

use crate::db;
use crate::error::AppError;

/// POST /api/auth/logout
///
/// Destroys the current session and clears the HttpOnly cookie.
/// Returns 401 if no valid session cookie is present.
pub async fn logout_handler(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    // ── 1. Read the session cookie ──────────────────────────────
    let cookie_header = headers
        .get(axum::http::header::COOKIE)
        .ok_or_else(|| AppError::AuthError("Missing Cookie header".into()))?;

    let cookie_str = cookie_header.to_str().map_err(|_| {
        AppError::AuthError("Cookie header contains invalid characters".into())
    })?;

    let session_token = extract_cookie_value(cookie_str, "session").ok_or_else(|| {
        AppError::AuthError("Missing session cookie".into())
    })?;

    // ── 2. Delete session from database (best-effort) ───────────
    let _ = db::queries::delete_session_by_token(&pool, &session_token).await;

    // ── 3. Clear the cookie ─────────────────────────────────────
    let clear_cookie = format!(
        "session=; Max-Age=0; Path=/; HttpOnly; Secure; SameSite=Lax"
    );

    let mut response: Response = StatusCode::OK.into_response();
    response
        .headers_mut()
        .insert(
            axum::http::header::SET_COOKIE,
            axum::http::header::HeaderValue::from_str(&clear_cookie)
                .expect("Valid cookie header"),
        );

    Ok(response)
}

/// Extract the value of a cookie named `name` from a `Cookie` header string.
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
}
