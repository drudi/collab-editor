//! Unified error types — application-wide error handling.
//!
//! Defines the `AppError` enum with `thiserror` derives for all error
//! categories. Each variant implements `IntoResponse` via Axum to produce
//! appropriate HTTP status codes and JSON error bodies.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Kinds of room-related errors for precise status code mapping.
#[derive(Debug, Clone, thiserror::Error)]
pub enum RoomErrorKind {
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
}

/// Application-wide error type covering all domain errors.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Room {0}: {1}")]
    RoomError(RoomErrorKind, String),

    #[error("Collaboration error: {0}")]
    CollabError(String),

    #[error("Lint error: {0}")]
    LintError(String),

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, variant, message) = match &self {
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "ValidationError",
                msg.clone(),
            ),
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, "AuthError", msg.clone()),
            AppError::RoomError(kind, msg) => {
                let status = match kind {
                    RoomErrorKind::NotFound => StatusCode::NOT_FOUND,
                    RoomErrorKind::Conflict => StatusCode::CONFLICT,
                };
                (status, "RoomError", msg.clone())
            }
            AppError::CollabError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "CollabError", msg.clone())
            }
            AppError::LintError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "LintError", msg.clone())
            }
            AppError::DbError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DbError",
                "Internal database error".to_string(),
            ),
            AppError::IoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "IoError",
                "Internal I/O error".to_string(),
            ),
        };

        (
            status,
            axum::Json(json!({
                "error": variant,
                "message": message
            })),
        )
            .into_response()
    }
}