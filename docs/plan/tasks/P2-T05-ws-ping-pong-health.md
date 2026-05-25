# P2-T05 — WS Ping/Pong Health

**Status:** Done

## Goal

Implement WebSocket health monitoring with ping/pong.

## Description

Added health monitoring to keep stale WebSocket connections from consuming resources indefinitely. The server uses application-level ping/pong (via `WsMessage::Ping`/`WsMessage::Pong`) to keep connections alive and detect dead clients.

Implementation:
1. 30-second interval task in `ws_handler` sends `WsMessage::Ping`
2. Client replies with `WsMessage::Pong` on receiving a ping
3. `Text(Ping)` message triggers `Text(Pong)` reply
4. `Text(Pong)` message updates awareness
5. On send failure or stream end, the connection loop breaks and client is cleaned up
6. `client_tx` is a `Sender<tungstenite::Message>` for direct message sending
7. `WsMessage` uses struct variants (not tuple) for JSON serialization

Key architectural decisions:
- Application-level ping/pong (not hardware-level) since axum 0.8's `WebSocket` doesn't expose tungstenite's `Ping`/`Pong` variants
- Used `tokio::select!` with `biased` mode to prioritize message receive
- All messages go through `WsMessage` JSON protocol for consistency

## Acceptance Criteria

- AC1: 30-second ping interval task started in `ws_handler` ✅
- AC2: `WsMessage::Ping` sent to client on each interval tick ✅
- AC3: Connection breaks on send failure (ping timeout) ✅
- AC4: `WsMessage::Pong` updates client awareness state ✅
- AC5: `WsMessage::Ping` message triggers `WsMessage::Pong` reply ✅
- AC6: Connection breaks cleanly on stream end or error ✅
- AC7: Client removed from room state on disconnect ✅
- AC8: `cargo check` passes ✅
- AC9: All 3 ws tests pass ✅

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
- axum 0.8's WebSocket doesn't expose hardware ping/pong — use application-level
- Refer to: https://docs.rs/tokio/latest/tokio/time/fn.interval.html
- Refer to: https://docs.rs/tokio/latest/tokio/macro.select.html
