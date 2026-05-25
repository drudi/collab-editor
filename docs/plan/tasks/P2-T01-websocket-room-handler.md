# P2-T01 — WebSocket Room Handler

**Status:** Done

## Goal

Create the WebSocket connection handler for room collaboration.

## Description

Created `src/collaboration/ws.rs` with the `ws_handler(Path(room_id), State(pool))` function that manages a single WebSocket connection for a room. This is the entry point for all real-time collaboration.

Implementation details:
1. Uses `axum::extract::WebSocketUpgrade::on_upgrade` to accept WebSocket connections
2. On connect: verifies room exists, creates/looks up room state, generates unique ClientID
3. Sends initial document state to new client
4. On message: parses `WsMessage` enum and routes to appropriate handler
5. On disconnect: removes client from room, cleans up resources
6. Uses `futures::StreamExt` for `ws.next()` message receiving
7. Uses `tokio::select!` with `biased` mode for concurrent ping/pong and message handling

## Acceptance Criteria

- AC1: `ws_handler(Path(room_id), State(pool), WebSocketUpgrade)` function exists ✅
- AC2: Uses `WebSocketUpgrade::on_upgrade` to accept WebSocket connection ✅
- AC3: Room existence check on connect ✅
- AC4: `yrs::Doc` per room via `RoomState` and global `RoomStore` ✅
- AC5: Unique `ClientId` generated via `yrs::ClientID::random()` ✅
- AC6: Initial state sent via `WsMessage::sync(encode_state_as_update_v1())` ✅
- AC7: Client removal on disconnect ✅
- AC8: Handler wired into router at `/ws/room/{room_id}` via `State<SqlitePool>` ✅
- AC9: `cargo check` passes ✅
- AC10: All ws tests pass ✅

## Technical Hints

- Use `axum::extract::ws::WebSocketUpgrade` for WebSocket handling
- Yrs doc sync: `yrs::sync::Protocol` for initial state exchange
- Store room state in global `RoomStore` via `LazyLock`
- Use `futures::StreamExt` for `ws.next()` message receiving
- Refer to: https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/
- Refer to: https://docs.rs/yrs/latest/yrs/
