# P2-T10 — Cursor Sharing: Server

**Status:** TODO

## Goal

Implement server-side cursor position tracking and broadcasting via Yjs Awareness.

## Description

Extend `RoomState` to track and broadcast cursor positions and selection ranges for all connected clients. This enables the "show other people's cursors" feature.

Implementation:
1. Use `yrs::sync::Awareness` for cursor state management
2. On receiving a `CursorMessage` from a client: update the awareness state for that client
3. Broadcast the delta (changed fields) to all other clients
4. Clean up stale cursors (client not seen in 10s)

The awareness state structure per client:
```rust
struct CursorState {
    user_id: u32,
    username: String,
    cursor_pos: Position,  // {line, ch}
    selection: Option<Range>,  // {from, to}
    color: String,  // deterministic color from user_id
}
```

Broadcast format:
```json
{
  "type": "awareness",
  "data": {
    "user_id": 123,
    "cursor": { "line": 5, "ch": 10 },
    "selection": { "from": { "line": 5, "ch": 10 }, "to": { "line": 5, "ch": 15 } }
  }
}
```

## Acceptance Criteria

- AC1: `Awareness` from `yrs::sync::Awareness` used in `RoomState`
- AC2: `CursorMessage` updates awareness state for the sender's client
- AC3: Awareness delta broadcast to all other clients
- AC4: Stale cursors cleaned up after 10s inactivity
- AC5: Deterministic cursor color from user_id (e.g., HSL color)
- AC6: Broadcast only sends changed fields (delta format)
- AC7: `cargo check` passes

## Technical Hints

- Awareness set/update:
  ```rust
  awareness.update_peer_state(client_id, Some(state), |_| {}).ok();
  let states = awareness.get_states();
  ```
- HSL color from ID: `hsl((user_id % 360), 70%, 60%)`
- Use `serde_json` to encode awareness state as `Vec<u8>`
- Stale cleanup: check `last_seen` timestamp on each ping interval
- Refer to: https://docs.rs/yrs/latest/yrs/sync/struct.Awareness.html
