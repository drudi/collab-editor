/**
 * useWebSocket — React hook for managing WebSocket connections to a room.
 *
 * Provides:
 * - sendMessage(msg: WsMessage) — send messages to the server
 * - reconnect() — manually trigger reconnection
 * - isConnected: boolean — connection state
 * - onSyncUpdate: callback for received updates
 *
 * Reconnection strategy: exponential backoff
 * 1s → 2s → 4s → 8s → 16s → max 30s
 */

import { useRef, useCallback, useEffect, useState } from 'react';

// ─── Message Types ──────────────────────────────────────────────────────────

export interface SyncMessage {
  type: 'sync';
  data: number[];
}

export interface AwarenessMessage {
  type: 'awareness';
  data: number[];
}

export interface CursorMessage {
  type: 'cursor';
  user_id: string;
  username: string;
  cursor: { line: number; ch: number };
  selection?: { from: { line: number; ch: number }; to: { line: number; ch: number } };
}

export interface PingMessage {
  type: 'Ping';
}

export interface PongMessage {
  type: 'Pong';
}

/** LSP diagnostic received from the server. */
export interface LintDiagnostic {
  line: number;
  column: number;
  end_line?: number;
  end_column?: number;
  severity: number;
  message: string;
}

export interface LintMessage {
  type: 'lint';
  diagnostics: LintDiagnostic[];
}

export type WsMessage =
  | SyncMessage
  | AwarenessMessage
  | CursorMessage
  | PingMessage
  | PongMessage
  | LintMessage;

// ─── Hook Options ───────────────────────────────────────────────────────────

export interface UseWebSocketOptions {
  /** Callback invoked when a sync update is received. */
  onSyncUpdate?: (update: Uint8Array) => void;
  /** Callback invoked when an awareness update is received. */
  onAwarenessUpdate?: (data: unknown) => void;
  /** Callback invoked when a lint message is received. */
  onLintMessage?: (diagnostics: LintDiagnostic[]) => void;
  /** Base URL for the WebSocket server. Defaults to `ws://localhost:3000`. */
  baseUrl?: string;
  /** Maximum reconnect delay in ms. Defaults to 30000. */
  maxDelay?: number;
  /** Initial reconnect delay in ms. Defaults to 1000. */
  initialDelay?: number;
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/** Convert a number array to Uint8Array for Yjs. */
/** Convert a number array to Uint8Array for Yjs. */
function numberArrayToUint8(data: number[]): Uint8Array {
  return new Uint8Array(data);
}

/** Convert a Uint8Array to a number array for JSON serialization. */


/**
 * Encode a message for sending over the WebSocket.
 * Binary data is converted to a number array for JSON transport.
 */
function encodeMessage(msg: WsMessage): string {
  if (msg.type === 'sync') {
    const syncMsg = msg as SyncMessage;
    return JSON.stringify({ type: 'sync', data: syncMsg.data });
  }
  if (msg.type === 'awareness') {
    const awMsg = msg as AwarenessMessage;
    return JSON.stringify({ type: 'awareness', data: awMsg.data });
  }
  return JSON.stringify(msg);
}

// ─── Hook ───────────────────────────────────────────────────────────────────

/**
 * React hook for managing a WebSocket connection to a room.
 *
 * @param roomId - The room ID (room code) to connect to.
 * @param options - Optional configuration.
 * @returns Connection state and helper methods.
 */
export function useWebSocket(
  roomId: string,
  options: UseWebSocketOptions = {},
): {
  sendMessage: (msg: WsMessage) => void;
  reconnect: () => void;
  isConnected: boolean;
} {
  const {
    onSyncUpdate,
    onAwarenessUpdate,
    onLintMessage,
    baseUrl = 'ws://localhost:3000',
    maxDelay = 30000,
    initialDelay = 1000,
  } = options;

  // Refs to avoid re-renders
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<number | null>(null);
  const retryCountRef = useRef(0);
  // Keep onSyncUpdate current for the WebSocket's onmessage handler
  const onSyncUpdateRef = useRef(onSyncUpdate);
  const onAwarenessUpdateRef = useRef(onAwarenessUpdate);
  // Connection state
  const [isConnected, setIsConnected] = useState(false);

  // Update refs when callbacks change
  useEffect(() => {
    onSyncUpdateRef.current = onSyncUpdate;
  }, [onSyncUpdate]);
  useEffect(() => {
    onAwarenessUpdateRef.current = onAwarenessUpdate;
  }, [onAwarenessUpdate]);

  // Keep lint message callback current
  const onLintMessageRef = useRef(onLintMessage);
  useEffect(() => {
    onLintMessageRef.current = onLintMessage;
  }, [onLintMessage]);

  // Clean up on unmount
  useEffect(() => {
    return () => {
      if (reconnectTimeoutRef.current !== null) {
        clearTimeout(reconnectTimeoutRef.current);
        reconnectTimeoutRef.current = null;
      }
      wsRef.current?.close(1000, 'unmount');
    };
  }, []);

  // Reconnect function (exponential backoff)
  const reconnect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN || wsRef.current?.readyState === WebSocket.CONNECTING) {
      return;
    }

    const delay = Math.min(initialDelay * Math.pow(2, retryCountRef.current), maxDelay);
    retryCountRef.current += 1;

    reconnectTimeoutRef.current = window.setTimeout(() => {
      connect();
    }, delay);
  }, [initialDelay, maxDelay]);

  // Connect to the WebSocket server
  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      return;
    }

    const wsUrl = `${baseUrl}/ws/room/${roomId}`;
    const ws = new WebSocket(wsUrl);
    ws.binaryType = 'arraybuffer';

    ws.onopen = () => {
      retryCountRef.current = 0; // Reset retry count on successful connection
      setIsConnected(true);
      console.log('[WS] Connected to', wsUrl);
    };

    ws.onclose = (event) => {
      setIsConnected(false);
      console.log('[WS] Disconnected:', event.code, event.reason);
      // Trigger reconnection if not a clean close
      if (event.code !== 1000) {
        reconnect();
      }
    };

    ws.onerror = (event) => {
      console.error('[WS] Error:', event);
    };

    ws.onmessage = (event) => {
      if (event.data instanceof ArrayBuffer) {
        // Binary message (sync updates)
        const data = new Uint8Array(event.data);
        if (onSyncUpdateRef.current) {
          try {
            onSyncUpdateRef.current(data);
          } catch (err) {
            console.error('[WS] Error processing sync update:', err);
          }
        }
        return;
      }

      // Text message — parse JSON
      let parsed: WsMessage;
      try {
        parsed = JSON.parse(event.data);
      } catch (err) {
        console.error('[WS] Failed to parse message:', event.data);
        return;
      }

      // Handle sync updates from server (binary data encoded as number array)
      if (parsed.type === 'sync' && 'data' in parsed && Array.isArray(parsed.data)) {
        const updateData = numberArrayToUint8(parsed.data as number[]);
        if (onSyncUpdateRef.current) {
          try {
            onSyncUpdateRef.current(updateData);
          } catch (err) {
            console.error('[WS] Error processing sync update:', err);
          }
        }
      }

      // Handle awareness updates from server
      if (parsed.type === 'awareness' && 'data' in parsed && Array.isArray(parsed.data)) {
        const awarenessData = numberArrayToUint8(parsed.data as number[]);
        try {
          const decoded = new TextDecoder().decode(awarenessData);
          const parsedAwareness = JSON.parse(decoded);
          if (onAwarenessUpdateRef.current) {
            onAwarenessUpdateRef.current(parsedAwareness);
          }
        } catch (err) {
          console.error('[WS] Error processing awareness update:', err);
        }
      }

      // Handle pong (ignore, server tracks health)
      if (parsed.type === 'Pong') {
        // Health check confirmed
      }

      // Handle lint diagnostics from server
      if (parsed.type === 'lint' && 'diagnostics' in parsed) {
        const lintMsg = parsed as LintMessage;
        if (onLintMessageRef.current) {
          try {
            onLintMessageRef.current(lintMsg.diagnostics);
          } catch (err) {
            console.error('[WS] Error processing lint message:', err);
          }
        }
      }

      // Handle ping from server — reply with pong
      if (parsed.type === 'Ping') {
        const pong: PongMessage = { type: 'Pong' };
        ws.send(JSON.stringify(pong));
      }
    };

    wsRef.current = ws;
  }, [roomId, baseUrl, reconnect]);

  // Initial connect on mount
  useEffect(() => {
    connect();
  }, [connect]);

  // Send a message to the server
  const sendMessage = useCallback(
    (msg: WsMessage) => {
      if (wsRef.current?.readyState === WebSocket.OPEN) {
        wsRef.current.send(encodeMessage(msg));
      }
    },
    [],
  );

  return {
    sendMessage,
    reconnect,
    isConnected,
  };
}
