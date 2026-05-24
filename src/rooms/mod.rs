//! Rooms module — room CRUD operations and membership management.
//!
//! Provides HTTP handlers for collaborative editing rooms:
//! - `create`: Create a new room (POST /api/rooms)
//! - `get`: Retrieve room metadata (GET /api/rooms/:id)
//! - `list`: List rooms for current user (GET /api/rooms)

pub mod create;
pub mod get;
pub mod list;
pub mod metadata;