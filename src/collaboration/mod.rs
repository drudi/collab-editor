//! Collaboration module — WebSocket handling, room state, and CRDT sync.
//!
//! Manages real-time collaboration via WebSocket connections:
//! - `ws`: WebSocket upgrade handler and per-room connection management
//! - `room_state`: In-memory room state with Yjs document, awareness, and broadcast
//! - `messages`: WebSocket message protocol types

pub mod ws;
pub mod room_state;
pub mod messages;