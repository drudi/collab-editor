//! Room creation handler — POST /api/rooms
//!
//! Allows authenticated users to create a new collaborative editing room.
//! Generates a unique 6-character alphanumeric code and inserts the room
//! with the user as owner.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rand::distr::Alphanumeric;
use rand::rng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::auth::session::ValidSession;
use crate::db;
use crate::error::{AppError, RoomErrorKind};
use crate::models::Language;

/// Request body for room creation.
#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
}

/// Response body for room creation.
#[derive(Debug, Serialize)]
pub struct CreateRoomResponse {
    pub room_id: i64,
    pub room_code: String,
    pub name: String,
    pub language: String,
}

/// Validate room name: 1-64 characters, non-empty.
fn validate_name(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::ValidationError(
            "Room name must not be empty".to_string(),
        ));
    }
    if name.len() > 64 {
        return Err(AppError::ValidationError(
            "Room name must be at most 64 characters".to_string(),
        ));
    }
    Ok(())
}

/// Validate and normalize language against the `Language` enum.
/// Defaults to 'plaintext' if not provided.
fn validate_language(raw: &str) -> Result<String, AppError> {
    match raw.parse::<Language>() {
        Ok(lang) => Ok(lang.to_string()),
        Err(_) => Err(AppError::ValidationError(format!(
            "Unknown language: {raw}. Valid values: plain_text, javascript, python, rust, typescript"
        ))),
    }
}

/// Generate a unique 6-character alphanumeric room code.
/// Retries up to 5 times on collision.
async fn generate_unique_code(pool: &SqlitePool) -> Result<String, AppError> {
    for _ in 0..5 {
        let code: String = rng()
            .sample_iter::<u8, _>(Alphanumeric)
            .take(6)
            .map(|b| b as char)
            .collect();

        // Check for uniqueness using COUNT
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM rooms WHERE code = ?",
            code
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DbError(e))?;

        if count == 0 {
            return Ok(code);
        }
    }

    Err(AppError::RoomError(
        RoomErrorKind::Conflict,
        "Failed to generate a unique room code".to_string(),
    ))
}

/// POST /api/rooms
///
/// Creates a new collaborative editing room for the authenticated user.
/// Returns 201 with the room data on success.
pub async fn create_room_handler(
    State(pool): State<SqlitePool>,
    ValidSession(user): ValidSession,
    Json(req): Json<CreateRoomRequest>,
) -> Result<axum::response::Response, AppError> {
    // ── Validate name ───────────────────────────────────────────
    let name = req
        .name
        .unwrap_or_else(|| format!("Untitled ({})", chrono::Utc::now().format("%Y-%m-%d %H:%M")));
    validate_name(&name)?;

    // ── Validate / normalize language ───────────────────────────
    let language = match req.language {
        Some(ref lang) => validate_language(lang)?,
        None => "plain_text".to_string(),
    };

    // ── Generate unique room code ───────────────────────────────
    let code = generate_unique_code(&pool).await?;

    // ── Insert room ─────────────────────────────────────────────
    let room = db::queries::create_room(
        &pool,
        &code,
        &name,
        req.description.as_deref(),
        &language,
        user.id,
    )
    .await?;

    // ── Insert owner membership ─────────────────────────────────
    db::queries::add_room_member(&pool, room.id, user.id, "owner")
        .await
        .map_err(|e| AppError::DbError(e))?;

    info!(
        room_id = room.id,
        room_code = %code,
        owner_id = user.id,
        "Room created"
    );

    // ── Build response ──────────────────────────────────────────
    let body = serde_json::to_string(&CreateRoomResponse {
        room_id: room.id,
        room_code: code.clone(),
        name: room.name,
        language: room.language.map(|l| l.to_string()).unwrap_or_else(|| "plain_text".to_string()),
    })
    .expect("Serialize CreateRoomResponse");

    let mut response: axum::response::Response = axum::body::Body::from(body).into_response();
    *response.status_mut() = StatusCode::CREATED;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_name_rejects_empty() {
        assert!(validate_name("").is_err());
    }

    #[test]
    fn validate_name_rejects_too_long() {
        let long = "a".repeat(65);
        assert!(validate_name(&long).is_err());
    }

    #[test]
    fn validate_name_accepts_valid() {
        assert!(validate_name("My Room").is_ok());
        assert!(validate_name("a").is_ok());
        assert!(validate_name(&"a".repeat(64)).is_ok());
    }

    #[test]
    fn validate_language_accepts_valid() {
        assert!(validate_language("plain_text").is_ok());
        assert!(validate_language("javascript").is_ok());
        assert!(validate_language("python").is_ok());
        assert!(validate_language("rust").is_ok());
        assert!(validate_language("typescript").is_ok());
    }

    #[test]
    fn validate_language_rejects_invalid() {
        assert!(validate_language("cobol").is_err());
    }

    #[test]
    fn generate_code_returns_6_chars() {
        use crate::db::connect;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap();

        rt.block_on(async {
            let pool = connect("sqlite::memory:").await.unwrap();
            let code = generate_unique_code(&pool).await.unwrap();
            assert_eq!(code.len(), 6);
            assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
        });
    }
}
