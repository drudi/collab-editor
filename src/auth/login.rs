//! Login handler — POST /api/auth/login
//!
//! Authenticates a user by username and password, creates a session,
//! and sets an HttpOnly cookie with the session token.

use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
    Json as AxumJson,
};
use argon2::Argon2;
use argon2::password_hash::PasswordHash;
use argon2::PasswordVerifier;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::db;
use crate::error::AppError;
use crate::models::User;

/// Request body for the login endpoint.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Response body returned on successful login.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: i64,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<User> for LoginResponse {
    fn from(user: User) -> Self {
        LoginResponse {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        }
    }
}

/// Validate username: 3–32 chars, alphanumeric + underscore only.
fn validate_username(username: &str) -> Result<(), AppError> {
    if username.len() < 3 {
        return Err(AppError::ValidationError(
            "Username must be at least 3 characters".to_string(),
        ));
    }
    if username.len() > 32 {
        return Err(AppError::ValidationError(
            "Username must be at most 32 characters".to_string(),
        ));
    }
    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(AppError::ValidationError(
            "Username can only contain letters, numbers, and underscores".to_string(),
        ));
    }
    Ok(())
}

/// Validate password: at least 6 characters.
fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 6 {
        return Err(AppError::ValidationError(
            "Password must be at least 6 characters".to_string(),
        ));
    }
    Ok(())
}

/// Generate a random session token (32 bytes → 64 hex chars).
fn generate_session_token() -> String {
    let bytes: [u8; 32] = rand::random();
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// POST /api/auth/login
///
/// Authenticates a user with username and password. On success, creates a
/// new session and returns the user data with an HttpOnly cookie.
///
/// Returns `401 Unauthorized` for invalid credentials (never reveals whether
/// the username or password was wrong, to prevent username enumeration).
pub async fn login_handler(
    State(pool): State<SqlitePool>,
    AxumJson(req): AxumJson<LoginRequest>,
) -> Result<Response, AppError> {
    // ── Validation ──────────────────────────────────────────────
    validate_username(&req.username)?;
    validate_password(&req.password)?;

    // ── Look up user by username ────────────────────────────────
    let user = db::queries::get_user_by_username(&pool, &req.username)
        .await
        .map_err(|e| AppError::DbError(e))?;

    let user = match user {
        Some(u) => u,
        None => {
            // Never reveal whether username or password was wrong.
            return Err(AppError::AuthError("Invalid credentials".to_string()));
        }
    };

    // ── Verify password with argon2 (constant-time) ─────────────
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::AuthError("Invalid credentials".to_string()))?;
    argon2
        .verify_password(req.password.as_bytes(), &hash)
        .map_err(|_| AppError::AuthError("Invalid credentials".to_string()))?;

    // ── Create session ──────────────────────────────────────────
    let token = generate_session_token();
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("7 days from now")
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();

    db::queries::insert_session(&pool, user.id, &token, &expires_at)
        .await
        .map_err(|e| AppError::DbError(e))?;

    // ── Build response ──────────────────────────────────────────
    info!(username = %user.username, "User logged in successfully");

    let body = serde_json::to_string(&LoginResponse::from(user))
        .expect("Serialize LoginResponse");

    let mut response = Response::new(axum::body::Body::from(body));
    response
        .headers_mut()
        .insert(
            axum::http::header::SET_COOKIE,
            axum::http::header::HeaderValue::from_str(&format!(
                "session={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age={}",
                token,
                7 * 24 * 60 * 60
            ))
            .expect("Valid cookie header"),
        );
    *response.status_mut() = StatusCode::OK;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_username_rejects_too_short() {
        assert!(validate_username("ab").is_err());
    }

    #[test]
    fn validate_username_rejects_special_chars() {
        assert!(validate_username("user@name").is_err());
        assert!(validate_username("user name").is_err());
    }

    #[test]
    fn validate_username_accepts_valid() {
        assert!(validate_username("alice").is_ok());
        assert!(validate_username("a_b_c").is_ok());
    }

    #[test]
    fn validate_password_rejects_short() {
        assert!(validate_password("abc").is_err());
    }

    #[test]
    fn validate_password_accepts_long_enough() {
        assert!(validate_password("abcdef").is_ok());
    }

    #[test]
    fn generate_session_token_returns_64_hex_chars() {
        let token = generate_session_token();
        assert_eq!(token.len(), 64);
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
