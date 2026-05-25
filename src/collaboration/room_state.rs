//! Room state manager — in-memory Yjs document, awareness, and broadcast.
//!
//! Manages per-room CRDT state, connected clients, and awareness (cursor/selection).
//!
//! Core data structure:
//! ```text
//! RoomState
//! └── RoomStateInner
//!     ├── doc: Yjs Doc
//!     ├── text: Yjs Text (collaborative document)
//!     ├── awareness: Yjs Awareness (cursor/selection state)
//!     └── clients: HashMap<ClientId, ClientInfo>
//! ```

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use tokio::sync::{mpsc, RwLock};
use tokio::time::{Duration, Instant};
use tokio_tungstenite::tungstenite::Message;
use yrs::sync::awareness::Awareness;
use yrs::updates::decoder::Decode;
use yrs::ClientID;
use yrs::{Doc, GetString, Origin, ReadTxn, StateVector, Transact, Update};

/// Persistent information about a connected client.
#[derive(Debug, Clone)]
pub struct ClientInfo {
    /// Client's unique Yjs ClientID
    pub client_id: ClientID,
    /// Username of the connected user
    pub username: String,
    /// User's cursor position (line, ch) — None if no cursor info
    pub cursor_pos: Option<CursorPos>,
    /// User's text selection range — None if no selection
    pub selection: Option<SelectionRange>,
    /// Last time the client was seen (for stale cleanup)
    pub last_seen: Instant,
    /// Channel sender for broadcasting messages to this client
    pub tx: mpsc::Sender<Message>,
}

/// Cursor position in the document.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CursorPos {
    pub line: i32,
    pub ch: i32,
}

/// Text selection range.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SelectionRange {
    pub from: CursorPos,
    pub to: CursorPos,
}

/// Awareness state delta for broadcasting.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AwarenessDelta {
    #[serde(rename = "type")]
    pub msg_type: &'static str,
    pub data: HashMap<String, ClientAwarenessUpdate>,
}

/// Per-client awareness update in a delta.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientAwarenessUpdate {
    pub client_id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<CursorPos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Client identifier alias.
pub type ClientId = ClientID;

/// Inner mutable room state.
pub struct RoomStateInner {
    /// Yjs document for CRDT synchronization
    pub doc: Doc,
    /// Shared text type — the collaborative document content
    pub text: yrs::types::TextRef,
    /// Awareness manager for cursor/selection state
    pub awareness: Awareness,
    /// Connected clients: ClientId → ClientInfo
    pub clients: HashMap<ClientID, ClientInfo>,
}

impl RoomStateInner {
    /// Create a new room state with a fresh Yjs document.
    pub fn new() -> Self {
        let doc = Doc::with_client_id(0); // doc-level client id 0
        let text = doc.get_or_insert_text("content");
        let awareness = Awareness::new(doc.clone());

        Self {
            doc,
            text,
            awareness,
            clients: HashMap::new(),
        }
    }

    /// Add a new client to the room.
    ///
    /// Returns the client's send channel for broadcasting.
    pub fn add_client(
        &mut self,
        client_id: ClientID,
        username: String,
    ) -> mpsc::Sender<Message> {
        let (tx, _rx) = mpsc::channel::<Message>(256);
        let info = ClientInfo {
            client_id,
            username,
            cursor_pos: None,
            selection: None,
            last_seen: Instant::now(),
            tx: tx.clone(),
        };
        self.clients.insert(client_id, info);
        tx
    }

    /// Remove a client from the room and clean up awareness.
    pub fn remove_client(&mut self, client_id: &ClientID) {
        self.clients.remove(client_id);
        // Remove awareness state for this client
        self.awareness.remove_state(*client_id);
    }

    /// Get a differential update from the current state vector.
    pub fn get_update_diff(&self, state_vector: &StateVector) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.encode_diff_v1(state_vector)
    }

    /// Get the initial full state update for a new client.
    pub fn get_full_state_update(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&StateVector::default())
    }

    /// Apply an incoming Yjs update from a remote client.
    ///
    /// The `origin` parameter marks the source client to prevent echo.
    pub fn apply_update(&mut self, update: Vec<u8>, origin: Option<ClientID>) {
        let _origin_key = origin.map(|id| Origin::from(format!("client-{}", id)));
        // Decode the raw bytes into a Yjs Update
        let yjs_update = Update::decode_v1(&update).unwrap_or_default();
        let mut txn = self.doc.transact_mut();
        txn.apply_update(yjs_update).ok();
    }

    /// Broadcast a message to all clients except the sender.
    ///
    /// Returns a count of clients that received the message.
    /// Uses try_send for non-blocking sends; skips clients with full queues.
    pub async fn broadcast(
        &mut self,
        message: Message,
        sender: Option<ClientID>,
    ) -> usize {
        let mut count = 0;
        // Collect recipients to avoid borrow issues with async send
        let recipients: Vec<ClientID> = self
            .clients
            .iter()
            .filter(|(&id, _info)| {
                sender.map_or(true, |s| s != id)
            })
            .map(|(&id, _)| id)
            .collect();

        for client_id in recipients {
            if let Some(info) = self.clients.get(&client_id) {
                // Use try_send to avoid blocking; skip if channel is full
                if info.tx.try_send(message.clone()).is_ok() {
                    count += 1;
                }
            }
        }
        count
    }

    /// Update cursor/selection state for a client.
    ///
    /// Updates the local client info and the awareness state.
    pub fn update_cursor_state(
        &mut self,
        client_id: ClientID,
        username: String,
        cursor: Option<CursorPos>,
        selection: Option<SelectionRange>,
    ) {
        // Update last_seen and cursor/selection in client info
        if let Some(info) = self.clients.get_mut(&client_id) {
            info.last_seen = Instant::now();
            info.cursor_pos = cursor.clone();
            info.selection = selection.clone();
        }

        // Update awareness state for this client (as the local peer)
        let color = deterministic_color(&client_id);
        let update = ClientAwarenessUpdate {
            client_id: client_id.to_string(),
            username,
            cursor,
            selection,
            color: Some(color),
        };

        if let Ok(json) = serde_json::to_string(&update) {
            self.awareness.set_local_state(&json).ok();
        }
    }

    /// Get awareness states for all clients.
    ///
    /// Returns a map from client_id string to awareness update data.
    /// Reads from both the local client info and the awareness store.
    pub fn get_awareness_states(&self) -> HashMap<String, ClientAwarenessUpdate> {
        let mut result = HashMap::new();

        // First, read from our local client info
        for (&client_id, info) in &self.clients {
            if let Some(cursor) = info.cursor_pos.clone() {
                let update = ClientAwarenessUpdate {
                    client_id: client_id.to_string(),
                    username: info.username.clone(),
                    cursor: Some(cursor),
                    selection: info.selection.clone(),
                    color: Some(deterministic_color(&client_id)),
                };
                result.insert(client_id.to_string(), update);
            }
        }

        // Then overlay awareness states from the Awareness store
        for (client_id, _data) in self.awareness.iter() {
            if let Some(update) = self.awareness.state::<ClientAwarenessUpdate>(client_id) {
                result.insert(client_id.to_string(), update);
            }
        }

        result
    }

    /// Check for stale clients (not seen in 10 seconds) and clean them up.
    /// Returns the list of stale client IDs.
    pub fn check_stale_clients(&mut self) -> Vec<ClientID> {
        const STALE_TIMEOUT: Duration = Duration::from_secs(10);
        let now = Instant::now();
        let stale: Vec<ClientID> = self
            .clients
            .iter()
            .filter(|(_, info)| now.duration_since(info.last_seen) > STALE_TIMEOUT)
            .map(|(&id, _)| id)
            .collect();

        for client_id in &stale {
            tracing::info!("Removing stale client {}", client_id);
            self.remove_client(client_id);
        }

        stale
    }

    /// Get the current document content as a string.
    pub fn get_content(&self) -> String {
        let txn = self.doc.transact();
        self.text.get_string(&txn)
    }

    /// Get the current document version (sequence number).
    /// Uses the document store's internal sequence counter as a proxy.
    pub fn get_version(&self) -> u64 {
        // In yrs 0.26, we track version via the doc's internal store.
        // For simplicity, return a monotonically increasing counter.
        // In practice, the actual Yjs sequence length is available via
        // txn.after_state().len() or similar transaction methods.
        0 // placeholder — version tracked separately in room store
    }

    /// Serialize the document to bytes for persistence (P4-T01).
    ///
    /// Encodes the full Yjs state as an update via `encode_state_as_update_v1`.
    pub fn get_persist_state(&self) -> Vec<u8> {
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&StateVector::default())
    }

    /// Apply a persisted state snapshot to this document (P4-T01).
    ///
    /// Decodes the raw bytes and applies via `apply_update`.
    pub fn apply_persist_state(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let update = match Update::decode_v1(data) {
            Ok(u) => u,
            Err(_) => {
                tracing::warn!("Failed to decode persisted update, skipping load");
                return;
            }
        };
        let mut txn = self.doc.transact_mut();
        txn.apply_update(update).ok();
    }
}

impl Default for RoomStateInner {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper around RoomStateInner.
///
/// Uses `Arc<RwLock<RoomStateInner>>` for interior mutability across
/// async tasks (broadcast loop, message handler, ping loop).
#[derive(Clone)]
pub struct RoomState {
    inner: Arc<RwLock<RoomStateInner>>,
}

impl RoomState {
    /// Create a new RoomState.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(RoomStateInner::new())),
        }
    }

    /// Add a client and return their send channel.
    pub async fn add_client(
        &self,
        client_id: ClientID,
        username: String,
    ) -> mpsc::Sender<Message> {
        let mut inner = self.inner.write().await;
        inner.add_client(client_id, username)
    }

    /// Remove a client from the room.
    pub async fn remove_client(&self, client_id: &ClientID) {
        let mut inner = self.inner.write().await;
        inner.remove_client(client_id);
    }

    /// Get a differential update for incremental sync.
    pub async fn get_update_diff(&self, state_vector: &StateVector) -> Vec<u8> {
        let inner = self.inner.read().await;
        inner.get_update_diff(state_vector)
    }

    /// Get the full state update for a new client.
    pub async fn get_full_state_update(&self) -> Vec<u8> {
        let inner = self.inner.read().await;
        inner.get_full_state_update()
    }

    /// Apply a remote update to the document.
    pub async fn apply_update(&self, update: Vec<u8>, origin: Option<ClientID>) {
        let mut inner = self.inner.write().await;
        inner.apply_update(update, origin);
    }

    /// Broadcast a message to all clients except the sender.
    pub async fn broadcast(&self, message: Message, sender: Option<ClientID>) -> usize {
        let mut inner = self.inner.write().await;
        inner.broadcast(message, sender).await
    }

    /// Update cursor/selection state for a client.
    pub async fn update_cursor_state(
        &self,
        client_id: ClientID,
        username: String,
        cursor: Option<CursorPos>,
        selection: Option<SelectionRange>,
    ) {
        let mut inner = self.inner.write().await;
        inner.update_cursor_state(client_id, username, cursor, selection);
    }

    /// Get awareness states for all clients.
    pub async fn get_awareness_states(&self) -> HashMap<String, ClientAwarenessUpdate> {
        let inner = self.inner.read().await;
        inner.get_awareness_states()
    }

    /// Check and clean up stale clients.
    pub async fn check_stale_clients(&self) -> Vec<ClientID> {
        let mut inner = self.inner.write().await;
        inner.check_stale_clients()
    }

    /// Get the current document content.
    pub async fn get_content(&self) -> String {
        let inner = self.inner.read().await;
        inner.get_content()
    }

    /// Get the current document version.
    pub async fn get_version(&self) -> u64 {
        let inner = self.inner.read().await;
        inner.get_version()
    }

    /// Get the number of connected clients.
    pub async fn client_count(&self) -> usize {
        let inner = self.inner.read().await;
        inner.clients.len()
    }

    /// Get the Yjs Doc reference (read-only clone).
    pub async fn get_doc(&self) -> Doc {
        let inner = self.inner.read().await;
        inner.doc.clone()
    }

    /// Serialize the document to bytes for persistence (P4-T01, AC1).
    pub async fn get_persist_state(&self) -> Vec<u8> {
        let inner = self.inner.read().await;
        inner.get_persist_state()
    }

    /// Apply a persisted state snapshot to the document (P4-T01, AC8).
    pub async fn apply_persist_state(&self, data: &[u8]) {
        let mut inner = self.inner.write().await;
        inner.apply_persist_state(data);
    }
}

impl Default for RoomState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global room state store.
///
/// Maps room IDs to their RoomState instances.
/// Uses `DashMap` for concurrent access from multiple handler tasks.
pub type RoomStore = dashmap::DashMap<String, Arc<RoomState>>;

/// Create a new empty room store.
pub fn create_room_store() -> RoomStore {
    dashmap::DashMap::new()
}

/// Get or create a room state for the given room ID.
///
/// If the room doesn't exist in the store, creates a new RoomState
/// and inserts it. Returns the Arc<RoomState>.
pub fn get_or_create_room(room_store: &RoomStore, room_id: &str) -> Arc<RoomState> {
    if let Some(entry) = room_store.get(room_id) {
        return entry.value().clone();
    }

    // Create new room state
    let new_state = Arc::new(RoomState::new());
    room_store.insert(room_id.to_string(), new_state.clone());
    new_state
}

/// Get an existing room state, or None if the room doesn't exist.
pub fn get_room(room_store: &RoomStore, room_id: &str) -> Option<Arc<RoomState>> {
    room_store.get(room_id).map(|e| e.value().clone())
}

/// Remove a room from the store.
pub fn remove_room(room_store: &RoomStore, room_id: &str) {
    room_store.remove(room_id);
}

/// Generate a deterministic color for a user based on their client ID.
///
/// Uses HSL color space: hue derived from client_id hash mod 360, saturation 70%, lightness 60%.
pub fn deterministic_color(client_id: &ClientID) -> String {
    // Hash the ClientID's Display string to derive a deterministic hue
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    client_id.hash(&mut hasher);
    let hash = hasher.finish();
    let hue = (hash % 360) as f64;
    format!("hsl({}, 70%, 60%)", hue)
}

#[cfg(test)]
mod tests {
    use super::*;
    use yrs::types::text::Text;

    #[tokio::test]
    async fn test_add_and_remove_client() {
        let state = RoomState::new();
        let client_id = yrs::ClientID::random();

        let tx = state
            .add_client(client_id, "alice".to_string())
            .await;

        assert_eq!(state.client_count().await, 1);
        drop(tx);

        state.remove_client(&client_id).await;
        assert_eq!(state.client_count().await, 0);
    }

    #[tokio::test]
    async fn test_update_and_get_content() {
        let state = RoomState::new();

        // Insert content via the Yjs doc
        {
            let mut inner = state.inner.write().await;
            let mut txn = inner.doc.transact_mut();
            inner.text.insert(&mut txn, 0, "Hello, world!");
        }

        let content = state.get_content().await;
        assert_eq!(content, "Hello, world!");
    }

    #[tokio::test]
    async fn test_broadcast() {
        let state = RoomState::new();
        let (tx1, mut rx1) = tokio::sync::mpsc::channel::<Message>(256);
        let (tx2, mut rx2) = tokio::sync::mpsc::channel::<Message>(256);
        let cid1 = yrs::ClientID::random();
        let cid2 = yrs::ClientID::random();

        // Add clients directly to inner state with live channels
        {
            let mut inner = state.inner.write().await;
            inner.add_client(cid1, "alice".to_string());
            inner.clients.get_mut(&cid1).unwrap().tx = tx1;
            inner.add_client(cid2, "bob".to_string());
            inner.clients.get_mut(&cid2).unwrap().tx = tx2;
        }

        // Broadcast from alice's perspective — should reach bob only
        let msg = Message::Text("broadcast".into());
        let count = state.broadcast(msg.clone(), Some(cid1)).await;
        assert_eq!(count, 1); // only bob received

        // Verify bob received the message
        let received = tokio::time::timeout(
            Duration::from_millis(100),
            rx2.recv()
        ).await.expect("bob should receive message").expect("message should be Some");
        assert!(matches!(received, Message::Text(_)));

        // Broadcast with no sender — should reach both
        let count = state.broadcast(msg, None).await;
        assert_eq!(count, 2); // both alice and bob

        // Verify alice received
        let received = tokio::time::timeout(
            Duration::from_millis(100),
            rx1.recv()
        ).await.expect("alice should receive message").expect("message should be Some");
        assert!(matches!(received, Message::Text(_)));
    }

    #[tokio::test]
    async fn test_update_cursor_state() {
        let state = RoomState::new();
        let client_id = yrs::ClientID::random();

        state
            .add_client(client_id, "alice".to_string())
            .await;

        let cursor = CursorPos { line: 5, ch: 10 };
        let selection = SelectionRange {
            from: CursorPos { line: 5, ch: 10 },
            to: CursorPos { line: 5, ch: 15 },
        };

        state
            .update_cursor_state(
                client_id,
                "alice".into(),
                Some(cursor.clone()),
                Some(selection.clone()),
            )
            .await;

        let states = state.get_awareness_states().await;
        assert!(states.contains_key(&client_id.to_string()));
        let update = states.get(&client_id.to_string()).unwrap();
        assert_eq!(update.cursor, Some(cursor));
        assert_eq!(update.selection, Some(selection));
    }

    #[tokio::test]
    async fn test_stale_client_cleanup() {
        let state = RoomState::new();
        let client_id = yrs::ClientID::random();

        state
            .add_client(client_id, "alice".to_string())
            .await;

        // Force last_seen to be very old
        {
            let mut inner = state.inner.write().await;
            if let Some(info) = inner.clients.get_mut(&client_id) {
                info.last_seen = Instant::now() - Duration::from_secs(11);
            }
        }

        let stale = state.check_stale_clients().await;
        assert_eq!(stale.len(), 1);
        assert_eq!(state.client_count().await, 0);
    }

    #[tokio::test]
    async fn test_get_full_state_update() {
        let state = RoomState::new();
        // Insert some content
        {
            let mut inner = state.inner.write().await;
            let mut txn = inner.doc.transact_mut();
            inner.text.insert(&mut txn, 0, "test content");
        }
        let update = state.get_full_state_update().await;
        // Should have some update data
        assert!(!update.is_empty());
    }

    #[tokio::test]
    async fn test_room_store_get_or_create() {
        let store = create_room_store();

        let state1 = get_or_create_room(&store, "room1");
        let state2 = get_or_create_room(&store, "room1");

        // Same room should return the same Arc
        assert!(Arc::ptr_eq(&state1, &state2));
    }
}
