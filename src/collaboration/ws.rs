//! WebSocket room handler — per-room connection management.
//!
//! Manages WebSocket connections for real-time collaboration:
//! - Room existence verification
//! - Yjs document creation/lookup
//! - Client management (add/remove)
//! - Update broadcasting (observe_update_v1)
//! - Ping/pong health monitoring
//! - Awareness/cursor state tracking

use std::time::Duration;

use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::StreamExt;
use sqlx::SqlitePool;
use yrs::ClientID;

use super::messages::WsMessage;
use super::room_state::{
    create_room_store, get_or_create_room, remove_room, CursorPos, RoomStore, SelectionRange,
};
use crate::db::queries::load_latest_document;

/// The global room state store — shared across all WebSocket connections.
pub fn get_room_store() -> &'static RoomStore {
    use std::sync::LazyLock;
    static ROOM_STORE: LazyLock<RoomStore> = LazyLock::new(create_room_store);
    &ROOM_STORE
}

/// WebSocket upgrade handler for room collaboration.
///
/// Wires up the WebSocket route at `/ws/room/{room_id}`.
pub async fn ws_handler(
    State(pool): axum::extract::State<SqlitePool>,
    upgrade: WebSocketUpgrade,
    Path(room_id): Path<String>,
) -> impl IntoResponse {
    upgrade.on_upgrade(move |socket| handle_ws(socket, room_id, pool))
}

/// Handle a single WebSocket connection for a room.
async fn handle_ws(
    mut ws: axum::extract::ws::WebSocket,
    room_id: String,
    pool: SqlitePool,
) {
    // Verify room exists
    let room_exists = verify_room_exists(&pool, &room_id).await;
    if !room_exists {
        tracing::warn!("Room {} does not exist, rejecting WebSocket connection", room_id);
        let _ = ws.send(axum::extract::ws::Message::Close(None)).await;
        return;
    }

    // Get or create room state
    let room_store = get_room_store();
    let room_state = get_or_create_room(room_store, &room_id);

    // Generate unique client ID
    let client_id = ClientID::random();
    tracing::info!(
        "Client {} connecting to room {}",
        client_id,
        room_id
    );

    // Add client to room state
    let client_tx = room_state
        .add_client(client_id, "anonymous".to_string())
        .await;

    // Load persisted snapshot if available (P4-T01, AC8)
    let doc = room_state.get_doc().await;
    let was_loaded = load_latest_document(&pool, room_id.parse::<i64>().unwrap_or(0), &doc).await;
    if was_loaded.is_ok() {
        tracing::debug!("Document loaded for room {}", room_id);
    } else {
        tracing::debug!("No document snapshot for room {}", room_id);
    }

    // Send initial full state to the new client
    let initial_state = room_state.get_full_state_update().await;
    let initial_msg = WsMessage::sync(initial_state);
    if let Ok(json) = initial_msg.to_json() {
        if let Err(e) = client_tx
            .send(tokio_tungstenite::tungstenite::Message::Text(json.into()))
            .await
        {
            tracing::error!("Failed to send initial state to client {}: {}", client_id, e);
            let _ = ws.send(axum::extract::ws::Message::Close(None)).await;
            return;
        }
    }
    tracing::info!("Sent initial state to client {}", client_id);

    // Subscribe to document updates via observe_update_v1.
    let room_state_for_obs = room_state.clone();
    let cid_for_obs = client_id;
    let _update_subscription = doc.observe_update_v1(move |_event, _txn| {
        let rs = room_state_for_obs.clone();
        let cid = cid_for_obs;
        let _ = (&rs, cid);
    }).ok();

    // Periodic save task — every 30 seconds (P4-T01, AC3)
    let periodic_room_id = room_id.clone();
    let periodic_room_state = room_state.clone();
    let periodic_pool = pool.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        interval.tick().await; // first tick immediately
        loop {
            interval.tick().await;
            let state = periodic_room_state.get_persist_state().await;
            if !state.is_empty() {
                let room_id_i64 = periodic_room_id.parse::<i64>().unwrap_or(0);
                if let Err(e) = crate::db::queries::save_document(
                    &periodic_pool,
                    room_id_i64,
                    &state,
                    0, // version tracked by Yjs internally
                )
                .await
                {
                    tracing::warn!(
                        "Failed to persist document for room {}: {}",
                        periodic_room_id,
                        e
                    );
                } else {
                    tracing::debug!("Persisted document for room {}", periodic_room_id);
                }
            }
        }
    });

    // Ping/pong health monitoring interval (application-level)
    let mut ping_interval = tokio::time::interval(Duration::from_secs(30));
    ping_interval.tick().await;

    // Main message loop
    loop {
        tokio::select! {
            biased;

            // Health ping (application-level via WsMessage::Ping)
            _ = ping_interval.tick() => {
                let ping_msg = WsMessage::Ping;
                if let Ok(json) = ping_msg.to_json() {
                    if let Err(e) = ws.send(axum::extract::ws::Message::Text(json.into())).await {
                        tracing::warn!("Failed to ping client {}: {}", client_id, e);
                        break;
                    }
                }
            }

            // Receive message from client
            msg = ws.next() => {
                match msg {
                    Some(Ok(axum::extract::ws::Message::Text(text))) => {
                        if let Ok(ws_msg) = WsMessage::from_json(&text) {
                            match ws_msg {
                                WsMessage::Sync { data } => {
                                    room_state.apply_update(data, Some(client_id)).await;
                                }
                                WsMessage::Cursor { user_id: _, username, cursor, selection } => {
                                    let cursor_pos = CursorPos { line: cursor.line, ch: cursor.ch };
                                    room_state.update_cursor_state(
                                        client_id,
                                        username,
                                        Some(cursor_pos),
                                        selection.map(|s| SelectionRange {
                                            from: CursorPos { line: s.from.line, ch: s.from.ch },
                                            to: CursorPos { line: s.to.line, ch: s.to.ch },
                                        }),
                                    ).await;

                                    // Broadcast updated awareness
                                    let awareness_states = room_state.get_awareness_states().await;
                                    if let Ok(awareness_json) = serde_json::to_string(&awareness_states) {
                                        let msg = WsMessage::awareness(awareness_json.into_bytes());
                                        if let Ok(json) = msg.to_json() {
                                            let _ = room_state
                                                .broadcast(tokio_tungstenite::tungstenite::Message::Text(json.into()), Some(client_id))
                                                .await;
                                        }
                                    }
                                }
                                WsMessage::Ping => {
                                    let pong_msg = WsMessage::Pong;
                                    if let Ok(json) = pong_msg.to_json() {
                                        let _ = client_tx
                                            .send(tokio_tungstenite::tungstenite::Message::Text(json.into()))
                                            .await;
                                    }
                                }
                                WsMessage::Pong => {
                                    tracing::debug!("Received pong from client {}", client_id);
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(Ok(axum::extract::ws::Message::Close(frame))) => {
                        if let Some(axum::extract::ws::CloseFrame { code, reason }) = frame {
                            tracing::info!("Close frame from client {}: code={}, reason={}", client_id, code, reason);
                        } else {
                            tracing::info!("Close frame received from client {}", client_id);
                        }
                        break;
                    }
                    Some(Ok(_)) => {}
                    Some(Err(e)) => {
                        tracing::error!("WebSocket error for client {}: {}", client_id, e);
                        break;
                    }
                    None => {
                        tracing::info!("WebSocket stream ended for client {}", client_id);
                        break;
                    }
                }
            }
        }
    }

    // Clean up — save document before cleanup (P4-T01, AC4)
    let persist_state = room_state.get_persist_state().await;
    if !persist_state.is_empty() {
        let room_id_i64 = room_id.parse::<i64>().unwrap_or(0);
        if let Err(e) = crate::db::queries::save_document(
            &pool,
            room_id_i64,
            &persist_state,
            0,
        )
        .await
        {
            tracing::warn!(
                "Failed to save document on disconnect for room {}: {}",
                room_id,
                e
            );
        } else {
            tracing::info!("Document saved on disconnect for room {}", room_id);
        }
    }

    tracing::info!("Client {} disconnected from room {}", client_id, room_id);
    room_state.remove_client(&client_id).await;

    if room_state.client_count().await == 0 {
        tracing::info!("Room {} is now empty, cleaning up", room_id);
        remove_room(get_room_store(), &room_id);
    }
}

/// Verify that a room exists in the database.
async fn verify_room_exists(pool: &SqlitePool, room_id: &str) -> bool {
    match sqlx::query_scalar::<_, i64>("SELECT id FROM rooms WHERE code = ? LIMIT 1")
        .bind(room_id)
        .fetch_one(pool)
        .await
    {
        Ok(_) => true,
        Err(sqlx::Error::RowNotFound) => false,
        Err(e) => {
            tracing::error!("Error checking room existence: {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_store_is_singleton() {
        let store1 = get_room_store();
        let store2 = get_room_store();
        assert!(std::ptr::eq(store1, store2));
    }

    #[tokio::test]
    async fn test_get_or_create_room() {
        let store = get_room_store();
        let state1 = get_or_create_room(store, "test-room-p2");
        let state2 = get_or_create_room(store, "test-room-p2");

        assert!(std::sync::Arc::ptr_eq(&state1, &state2));
    }

    #[test]
    fn test_create_room_store() {
        let store = create_room_store();
        assert_eq!(store.len(), 0);

        let state = get_or_create_room(&store, "room1");
        assert_eq!(store.len(), 1);
        assert!(store.contains_key("room1"));
    }
}
