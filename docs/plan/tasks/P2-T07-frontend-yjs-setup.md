# P2-T07 — Frontend Yjs Setup

**Status:** Done

## Goal

Create the `useYjs` hook that manages the Yjs document and syncs with WebSocket.

## Description

Create `src/collab/useYjs.ts` — a React hook that sets up and manages the Yjs document for a room. This hook bridges the WebSocket communication with the CRDT document state.

The hook:
1. Creates a `Y.Doc` instance
2. Gets or creates the `Y.getText('content')` shared type
3. Gets or creates `Y.getMap('awareness')` for cursor state
4. On WebSocket sync message received: `Y.applyUpdate(doc, updateData)`
5. On `doc.on('update')`: `ws.sendMessage(SyncMessage(updateData))`

The `Y.Doc` is created once per room and persists for the lifetime of the room page. The text shared type `content` is the collaborative document that CodeMirror binds to.

## Acceptance Criteria

- AC1: `useYjs(roomId)` hook function defined
- AC2: `Y.Doc` created on hook initialization
- AC3: `Y.getText('content')` shared type obtained
- AC4: `Y.getMap('awareness')` shared type obtained
- AC5: `doc.on('update')` listener sends updates via WebSocket
- AC6: Received sync updates applied via `Y.applyUpdate(doc, data)`
- AC7: `Y.Doc` cleaned up on unmount
- AC8: Returns `YText` reference for editor binding
- AC9: TypeScript types for return value
- AC10: No lint errors

## Technical Hints

- Hook pattern:
  ```typescript
  function useYjs(roomId: string, ws: WebSocket) {
    const ydocRef = useRef<Y.Doc>(new Y.Doc());
    const ytextRef = useRef<Y.Text>(ydocRef.current.getText('content'));
    
    useEffect(() => {
      const updateHandler = (update: Uint8Array) => {
        ws.sendMessage({ type: 'sync', data: Array.from(update) });
      };
      ydocRef.current.on('update', updateHandler);
      return () => {
        ydocRef.current.off('update', updateHandler);
        ydocRef.current.destroy();
      };
    }, []);
    
    return ytextRef.current;
  }
  ```
- `Y.applyUpdate(doc, update)`: https://docs.yjs.dev/api/shared-types/y.doc#y-applyupdate
- Refer to: https://docs.yjs.dev/
