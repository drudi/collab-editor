//! Linter module — LSP server integration and lint service.
//!
//! Provides language server protocol integration for real-time diagnostics:
//! - `server`: Spawn and manage external LSP processes
//! - `protocol`: LSP JSON-RPC message types and serialization
//! - `service`: Bridge between room collaboration events and LSP diagnostics

pub mod server;
pub mod protocol;
pub mod service;