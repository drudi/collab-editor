# P2-T05 — WS Ping/Pong Health

**Status:** TODO

## Goal

Implement WebSocket health monitoring with ping/pong.

## Description

Add health monitoring to keep stale WebSocket connections from consuming resources indefinitely. This ensures the server cleans up dead connections and maintains accurate awareness state.

Implementation:
1. Start a 30-second interval task in `ws_handler`
2. Every 30s, send `Ping` to the client
3. If client doesn't reply with `Pong` within 30s, close the connection
4. On `Text(Ping)` message, reply with `Text(Pong)`
5. On `Text(Pong)` message, update `last_seen` timestamp in awareness

The health check task:
```rust
let mut interval = tokio::time::interval(Duration::from_secs(30));
loop {
    interval.tick().await;
    stream.send(Message::Ping(Bytes::from("health"))).await?;
}
```

## Acceptance Criteria

- AC1: 30-second ping interval task started in `ws_handler`
- AC2: `Message::Ping` sent to client on each interval tick
- AC3: `Pong` timeout after 30s closes the connection
- AC4: `Text(Pong)` message updates client's `last_seen` timestamp
- AC5: `Text(Ping)` message triggers `Pong` reply
- AC6: Connection closed cleanly on timeout via `stream.close().await`
- AC7: Client removed from room state on disconnect
- AC8: `cargo check` passes

## Technical Hints

- Use `tokio::time::interval(Duration::from_secs(30))`
- Combine ping task with message receive loop using `tokio::select!`:
  ```rust
  tokio::select! {
      biased; // prefer message receive
      _ = interval.tick() => { /* send ping */ }
      msg = stream.next() => { /* handle message */ }
  }
  ```
- `biased` mode ensures message receive is checked first (prevents starvation)
- Connection cleanup: use `drop(stream)` or `stream.close().await`
- Refer to: https://docs.rs/tokio/latest/tokio/time/fn.interval.html
- Refer to: https://docs.rs/tokio/latest/tokio/macro.select.html
