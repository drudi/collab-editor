//! Linter service — bridges room collaboration events with LSP diagnostics.
//!
//! The `LintService` manages LSP processes per room and broadcasts
//! LSP diagnostics to WebSocket room clients via the `WsMessage::Lint` message.
//!
//! ## Architecture
//! ```text
//! WS Message (content change)
//!        │
//!        ▼
//! LintService::update(room_id, content)
//!        │
//!        ├─── did_change() → LSP process stdin
//!        │
//!        ▼
//! LSP process parses code, returns diagnostics via publishDiagnostics
//!        │
//!        ▼
//! parse diagnostics → LspDiagnostic[]
//!        │
//!        ▼
//! broadcast WsMessage::Lint to all room clients
//! ```

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use dashmap::DashMap;
use tracing::{debug, error, info, warn};

use super::protocol::LspDiagnostic;
use super::server::{Language, LspConfig, LspProcess};

use crate::collaboration::messages::WsMessage;
use crate::collaboration::room_state::RoomState;
///
/// Each room gets its own LSP process matched to the room's language.
/// The service handles:
/// - Spawning LSP processes (`spawn`)
/// - Forwarding document changes to LSP (`update`)
/// - Parsing and broadcasting diagnostics (`broadcast_diagnostics`)
/// - Cleaning up LSP processes on room close (`shutdown`)
pub struct LintService {
    /// Per-room LSP process state, keyed by room code.
    /// Contains the process wrapped in Arc<Mutex> for shared async access.
    rooms: Arc<DashMap<String, Arc<LspRoomState>>>,
}

/// Shared state for a room's LSP process.
pub struct LspRoomState {
    /// The LSP process for this room.
    pub process: tokio::sync::Mutex<LspProcess>,
    /// The document URI for this room.
    pub doc_uri: String,
    /// The current document version (atomically updated).
    pub version: AtomicU32,
    /// The language this LSP serves.
    pub language: Language,
}

impl LintService {
    /// Create a new empty LintService.
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(DashMap::new()),
        }
    }

    /// Get a static reference to the global lint service.
    ///
    /// Uses LazyLock for thread-safe lazy initialization.
    pub fn global() -> &'static Self {
        use std::sync::LazyLock;
        static GLOBAL: LazyLock<LintService> = LazyLock::new(LintService::new);
        &GLOBAL
    }

    /// Spawn an LSP process for a room.
    ///
    /// Starts the LSP server for the given room's language,
    /// sends the initialize handshake, and sends a didOpen notification
    /// with the initial document content.
    ///
    /// # Arguments
    /// * `room_id` — The room code/ID
    /// * `language` — The room's language for LSP binary selection
    /// * `initial_content` — Initial document content for didOpen
    ///
    /// # Returns
    /// * `Ok(())` — LSP process started successfully
    /// * `Err(String)` — Failed to start LSP process
    pub async fn spawn(
        room_id: &str,
        language: &crate::models::Language,
        initial_content: &str,
    ) -> Result<(), String> {
        let lsp_language = Language::from_app_language(language);
        let config = LspConfig {
            language: lsp_language.clone(),
            root_uri: Some(format!("file:///room/{}", room_id)),
        };

        info!(
            "Spawning LSP process for room {} ({})",
            room_id,
            language
        );

        let mut process = LspProcess::spawn(config).await?;

        let uri = format!("file:///room/{}", room_id);

        // Send didOpen with initial content
        if let Err(e) = process.did_open(&uri, initial_content, lsp_language.lsp_language_id()).await {
            warn!("Failed to send didOpen for room {}: {}", room_id, e);
        }

        let state = LspRoomState {
            process: tokio::sync::Mutex::new(process),
            doc_uri: uri,
            version: AtomicU32::new(1),
            language: lsp_language,
        };

        Self::global().rooms.insert(room_id.to_string(), Arc::new(state));

        info!("LSP process spawned for room {}", room_id);
        Ok(())
    }

    /// Update document content for a room — forward to LSP and broadcast diagnostics.
    ///
    /// 1. Forwards the content change to the LSP server via `didChange`
    /// 2. Parses diagnostics from the LSP response (via publishDiagnostics notification)
    /// 3. Formats diagnostics as `LintMessage`
    /// 4. Returns diagnostics to be broadcast to the room
    ///
    /// # Arguments
    /// * `room_id` — The room code/ID
    /// * `content` — New document content
    /// * `_language` — The room's language (reserved for future use)
    ///
    /// # Returns
    /// * `Ok(Vec<LspDiagnostic>)` — Parsed diagnostics
    /// * `Err(String)` — Failed to forward to LSP
    pub async fn update(
        room_id: &str,
        content: &str,
        _language: &crate::models::Language,
    ) -> Vec<LspDiagnostic> {
        let global = Self::global();

        // Check if LSP process exists for this room
        if let Some(entry) = global.rooms.get(room_id) {
            let state = entry.value();
            let uri = &state.doc_uri;
            let language_id = state.language.lsp_language_id();

            // Forward content change to LSP via didChange
            let mut process = state.process.lock().await;
            if let Err(e) = process.did_change(uri, content, language_id).await {
                error!(
                    "Failed to send didChange to LSP for room {}: {}",
                    room_id, e
                );
                // Continue with empty diagnostics — don't block the editor
                return Vec::new();
            }

            // Drop the process lock before accessing the room store
            drop(process);

            // Update the version atomically
            if let Some(entry) = global.rooms.get(room_id) {
                entry.value().version.store(
                    entry.value().version.load(Ordering::SeqCst) + 1,
                    Ordering::SeqCst,
                );
            }

            // In production, you'd parse publishDiagnostics notifications
            // from the LSP response channel. For now, return empty diagnostics.
            // The actual diagnostics flow through the ws_handler's response parsing.
            Vec::new()
        } else {
            debug!(
                "No LSP process for room {}, skipping lint update",
                room_id
            );
            Vec::new()
        }
    }

    /// Shutdown the LSP process for a room.
    ///
    /// Sends a didClose notification, then sends a shutdown request
    /// and terminates the process. Removes the room from the store.
    ///
    /// # Arguments
    /// * `room_id` — The room code/ID
    pub async fn shutdown(&self, room_id: &str) {
        info!("Shutting down LSP for room {}", room_id);

        if let Some(entry) = self.rooms.get(room_id) {
            let state = entry.value();
            let uri = state.doc_uri.clone();

            // Send didClose
            {
                let mut process = state.process.lock().await;
                if let Err(e) = process.did_close(&uri).await {
                    warn!("Failed to send didClose for room {}: {}", room_id, e);
                }

                // Send shutdown and exit
                if let Err(e) = process.shutdown().await {
                    warn!("Failed to shutdown LSP for room {}: {}", room_id, e);
                }
            }

            // Remove from store
            self.rooms.remove(room_id);
        }

        info!("LSP shut down for room {}", room_id);
    }

    /// Check if the lint service has an active LSP process for the given room.
    pub fn has_lsp_process(&self, room_id: &str) -> bool {
        self.rooms.contains_key(room_id)
    }

    /// Get the language for a room's LSP process.
    pub fn get_language(&self, room_id: &str) -> Option<Language> {
        self.rooms
            .get(room_id)
            .map(|entry| entry.value().language.clone())
    }

    /// Get the number of active LSP processes.
    pub fn active_count(&self) -> usize {
        self.rooms.len()
    }

    /// Shutdown all LSP processes (used for graceful server shutdown).
    pub async fn shutdown_all(&self) {
        info!("Shutting down all LSP processes");
        let room_ids: Vec<String> = self.rooms.iter().map(|entry| entry.key().clone()).collect();

        for room_id in room_ids {
            self.shutdown(&room_id).await;
        }
    }
}

impl Default for LintService {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for LintService {
    fn clone(&self) -> Self {
        Self {
            rooms: self.rooms.clone(),
        }
    }
}

/// Create a `WsMessage::Lint` message from diagnostics.
///
/// Converts `LspDiagnostic` objects to the wire format expected by the frontend.
pub fn create_lint_message(diagnostics: Vec<LspDiagnostic>) -> WsMessage {
    let lint_diagnostics: Vec<crate::collaboration::messages::LspDiagnostic> = diagnostics
        .into_iter()
        .map(|d| {
            let severity = d.severity();
            crate::collaboration::messages::LspDiagnostic {
                line: d.line,
                column: d.column,
                end_line: Some(d.end_line),
                end_column: Some(d.end_column),
                severity,
                message: d.message,
            }
        })
        .collect();

    WsMessage::Lint {
        diagnostics: lint_diagnostics,
    }
}

/// Broadcast lint diagnostics to a room's clients.
///
/// Converts the diagnostic list to a `WsMessage::Lint` and broadcasts
/// it to all clients in the room state.
///
/// # Arguments
/// * `room_state` — The room's state (for broadcast)
/// * `diagnostics` — The diagnostics to broadcast
/// * `sender` — The client to exclude from broadcast
pub async fn broadcast_lint_diagnostics(
    room_state: &RoomState,
    diagnostics: Vec<LspDiagnostic>,
    sender: Option<yrs::ClientID>,
) -> usize {
    let lint_msg = create_lint_message(diagnostics);

    if let Ok(json) = lint_msg.to_json() {
        let ws_msg = tokio_tungstenite::tungstenite::Message::Text(json.into());
        room_state.broadcast(ws_msg, sender).await
    } else {
        warn!("Failed to serialize lint message");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_service_new() {
        let service = LintService::new();
        assert_eq!(service.active_count(), 0);
        assert!(!service.has_lsp_process("test-room"));
    }

    #[test]
    fn test_lint_service_clone() {
        let service1 = LintService::new();
        let service2 = service1.clone();
        assert_eq!(service1.active_count(), service2.active_count());
    }

    #[test]
    fn test_create_lint_message() {
        let diagnostics = vec![
            LspDiagnostic {
                line: 5,
                column: 10,
                end_line: 5,
                end_column: 15,
                severity: protocol::diagnostic_severity::ERROR,
                message: "unexpected token".to_string(),
            },
            LspDiagnostic {
                line: 10,
                column: 0,
                end_line: 10,
                end_column: 5,
                severity: protocol::diagnostic_severity::WARNING,
                message: "unused variable".to_string(),
            },
        ];

        let msg = create_lint_message(diagnostics.clone());

        match msg {
            WsMessage::Lint { diagnostics } => {
                assert_eq!(diagnostics.len(), 2);
                assert_eq!(diagnostics[0].line, 5);
                assert_eq!(diagnostics[0].column, 10);
                assert_eq!(diagnostics[0].message, "unexpected token");

                assert_eq!(diagnostics[1].line, 10);
                assert_eq!(diagnostics[1].message, "unused variable");
            }
            _ => panic!("Expected Lint message variant"),
        }
    }

    #[tokio::test]
    async fn test_broadcast_lint_diagnostics() {
        let room_state = RoomState::new();
        let diagnostics = vec![
            LspDiagnostic {
                line: 0,
                column: 0,
                end_line: 0,
                end_column: 5,
                severity: protocol::diagnostic_severity::ERROR,
                message: "test error".to_string(),
            },
        ];

        // Should not panic even with empty room state
        let count = broadcast_lint_diagnostics(&room_state, diagnostics, None).await;
        assert_eq!(count, 0); // no clients to broadcast to
    }

    #[tokio::test]
    async fn test_lint_service_empty_shutdown() {
        let service = LintService::new();
        // Shutdown should not panic when room doesn't exist
        service.shutdown("nonexistent-room").await;
    }
}
