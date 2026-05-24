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

use axum::{routing::get, routing::post, Router};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

/// Build the application router with all routes mounted.
///
/// # Current routes
/// - `POST /api/auth/register` — user registration
/// - `POST /api/auth/login`    — user login
///
/// # Development
/// The server binds to `0.0.0.0:3000`. CORS is configured for
/// `http://localhost:5173` (Vite dev server) — tighten in production.
pub fn build_app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/auth/register", post(auth::register::register_handler))
        .route("/api/auth/login",    post(auth::login::login_handler))
        .route("/api/auth/me",       get(auth::session::me_handler))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::OPTIONS,
                ])
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
}
