# P2-T03 — WS Message Protocol

**Status:** TODO

## Goal

Define the WebSocket message protocol and implement initial sync.

## Description

Create the message type definitions and the handshake protocol for WebSocket communication. This defines how the client and server communicate.

Define `src/collaboration/messages.rs` with the `WsMessage` enum:

```rust
enum WsMessage {
    Sync(Vec<u8>),           // Yjs document update (binary encoded)
    Awareness(Vec<u8>),      // Awareness/state update (binary encoded)
    Cursor { user_id: u32, x: f64, y: f64 },  // Cursor position
    Ping,
    Pong,
}
```

On connect handshake:
1. Send initial full state: `Sync(doc.encode_state_as_update_v1(&StateVector::default()))`
2. Subscribe to `doc.observe_update_v1` for incremental updates
3. Subscribe to `awareness.observe()` for state changes

On message handling:
- `Sync` → apply via `doc.transact_mut().apply_update()`
- `Awareness` → apply via `awareness.set_local_state()` or broadcast
- `Cursor` → update awareness state for this client
- `Ping` → reply `Pong`

## Acceptance Criteria

- AC1: `WsMessage` enum with Sync, Awareness, Cursor, Ping, Pong variants
- AC2: `Serialize`/`Deserialize` derive on `WsMessage` (using serde_json)
- AC3: Initial full state sent on connect
- AC4: `observe_update_v1` subscription for incremental sync
- AC5: `Awareness.observe()` subscription for state changes
- AC6: Sync messages applied to doc via `apply_update()`
- AC7: Awareness messages broadcast to all clients
- AC8: Cursor messages update local awareness state
- AC9: Ping → Pong reply
- AC10: `cargo check` passes

## Technical Hints

- Use `serde_json::to_string(&WsMessage::Sync(data))` for serialization
- For binary data, encode as base64 in the JSON envelope or use a `r#type` field:
  ```json
  {"type": "sync", "data": "base64_encoded_bytes"}
  ```
- Yrs observe pattern:
  ```rust
  doc.observe_update_v1(move |event| {
      // broadcast event.data to all clients
  })
  ```
- State vector for delta sync:
  ```rust
  let local_sv = doc.transact().state_vector();
  let update = doc.transact().encode_diff_v1(&remote_sv)?;
  ```
- Refer to: https://docs.rs/yrs/latest/yrs/doc/struct.Doc.html#method.observe_update_v1
