//! Registration handler — POST /api/auth/register
//!
//! Creates a new user account, hashes the password with argon2id,
//! generates a session token, and sets an HttpOnly cookie.

use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
    Json as AxumJson,
};
use argon2::PasswordHasher;
use argon2::password_hash::SaltString;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::db;
use crate::error::{AppError, RoomErrorKind};
use crate::models::User;

/// Request body for the registration endpoint.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

/// Response body returned on successful registration.
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: i64,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<User> for RegisterResponse {
    fn from(user: User) -> Self {
        RegisterResponse {
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

/// POST /api/auth/register
///
/// Accepts a username and password, validates input, hashes the password
/// with argon2id, creates the user and an initial session, and sets
/// an HttpOnly cookie with the session token.
pub async fn register_handler(
    State(pool): State<SqlitePool>,
    AxumJson(req): AxumJson<RegisterRequest>,
) -> Result<Response, AppError> {
    // ── Validation ──────────────────────────────────────────────
    validate_username(&req.username)?;
    validate_password(&req.password)?;

    // ── Check for existing username ─────────────────────────────
    let existing = db::queries::get_user_by_username(&pool, &req.username)
        .await
        .map_err(|e| AppError::DbError(e))?;
    if existing.is_some() {
        return Err(AppError::RoomError(
            RoomErrorKind::Conflict,
            "Username already taken".to_string(),
        ));
    }

    // ── Hash password with argon2id ─────────────────────────────
    let salt_bytes: [u8; 16] = rand::random();
    let salt = SaltString::encode_b64(&salt_bytes)
        .expect("base64 encoding of random salt should not fail");
    let argon2 = argon2::Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::default(),
    );
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::AuthError(format!("Failed to hash password: {e}")))?;

    // ── Insert user ─────────────────────────────────────────────
    let user_id = db::queries::create_user(&pool, &req.username, &password_hash)
        .await
        .map_err(|e| AppError::DbError(e))?;

    // ── Create initial session ──────────────────────────────────
    let token = generate_session_token();
    let expires_at = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("7 days from now")
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();

    db::queries::insert_session(&pool, user_id, &token, &expires_at)
        .await
        .map_err(|e| AppError::DbError(e))?;

    // ── Build response ──────────────────────────────────────────
    let user = db::queries::get_user_by_username(&pool, &req.username)
        .await
        .map_err(|e| AppError::DbError(e))?
        .ok_or_else(|| {
            AppError::AuthError("User was created but could not be retrieved".into())
        })?;

    info!(username = %req.username, "User registered successfully");

    let body = serde_json::to_string(&RegisterResponse::from(user))
        .expect("Serialize RegisterResponse");

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
    *response.status_mut() = StatusCode::CREATED;

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
