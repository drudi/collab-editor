# P2-T11 — Cursor Sharing: Client

**Status:** TODO

## Goal

Implement client-side remote cursor rendering in the editor.

## Description

Create `src/collab/cursor-renderer.tsx` — a React component that renders remote cursors as overlays within the CodeMirror editor container. Each remote cursor shows a colored caret with the user's username.

The component:
1. Subscribes to Awareness updates from the WebSocket connection
2. Renders a `RemoteCursor` overlay for each active remote cursor
3. Each cursor positioned absolutely based on CodeMirror's `coordsChar` API
4. Cursor style: colored caret + username badge + smooth transitions

`RemoteCursor` props:
- `userId: number` — unique identifier
- `username: string` — display name
- `cursorPos: Position` — `{line, ch}` position
- `color: string` — deterministic color

Position mapping from CodeMirror `{line, ch}` to screen coordinates:
- Use `view.coordsAtPos(view.state.doc.resolve(line + 1).before(ch))` to get pixel position
- Apply `top` and `left` CSS transforms based on pixel position

## Acceptance Criteria

- AC1: `CursorRenderer` component exists as React component
- AC2: Renders remote cursors as absolute-positioned overlays
- AC3: Each cursor shows colored caret (div with border) + username badge (span)
- AC4: Cursor position mapped from CodeMirror `{line, ch}` to pixel coordinates
- AC5: Smooth CSS transitions for cursor movement (100ms)
- AC6: Cursors hidden when user disconnects
- AC7: Awareness subscription via WebSocket updates cursors in real-time
- AC8: Deterministic cursor color from userId
- AC9: Tailwind-styled with dark mode support
- AC10: No lint errors

## Technical Hints

- Position mapping:
  ```typescript
  const pos = view.posToDOM(line + 1, ch); // returns {left, top}
  ```
- Cursor overlay:
  ```tsx
  <div style={{ left: `${x}px`, top: `${y}px` }} className="absolute pointer-events-none">
    <div className="w-0.5 h-5 bg-{color} animate-pulse" />
    <span className="absolute -top-5 left-0 text-xs bg-gray-800 px-1 rounded">{username}</span>
  </div>
  ```
- Use `useEffect` with awareness subscription for updates
- CSS transition: `transition: all 100ms ease-out`
- Refer to: https://codemirror.net/docs/ref/#view.EditorView.posToDOM
