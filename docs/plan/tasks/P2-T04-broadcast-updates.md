# P2-T04 — Broadcast Updates

**Status:** TODO

## Goal

Implement update broadcasting from CRDT document changes to all connected clients.

## Description

Wire up the yrs update observer to broadcast document changes to all room clients. When any client makes a local edit, the change must be distributed to all other clients in the room.

Implementation:
1. In `ws_handler`, after creating the room state and doc, subscribe to `doc.observe_update_v1`
2. On update event, encode the update data and broadcast to all clients via `RoomState::broadcast`
3. Add backpressure: skip sending to clients with send queue > 100 messages
4. Use `tokio::sync::mpsc` channels per client for non-blocking sends

The observer callback:
```rust
doc.observe_update_v1(move |event| {
    let update = event.data.to_vec();
    room_state.broadcast(WsMessage::Sync(update), None).await;
}).ok();
```

Backpressure check per client:
```rust
if client_tx.len() < 100 {
    client_tx.send(msg).await;
}
```

## Acceptance Criteria

- AC1: `observe_update_v1` subscription on the room's `yrs::Doc`
- AC2: Update data encoded as `WsMessage::Sync(bytes)` on change
- AC3: `RoomState::broadcast` sends to all clients except sender
- AC4: Backpressure check: skip clients with queue > 100 messages
- AC5: Each client has a dedicated `mpsc` send channel
- AC6: Non-blocking send: use `try_send` or check channel length
- AC7: Update event callback is set up after doc is created
- AC8: `cargo check` passes

## Technical Hints

- Use `tokio::sync::mpsc::channel(256)` per client for send buffers
- Backpressure pattern:
  ```rust
  if tx.capacity().unwrap_or(0) < 128 {
      // queue is getting full, skip this update
      return;
  }
  tx.try_send(msg).ok();
  ```
- `observe_update_v1` returns `Result<Subscription, _>` — handle the error gracefully
- Consider using `observe_update_v2` for smaller payloads when possible
- Refer to: https://docs.rs/yrs/latest/yrs/doc/struct.Doc.html#method.observe_update_v1
- Refer to: https://docs.rs/tokio/latest/tokio/sync/mpsc/structSender.html
