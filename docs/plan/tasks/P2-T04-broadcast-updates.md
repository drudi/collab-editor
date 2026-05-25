# P2-T04 — Broadcast Updates

**Status:** Done

## Goal

Implement update broadcasting from CRDT document changes to all connected clients.

## Description

Wired up the yrs update observer to broadcast document changes to all room clients. When any client makes a local edit, the change is distributed to all other clients in the room via `WsMessage::Sync`.

Implementation:
1. In `ws_handler`, after creating the room state, subscribe to `doc.observe_update_v1`
2. On update event, encode the update data and broadcast to all clients via `RoomState::broadcast`
3. `RoomState::broadcast` has built-in backpressure: skips clients with send queue > 100 messages
4. Uses `tokio::sync::mpsc::channel(256)` per client for non-blocking sends

Observer callback:
```rust
doc.observe_update_v1(move |event, _txn| {
    let update = event.update.to_vec();
    let rs = room_state_clone.clone();
    let cid = client_id;
    tokio::spawn(async move {
        let msg = WsMessage::sync(update);
        if let Ok(json) = msg.to_json() {
            let _ = rs.broadcast(TungMessage::Text(json.into()), Some(cid)).await;
        }
    });
}).ok();
```

Backpressure in `RoomState::broadcast`:
```rust
if client_tx.len() >= 100 {
    tracing::warn!("Client {client_id} send queue full ({})", client_tx.len());
    return 0;
}
```

## Acceptance Criteria

- AC1: `observe_update_v1` subscription on the room's `yrs::Doc` ✅
- AC2: Update data encoded as `WsMessage::Sync(bytes)` on change ✅
- AC3: `RoomState::broadcast` sends to all clients except sender ✅
- AC4: Backpressure check: skip clients with queue > 100 messages ✅
- AC5: Each client has a dedicated `mpsc` send channel ✅
- AC6: Non-blocking send via channel capacity check ✅
- AC7: Update event callback is set up after doc is created ✅
- AC8: `cargo check` passes ✅
- AC9: Tests for broadcast in room_state pass ✅

## Technical Hints

- Use `tokio::sync::mpsc::channel(256)` per client for send buffers
- Backpressure pattern: check `client_tx.len()` before sending
- `observe_update_v1` returns `Result<Subscription, _>` — handle the error gracefully
- Consider using `observe_update_v2` for smaller payloads when possible
- Refer to: https://docs.rs/yrs/latest/yrs/doc/struct.Doc.html#method.observe_update_v1
- Refer to: https://docs.rs/tokio/latest/tokio/sync/mpsc/structSender.html
