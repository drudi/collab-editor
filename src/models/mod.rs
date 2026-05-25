//! Shared domain models — Serde-serializable structs for the application.
//!
//! Core data types used across HTTP handlers, database queries, and
//! WebSocket messages. All structs derive `Serialize` and `Deserialize`
//! for JSON interchange. Schema types are annotated with `ToSchema`
//! for OpenAPI documentation via utoipa.

use std::fmt;
use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Supported programming languages for editor syntax highlighting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Javascript,
    Python,
    Rust,
    Typescript,
    #[serde(rename = "plain_text")]
    PlainText,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Javascript => write!(f, "javascript"),
            Language::Python => write!(f, "python"),
            Language::Rust => write!(f, "rust"),
            Language::Typescript => write!(f, "typescript"),
            Language::PlainText => write!(f, "plain_text"),
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "javascript" | "js" => Ok(Language::Javascript),
            "python" | "py" => Ok(Language::Python),
            "rust" | "rs" => Ok(Language::Rust),
            "typescript" | "ts" => Ok(Language::Typescript),
            "plain_text" | "plaintext" | "text" | "" => Ok(Language::PlainText),
            _ => Err(format!("Unknown language: {s}")),
        }
    }
}

/// An application user.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

/// API-safe user representation (excludes `password_hash`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub created_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
        }
    }
}

/// An active user session tied to a browser cookie.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

/// A collaborative editing room.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Room {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<Language>,
    pub owner_id: i64,
    pub created_at: NaiveDateTime,
}

/// Rich room metadata returned by the room info endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMetadataResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<Language>,
    pub owner: UserResponse,
    pub members: Vec<RoomMember>,
    pub latest_document_snapshot: Option<Document>,
}

/// Membership type within a room.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum RoomMemberType {
    Owner,
    Editor,
    Viewer,
}

impl fmt::Display for RoomMemberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoomMemberType::Owner => write!(f, "owner"),
            RoomMemberType::Editor => write!(f, "editor"),
            RoomMemberType::Viewer => write!(f, "viewer"),
        }
    }
}

impl std::str::FromStr for RoomMemberType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owner" => Ok(RoomMemberType::Owner),
            "editor" => Ok(RoomMemberType::Editor),
            "viewer" => Ok(RoomMemberType::Viewer),
            other => Err(format!("Unknown member type: {other}")),
        }
    }
}

/// A user's membership in a room.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomMember {
    pub id: i64,
    pub room_id: i64,
    pub user_id: i64,
    pub user: User,
    pub member_type: RoomMemberType,
    pub created_at: NaiveDateTime,
}

/// A snapshot of a room's document content.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Document {
    pub id: i64,
    pub room_id: i64,
    #[serde(with = "serde_bytes")]
    pub content_snapshot: Vec<u8>,
    pub version: i32,
    pub saved_at: NaiveDateTime,
}