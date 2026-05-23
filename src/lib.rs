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

use axum::Router;

/// Build the application router with all routes mounted.
///
/// Currently returns an empty `Router`. Routes will be added as modules
/// are implemented across Phase 1.
pub fn build_app() -> Router {
    Router::new()
}