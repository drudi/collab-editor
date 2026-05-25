# P2-T06 — Frontend WebSocket Hook

**Status:** Done

## Goal

Create the `useWebSocket` React hook for managing WebSocket connections.

## Description

Create `src/hooks/useWebSocket.ts` — a React hook that manages the WebSocket connection to a room, including automatic reconnection with exponential backoff.

The hook provides:
- `sendMessage(msg: WsMessage)` — send messages to the server
- `reconnect()` — manually trigger reconnection
- `isConnected: boolean` — connection state
- `onSyncUpdate: (update: Uint8Array) => void` — callback for received updates

Reconnection strategy:
- Start with 1-second delay
- Double on each retry (1s → 2s → 4s → 8s → 16s → max 30s)
- Attempt reconnection indefinitely until the component unmounts
- Clean up WebSocket on unmount

Connection URL: `ws://localhost:3000/ws/room/{roomId}` (configurable via env or props).

## Acceptance Criteria

- AC1: `useWebSocket` hook function defined
- AC2: WebSocket URL constructed as `/ws/room/{roomId}`
- AC3: `sendMessage` function sends encoded `WsMessage`
- AC4: `isConnected` boolean reflects connection state
- AC5: Exponential backoff reconnection: 1s → 2s → 4s → 8s → 16s → max 30s
- AC6: WebSocket cleaned up on unmount (close connection, cancel timeout)
- AC7: `onSyncUpdate` callback invoked with received update data
- AC8: Hook uses `useRef` for WebSocket to prevent re-renders
- AC9: TypeScript types for `WsMessage` and update callbacks
- AC10: `cargo check` (no lint errors in TS)

## Technical Hints

- Hook pattern:
  ```typescript
  function useWebSocket(roomId: string, onSyncUpdate: (update: Uint8Array) => void) {
    const wsRef = useRef<WebSocket | null>(null);
    const reconnectTimeoutRef = useRef<number | null>(null);
    // ...
  }
  ```
- Reconnection with exponential backoff:
  ```typescript
  const delay = Math.min(1000 * Math.pow(2, retryCount), 30000);
  reconnectTimeoutRef.current = window.setTimeout(reconnect, delay);
  ```
- Use `WebSocket.binaryType = 'arraybuffer'` for binary messages
- Message send: `ws.send(JSON.stringify({ type: 'sync', data: Array.from(update) }))`
- Refer to: https://react.dev/reference/react/useRef
- Refer to: https://developer.mozilla.org/en-US/docs/Web/API/WebSocket
