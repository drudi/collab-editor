//! collab-editor — Collaborative Code Editor
//!
//! A real-time collaborative code editor with CRDT-based synchronization,
//! LSP-powered linting, and room-based collaboration.

pub mod auth;
pub mod rooms;
pub mod collaboration;
pub mod linter;
pub mod db;
pub mod models;
pub mod error;

use axum::{routing::get, routing::post, routing::delete, routing::patch, Router};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

/// Build the application router with all routes mounted.
///
/// # Current routes
/// - `POST /api/auth/register` — user registration
/// - `POST /api/auth/login`    — user login
/// - `GET  /api/auth/me`       — current user profile
/// - `POST /api/auth/logout`   — logout
/// - `POST /api/rooms`         — create room
/// - `GET  /api/rooms`         — list rooms for current user
/// - `GET  /api/rooms/:id`     — room metadata
/// - `GET  /api/rooms/:id/metadata` — detailed room metadata
/// - `PATCH /api/rooms/:id`    — edit room properties
pub fn build_app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/auth/register", post(auth::register::register_handler))
        .route("/api/auth/login",    post(auth::login::login_handler))
        .route("/api/auth/me",       get(auth::session::me_handler))
        .route("/api/auth/logout",   delete(auth::logout::logout_handler))
        .route("/api/rooms",         post(rooms::create::create_room_handler))
        .route("/api/rooms",         get(rooms::list::list_rooms_handler))
        .route("/api/rooms/{id}",      get(rooms::get::get_room_handler))
        .route("/api/rooms/{id}/metadata", get(rooms::get_metadata_handler))
        .route("/api/rooms/{id}",        patch(rooms::update_room_handler))
        .route("/ws/room/{room_id}",     get(collaboration::ws::ws_handler))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PATCH,
                    axum::http::Method::DELETE,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
}
