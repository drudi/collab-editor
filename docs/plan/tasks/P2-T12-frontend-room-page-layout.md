# P2-T12 — Frontend Room Page Layout

**Status:** TODO

## Goal

Create the room page layout that integrates all collaboration components.

## Description

Create `src/pages/RoomPage.tsx` — the main page component for the collaborative editor. This page integrates the WebSocket hook, Yjs hook, CodeMirror editor, and cursor renderer into a full-width editing experience.

The page:
1. Extracts `roomId` from URL params via `useParams()`
2. Initializes `useWebSocket(roomId, onSyncUpdate)`
3. Initializes `useYjs(roomId, ws)` for CRDT document
4. Renders `CodeMirrorEditor` with the Yjs text binding
5. Renders `CursorRenderer` overlay for remote cursors
6. Shows a loading state while connection is establishing

Layout structure:
```
RoomPage
├── CodeMirrorEditor (full width, full height)
│   ├── Editor content (Yjs-bound)
│   └── CursorRenderer (absolute overlay)
└── (Sidebar toggle — Phase 2.13)
```

## Acceptance Criteria

- AC1: `RoomPage` component in `src/pages/RoomPage.tsx`
- AC2: Room ID extracted from URL params via `useParams()`
- AC3: `useWebSocket` hook initialized with room ID
- AC4: `useYjs` hook initialized with room ID and WebSocket
- AC5: `CodeMirrorEditor` component renders with Yjs text binding
- AC6: `CursorRenderer` component rendered as overlay on editor
- AC7: Loading state while connection is establishing
- AC8: Error state if WebSocket connection fails
- AC9: Full-width, full-height layout
- AC10: No lint errors

## Technical Hints

- Route params:
  ```typescript
  const { roomId } = useParams<{ roomId: string }>();
  ```
- Layout: `className="w-screen h-screen flex flex-col"`
- Editor container: `className="flex-1 relative overflow-hidden"`
- Cursor overlay: `className="absolute inset-0 pointer-events-none"`
- Loading: "Connecting..." spinner
- Error: "Connection lost. Reconnecting..." banner
- Refer to: https://react.dev/reference/react/useEffect
