# P2-T01 — WebSocket Room Handler

**Status:** TODO

## Goal

Create the WebSocket connection handler for room collaboration.

## Description

Create `src/collaboration/ws.rs` with the `ws_handler(roomId: String)` function that manages a single WebSocket connection for a room. This is the entry point for all real-time collaboration.

On connect:
1. Verify room exists via database
2. Create a `yrs::Doc` for the room (or use existing one)
3. Create a unique `ClientId` for this connection
4. Add the client to the room's member list
5. Send initial document state via `encode_state_as_update_v1`

On disconnect:
1. Remove client from room
2. Clean up resources

On message:
1. Parse the incoming `WsMessage` enum
2. Route to appropriate handler based on message type

## Acceptance Criteria

- AC1: `ws_handler(room_id: String)` function exists in `src/collaboration/ws.rs`
- AC2: Uses `tokio_tungstenite::accept_async` to accept WebSocket connection
- AC3: Room existence check on connect
- AC4: `yrs::Doc` created per room for CRDT management
- AC5: Unique `ClientId` generated for each connection via `yrs::ClientID::random()`
- AC6: Initial state sent via `doc.transact().encode_state_as_update_v1(&StateVector::default())`
- AC7: Client removal on disconnect
- AC8: Handler wired into router at `/ws/room/{room_id}`
- AC9: `cargo check` passes

## Technical Hints

- Use `axum::extract::ws::WebSocketUpgrade` for WebSocket handling:
  ```rust
  async fn ws_handler(
    upgrade: WebSocketUpgrade,
    Path(room_id): Path<String>,
  ) -> impl IntoResponse {
    upgrade.on_upgrade(|ws| async { handle_ws(ws, room_id).await })
  }
  ```
- Yrs doc sync: `yrs::sync::Protocol` for initial state exchange
- Store room state in `Arc<Mutex<HashMap<String, RoomState>>>` or use an AppState
- Refer to: https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/
- Refer to: https://docs.rs/yrs/latest/yrs/
