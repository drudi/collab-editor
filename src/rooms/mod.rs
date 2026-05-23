//! Rooms module — room CRUD operations and membership management.
//!
//! Provides HTTP handlers for collaborative editing rooms:
//! - `create`: Create a new room (POST /api/rooms)
//! - `get`: Retrieve room metadata (GET /api/rooms/:id)
//! - `metadata`: Update room metadata (PATCH /api/rooms/:id)

pub mod create;
pub mod get;
pub mod metadata;