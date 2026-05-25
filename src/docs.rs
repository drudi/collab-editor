//! API documentation module — OpenAPI / Swagger integration via utoipa.
//!
//! Provides compile-time OpenAPI 3.0 spec generation for all REST endpoints.
//!
//! # Endpoints
//! - `GET /swagger-ui/openapi.json` — Raw OpenAPI 3.0 spec (machine-readable)
//! - `GET /swagger-ui/`             — Interactive Swagger UI (human-readable)
//!
//! # Usage
//! The spec is wired into `build_app()` in `lib.rs` via the
//! `openapi_json()` handler and a Swagger UI HTML page.

use axum::response::Html;
use serde::Serialize;
use utoipa::OpenApi;

use crate::auth::login::{LoginRequest, LoginResponse};
use crate::auth::register::{RegisterRequest, RegisterResponse};
use crate::collaboration::messages::{
    CursorPos, LspDiagnostic, LspDiagnosticSeverity, SelectionRange, WsMessage,
};
use crate::models::{
    Document, Language, RoomMember, RoomMemberType, UserResponse,
};
use crate::rooms::create::{CreateRoomRequest, CreateRoomResponse};
use crate::rooms::get::RoomMetadataResponse;
use crate::rooms::list::RoomInfo;
use crate::rooms::metadata::{RoomMetadataDetail, RoomMemberDetail, UpdateRoomRequest};

/// All API path documents for the application.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Collab Editor API",
        description = "Real-time collaborative code editor with CRDT sync and LSP linting.",
        version = "0.1.0"
    ),
    paths(
        register,
        login,
        me,
        logout,
        create_room,
        list_rooms,
        get_room,
        get_room_metadata,
        update_room,
    ),
    components(
        schemas(
            Language,
            UserResponse,
            RegisterRequest,
            RegisterResponse,
            LoginRequest,
            LoginResponse,
            CreateRoomRequest,
            CreateRoomResponse,
            RoomInfo,
            RoomMetadataResponse,
            RoomMetadataDetail,
            RoomMemberDetail,
            UpdateRoomRequest,
            RoomMemberType,
            RoomMember,
            Document,
            LspDiagnostic,
            LspDiagnosticSeverity,
            CursorPos,
            SelectionRange,
            WsMessage,
        ),
    ),
    tags(
        (name = "Auth", description = "User authentication endpoints"),
        (name = "Rooms", description = "Room management endpoints"),
        (name = "WebSocket", description = "Real-time collaboration protocol"),
    ),
)]
pub struct ApiDoc;

/// Returns the OpenAPI JSON spec for Swagger UI consumption.
pub async fn openapi_json() -> axum::Json<serde_json::Value> {
    let spec = ApiDoc::openapi();
    axum::Json(serde_json::to_value(spec).expect("OpenAPI spec should be serializable"))
}

/// Interactive Swagger UI page served via CDN (no local assets needed).
pub async fn swagger_ui_page() -> Html<String> {
    let html = include_str!("../swagger-ui/index.html").to_string();
    Html(html)
}

// ─── Auth Endpoints ───────────────────────────────────────────────────────

/// Register a new user account.
#[utoipa::path(
    tag = "Auth",
    operation_id = "register",
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse, content_type = "application/json"),
        (status = 400, description = "Validation error", body = Error),
        (status = 409, description = "Username already taken", body = Error),
    ),
)]
#[allow(dead_code)]
#[allow(dead_code)]
fn register() {}

/// Authenticate with username and password.
#[utoipa::path(
    tag = "Auth",
    operation_id = "login",
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse, content_type = "application/json"),
        (status = 401, description = "Invalid credentials", body = Error),
        (status = 400, description = "Validation error", body = Error),
    ),
)]
#[allow(dead_code)]
#[allow(dead_code)]
fn login() {}

/// Get the current authenticated user's profile.
#[utoipa::path(
    tag = "Auth",
    operation_id = "get_me",
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "User profile", body = UserResponse, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = Error),
    ),
)]
#[allow(dead_code)]
#[allow(dead_code)]
fn me() {}

/// Destroy the current session.
#[utoipa::path(
    tag = "Auth",
    operation_id = "logout",
    delete,
    path = "/api/auth/logout",
    responses(
        (status = 200, description = "Session destroyed"),
        (status = 401, description = "No active session", body = Error),
    ),
)]
#[allow(dead_code)]
fn logout() {}

// ─── Room Endpoints ───────────────────────────────────────────────────────

/// Create a new collaborative editing room.
#[utoipa::path(
    tag = "Rooms",
    operation_id = "create_room",
    post,
    path = "/api/rooms",
    request_body = CreateRoomRequest,
    responses(
        (status = 201, description = "Room created", body = CreateRoomResponse, content_type = "application/json"),
        (status = 400, description = "Validation error", body = Error),
        (status = 401, description = "Unauthorized", body = Error),
    ),
)]
#[allow(dead_code)]
fn create_room() {}

/// List all rooms the authenticated user is a member of.
#[utoipa::path(
    tag = "Rooms",
    operation_id = "list_rooms",
    get,
    path = "/api/rooms",
    responses(
        (status = 200, description = "Room list", body = Vec<RoomInfo>, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = Error),
    ),
)]
#[allow(dead_code)]
fn list_rooms() {}

/// Get metadata for a specific room.
#[utoipa::path(
    tag = "Rooms",
    operation_id = "get_room",
    get,
    path = "/api/rooms/{id}",
    params(
        ("id" = u64, Path, description = "Room ID"),
    ),
    responses(
        (status = 200, description = "Room metadata", body = RoomMetadataResponse, content_type = "application/json"),
        (status = 404, description = "Room not found", body = Error),
    ),
)]
#[allow(dead_code)]
fn get_room() {}

/// Get detailed room metadata including version information.
#[utoipa::path(
    tag = "Rooms",
    operation_id = "get_room_metadata",
    get,
    path = "/api/rooms/{id}/metadata",
    params(
        ("id" = u64, Path, description = "Room ID"),
    ),
    responses(
        (status = 200, description = "Detailed room metadata", body = RoomMetadataDetail, content_type = "application/json"),
        (status = 404, description = "Room not found", body = Error),
    ),
)]
#[allow(dead_code)]
fn get_room_metadata() {}

/// Update room properties (owner-only).
#[utoipa::path(
    tag = "Rooms",
    operation_id = "update_room",
    patch,
    path = "/api/rooms/{id}",
    params(
        ("id" = u64, Path, description = "Room ID"),
    ),
    request_body = UpdateRoomRequest,
    responses(
        (status = 200, description = "Room updated", body = RoomMetadataDetail, content_type = "application/json"),
        (status = 400, description = "Validation error", body = Error),
        (status = 401, description = "Unauthorized or not owner", body = Error),
        (status = 404, description = "Room not found", body = Error),
    ),
)]
#[allow(dead_code)]
fn update_room() {}

// ─── WebSocket Protocol Documentation ─────────────────────────────────────

/// WebSocket message types for real-time collaboration.
pub struct WsMessageDoc;

/// LSP diagnostic severity levels.
pub struct DiagnosticSeverityDoc;

// ─── Error Schema ─────────────────────────────────────────────────────────

/// Unified error response format returned by all endpoints.
#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct Error {
    pub error: String,
    pub message: String,
}
