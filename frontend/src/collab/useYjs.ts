/**
 * useYjs — React hook that manages the Yjs document for a room.
 *
 * Bridges the WebSocket communication with the CRDT document state:
 * 1. Creates a Y.Doc instance
 * 2. Gets/creates Y.getText('content') shared type
 * 3. Gets/creates Y.getMap('awareness') for cursor state
 * 4. On sync message received: applies update to Yjs doc
 * 5. On Yjs doc update: sends update via WebSocket
 * 6. Cleans up Y.Doc on unmount
 */

import { useRef, useEffect, useMemo } from 'react';
import * as Y from 'yjs';

// Message type for sending sync updates
interface SyncMessage {
  type: 'sync';
  data: number[];
}

export interface UseYjsResult {
  /** The shared text type — the collaborative document content. */
  ytext: Y.Text;
  /** The awareness map for cursor/selection state. */
  awareness: Y.Map<unknown>;
  /** The underlying Y.Doc instance. */
  doc: Y.Doc;
  /** Apply a received sync update to the Yjs document. */
  applyUpdate: (data: Uint8Array) => void;
}

/**
 * Create a Yjs document bound to a room.
 *
 * @param roomId - The room ID to bind to.
 * @param sendMessage - Optional callback to send sync updates to the server.
 * @returns Result containing the Yjs types and helper methods.
 */
export function useYjs(roomId: string, sendMessage?: (msg: string | Uint8Array) => void): UseYjsResult {
  // Create Y.Doc once per hook lifetime
  const ydocRef = useRef<Y.Doc | null>(new Y.Doc());
  const ytextRef = useRef<Y.Text | null>(null);
  const awarenessRef = useRef<Y.Map<unknown> | null>(null);

  // Create Yjs types once
  if (!ytextRef.current && ydocRef.current) {
    ytextRef.current = ydocRef.current.getText('content');
  }
  if (!awarenessRef.current && ydocRef.current) {
    awarenessRef.current = ydocRef.current.getMap('awareness');
  }

  // Track update handler for cleanup
  const updateHandlerRef = useRef<((update: Uint8Array) => void) | null>(null);
  const applyUpdateRef = useRef<((data: Uint8Array) => void) | null>(null);

  useEffect(() => {
    const doc = ydocRef.current;
    const ytext = ytextRef.current;
    if (!doc || !ytext) return;

    // Set up update listener: send local changes to the server
    const updateHandler = (update: Uint8Array) => {
      if (sendMessage) {
        const msg: SyncMessage = {
          type: 'sync',
          data: Array.from(update),
        };
        sendMessage(JSON.stringify(msg));
      }
    };
    updateHandlerRef.current = updateHandler;
    doc.on('update', updateHandler);

    // Set up applyUpdate ref for external sync
    const applyUpdate = (data: Uint8Array) => {
      Y.applyUpdate(doc, data);
      // Update awareness with the new state
      ytext.doc?.on('afterTransaction', () => {
        // Awareness state will be updated by the awareness system
      });
    };
    applyUpdateRef.current = applyUpdate;

    return () => {
      // Clean up update listener
      doc.off('update', updateHandler);
      updateHandlerRef.current = null;
      applyUpdateRef.current = null;

      // Destroy the Y.Doc (keep refs to avoid recreation issues)
      doc.destroy();
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [roomId, sendMessage]);

  // Return memoized result
  return useMemo(
    () => ({
      ytext: ytextRef.current!,
      awareness: awarenessRef.current!,
      doc: ydocRef.current!,
      applyUpdate: applyUpdateRef.current!,
    }) as UseYjsResult,
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [ytextRef.current, awarenessRef.current, ydocRef.current, applyUpdateRef.current],
  );
}
