//! Room listing handler — GET /api/rooms
//!
//! Returns all rooms that the current user is a member of.
//! Unauthenticated requests return an empty list (200 OK).

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::auth::session::OptionalSession;
use crate::db;
use crate::error::AppError;

/// Room info for the list endpoint (lightweight, excludes password_hash).
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RoomInfo {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    #[schema(example = "plain_text", nullable)]
    pub language: Option<String>,
    pub owner_id: i64,
    pub owner_username: String,
    pub created_at: String,
}

/// GET /api/rooms
///
/// Returns all rooms that the authenticated user is a member of.
/// Unauthenticated requests return an empty list (200 OK).
pub async fn list_rooms_handler(
    OptionalSession(user): OptionalSession,
    State(pool): State<SqlitePool>,
) -> Result<Response, AppError> {
    // Unauthenticated — return empty list
    let Some(user) = user else {
        let body = serde_json::to_string(&Vec::<RoomInfo>::new())
            .expect("Serialize empty room list");
        let response: Response = axum::body::Body::from(body).into_response();
        return Ok(response);
    };

    // Fetch all rooms the user is a member of
    let rooms = db::queries::get_rooms_by_user_id(&pool, user.id).await?;

    let body = serde_json::to_string(&rooms)
        .expect("Serialize room list");

    let mut response: Response = axum::body::Body::from(body).into_response();
    *response.status_mut() = StatusCode::OK;

    Ok(response)
}
