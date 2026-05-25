# P2-T02 — Room State Manager

**Status:** Done

## Goal

Create the RoomState struct that manages per-room CRDT state and client connections.

## Description

Create `src/collaboration/room_state.rs` with the `RoomState` struct that wraps all per-room state. This is the core data structure for the collaboration layer.

```
RoomStateInner {
    doc: Doc,                    // yrs::Doc for CRDT
    text: TextRef,               // Shared text document
    awareness: Awareness,        // User cursor/selection state
    clients: HashMap<ClientId, ClientInfo>,  // Connected clients
}
```

`ClientInfo` stores:
- `cursor_pos: Option<Position>`
- `selection: Option<Range>`
- `username: String`
- `last_seen: Instant`

Key methods:
- `add_client(client_id, username) -> Stream` — add and return write half of WS
- `remove_client(client_id)` — remove and clean up
- `get_update_diff(state_vector: StateVector) -> Vec<u8>` — encode differential update
- `apply_update(update: Vec<u8>, origin: Option<ClientId>)` — apply remote update
- `broadcast(update: Vec<u8>, sender: ClientId)` — send to all except sender

## Acceptance Criteria

- AC1: `RoomState` struct defined with `Arc<RwLock<RoomStateInner>>` wrapping
- AC2: `add_client` method returns client's write stream
- AC3: `remove_client` method cleans up client from awareness
- AC4: `get_update_diff` encodes differential update using yrs state vector
- AC5: `apply_update` applies update via `doc.transact_mut().apply_update()`
- AC6: `broadcast` sends update to all clients except sender
- AC7: Awareness stores cursor position per client
- AC8: `cargo check` passes

## Technical Hints

- Use `dashmap::DashMap<String, Arc<RoomState>>` for the global room store
- `Awareness` from `yrs::sync::Awareness` handles peer state
- Use `tokio::sync::RwLock` for interior mutability
- Broadcasting pattern:
  ```rust
  for (id, info) in &inner.clients {
      if id != &sender {
          info.stream.send(ws_message).await?;
      }
  }
  ```
- Refer to: https://docs.rs/yrs/latest/yrs/sync/struct.Awareness.html
- Refer to: https://docs.rs/dashmap/latest/dashmap/
