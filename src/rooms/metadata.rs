//! Room metadata — GET /api/rooms/:id/metadata and PATCH /api/rooms/:id
//!
//! Provides detailed room metadata retrieval and room property editing.
//! - GET: Full metadata including owner, members, version, lastSavedAt
//! - PATCH: Edit room name, description, language (owner-only)

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::auth::session::ValidSession;
use crate::db;
use crate::error::{AppError, RoomErrorKind};
use crate::models::{Language, UserResponse};

// ─── Request / Response Types ───────────────────────────────────────────

/// Response body for GET /api/rooms/:id/metadata (P4-T03, AC1-AC2).
#[derive(Debug, Serialize)]
pub struct RoomMetadataDetail {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub owner: UserResponse,
    pub members: Vec<RoomMemberDetail>,
    pub version: i32,
    pub last_saved_at: Option<String>,
}

/// Simplified member representation for the metadata response.
#[derive(Debug, Serialize)]
pub struct RoomMemberDetail {
    pub id: i64,
    pub username: String,
    pub member_type: String,
}

/// Request body for PATCH /api/rooms/:id (P4-T03, AC3-AC6).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateRoomRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
}

// ─── Handlers ───────────────────────────────────────────────────────────

/// GET /api/rooms/:id/metadata — detailed room metadata (P4-T03, AC1-AC2).
///
/// Returns full metadata including version and lastSavedAt from the latest document.
pub async fn get_metadata_handler(
    Path(room_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<RoomMetadataDetail>, AppError> {
    // Fetch room
    let room = db::queries::get_room_by_id(&pool, room_id)
        .await?
        .ok_or_else(|| AppError::RoomError(RoomErrorKind::NotFound, "Room not found".into()))?;

    // Fetch owner
    let owner = db::queries::get_user_by_id(&pool, room.owner_id)
        .await?
        .ok_or_else(|| AppError::DbError(sqlx::Error::RowNotFound))?;
    let owner_response = UserResponse::from(owner);

    // Fetch members as simplified detail
    let members_raw = db::queries::get_room_members_with_user(&pool, room_id).await?;
    let members: Vec<RoomMemberDetail> = members_raw
        .into_iter()
        .map(|m| RoomMemberDetail {
            id: m.id,
            username: m.username,
            member_type: m.member_type.to_string(),
        })
        .collect();

    // Fetch latest document for version and saved_at
    let latest_doc = db::queries::get_latest_document(&pool, room_id).await?;
    let (version, last_saved_at) = latest_doc
        .map(|d| (d.version, Some(d.saved_at.to_string())))
        .unwrap_or((0, None));

    Ok(Json(RoomMetadataDetail {
        id: room.id,
        code: room.code,
        name: room.name,
        description: room.description,
        language: room.language.map(|l| l.to_string()),
        owner: owner_response,
        members,
        version,
        last_saved_at,
    }))
}

/// PATCH /api/rooms/:id — edit room properties (P4-T03, AC3-AC10).
///
/// Only the room owner can edit. Validates language against the Language enum.
pub async fn update_room_handler(
    Path(room_id): Path<i64>,
    State(pool): State<SqlitePool>,
    session: ValidSession,
    Json(req): Json<UpdateRoomRequest>,
) -> Result<Json<RoomMetadataDetail>, AppError> {
    // Fetch room
    let room = db::queries::get_room_by_id(&pool, room_id)
        .await?
        .ok_or_else(|| AppError::RoomError(RoomErrorKind::NotFound, "Room not found".into()))?;

    // Owner authorization check (P4-T03, AC4/AC7)
    if room.owner_id != session.0.id {
        return Err(AppError::AuthError(
            "Only the room owner can edit room properties".into(),
        ));
    }

    // Validate language if provided (P4-T03, AC5/AC8)
    let validated_language = match &req.language {
        Some(lang_str) => {
            let lang = lang_str.parse::<Language>().map_err(|e| {
                AppError::ValidationError(format!(
                    "Invalid language: '{lang_str}'. {e}"
                ))
            })?;
            Some(lang.to_string())
        }
        None => None,
    };

    // Partial update — only update provided fields (P4-T03, AC6)
    // Use COALESCE to only set non-None values
    let name = req.name.as_ref().map(|s| s.as_str());
    let description = req.description.as_ref().map(|s| s.as_str());
    let language = validated_language.as_ref().map(|s| s.as_str());

    // Use raw query (no !) to avoid sqlx compile-time checking for UPDATE
    sqlx::query(
        r#"UPDATE rooms
           SET name = COALESCE($1, name),
               description = COALESCE($2, description),
               language = COALESCE($3, language)
           WHERE id = $4"#,
    )
    .bind(name)
    .bind(description)
    .bind(language)
    .bind(room_id as i64)
    .execute(&pool)
    .await?;

    // Fetch updated room and rebuild the metadata response
    let updated_room = db::queries::get_room_by_id(&pool, room_id)
        .await?
        .ok_or_else(|| AppError::RoomError(RoomErrorKind::NotFound, "Room not found".into()))?;

    // Fetch members
    let members_raw = db::queries::get_room_members_with_user(&pool, room_id).await?;
    let members: Vec<RoomMemberDetail> = members_raw
        .into_iter()
        .map(|m| RoomMemberDetail {
            id: m.id,
            username: m.username,
            member_type: m.member_type.to_string(),
        })
        .collect();

    // Fetch latest document
    let latest_doc = db::queries::get_latest_document(&pool, room_id).await?;
    let (version, last_saved_at) = latest_doc
        .map(|d| (d.version, Some(d.saved_at.to_string())))
        .unwrap_or((0, None));

    let owner = db::queries::get_user_by_id(&pool, updated_room.owner_id)
        .await?
        .ok_or_else(|| AppError::DbError(sqlx::Error::RowNotFound))?;

    Ok(Json(RoomMetadataDetail {
        id: updated_room.id,
        code: updated_room.code,
        name: updated_room.name,
        description: updated_room.description,
        language: updated_room.language.map(|l| l.to_string()),
        owner: UserResponse::from(owner),
        members,
        version,
        last_saved_at,
    }))
}
