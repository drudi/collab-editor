//! Room metadata handler — GET /api/rooms/:id
//!
//! Returns full metadata about a room including owner info, member list,
//! and latest document snapshot.

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::db;
use crate::error::{AppError, RoomErrorKind};
use crate::models::{RoomMember, User, UserResponse};

/// Response body for the room metadata endpoint.
#[derive(Debug, Serialize)]
pub struct RoomMetadataResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub owner: UserResponse,
    pub members: Vec<RoomMember>,
    pub latest_document_snapshot: Option<crate::models::Document>,
}

/// GET /api/rooms/:id
///
/// Returns full metadata about a room. Returns 404 if the room doesn't exist.
pub async fn get_room_handler(
    Path(room_id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<RoomMetadataResponse>, AppError> {
    // ── Fetch room ──────────────────────────────────────────────
    let room = db::queries::get_room_by_id(&pool, room_id)
        .await?
        .ok_or_else(|| AppError::RoomError(RoomErrorKind::NotFound, "Room not found".into()))?;

    // ── Fetch owner ─────────────────────────────────────────────
    let owner = db::queries::get_user_by_id(&pool, room.owner_id)
        .await?
        .ok_or_else(|| AppError::DbError(sqlx::Error::RowNotFound))?;

    let owner_response = UserResponse::from(owner);

    // ── Fetch members with full user data ───────────────────────
    let members_raw = db::queries::get_room_members_with_user(&pool, room_id).await?;

    let members: Vec<RoomMember> = members_raw
        .into_iter()
        .map(|m| RoomMember {
            id: m.id,
            room_id: m.room_id,
            user_id: m.user_id,
            user: User {
                id: m.user_id,
                username: m.username,
                password_hash: String::new(),
                created_at: m.user_created_at,
            },
            member_type: m.member_type,
            created_at: m.created_at,
        })
        .collect();

    // ── Fetch latest document snapshot ──────────────────────────
    let latest_doc = db::queries::get_latest_document(&pool, room_id).await?;

    Ok(Json(RoomMetadataResponse {
        id: room.id,
        code: room.code,
        name: room.name,
        description: room.description,
        language: room.language.map(|l| l.to_string()),
        owner: owner_response,
        members,
        latest_document_snapshot: latest_doc,
    }))
}
