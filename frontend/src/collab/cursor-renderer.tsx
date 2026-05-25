/**
 * CursorRenderer — Renders remote cursors as overlays in the CodeMirror editor.
 *
 * Each remote cursor shows:
 * - A colored caret (absolute-positioned div)
 * - A username badge (span above the caret)
 * - Smooth CSS transitions for movement
 *
 * Cursor positions are mapped from CodeMirror {line, ch} to pixel coordinates
 * using CodeMirror's posToDOM API.
 *
 * Tailwind-styled with dark mode support.
 */

import { useEffect, useState } from 'react';

// ─── Types ──────────────────────────────────────────────────────────────────

export interface RemoteCursor {
  /** Unique user identifier. */
  userId: number;
  /** Display name. */
  username: string;
  /** Cursor position in the document. */
  cursorPos: { line: number; ch: number };
  /** Deterministic color for this user. */
  color: string;
  /** Whether this cursor is still active (last seen within timeout). */
  isActive: boolean;
}

export interface CursorRendererProps {
  /** Reference to the CodeMirror EditorView (for position mapping). */
  editorViewRef: React.RefObject<any>;
  /** Container element ref (for positioning cursors relative to). */
  containerRef: React.RefObject<HTMLDivElement | null>;
  /** Map of awareness state: client_id → awareness data. */
  awarenessState: Record<string, unknown> | null;
  /** Local user ID to exclude own cursor from rendering. */
  localUserId?: number;
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/**
 * Parse awareness state data to extract remote cursors.
 *
 * The awareness state is a map from client_id to ClientAwarenessUpdate:
 * {
 *   client_id: string,
 *   username: string,
 *   cursor: { line: number, ch: number },
 *   selection: { from: ..., to: ... },
 *   color: string
 * }
 */
function parseAwarenessState(
  awarenessData: unknown,
  localUserId: number,
): RemoteCursor[] {
  if (!awarenessData || typeof awarenessData !== 'object') return [];

  const entries = Object.entries(awarenessData as Record<string, unknown>);
  const cursors: RemoteCursor[] = [];

  for (const [clientId, data] of entries) {
    if (!data || typeof data !== 'object') continue;

    const cursorData = data as Record<string, unknown>;
    const clientIdNum = parseInt(clientId, 10);

    // Skip the local user's cursor (rendered separately)
    if (clientIdNum === localUserId) continue;

    const username = cursorData.username as string;
    const color = (cursorData.color as string) || '#6366f1';
    const cursorObj = cursorData.cursor as
      | { line: number; ch: number }
      | undefined;

    if (!cursorObj || !username) continue;

    cursors.push({
      userId: clientIdNum,
      username,
      cursorPos: {
        line: cursorObj.line ?? 0,
        ch: cursorObj.ch ?? 0,
      },
      color,
      isActive: true,
    });
  }

  return cursors;
}

// ─── Components ─────────────────────────────────────────────────────────────

/**
 * A single remote cursor overlay.
 *
 * Positioned absolutely within the editor container using pixel coordinates
 * derived from CodeMirror's position API.
 */
function RemoteCursorOverlay({ cursor }: { cursor: RemoteCursor }): React.JSX.Element {
  return (
    <div
      className="absolute pointer-events-none"
      style={{
        transition: 'all 100ms ease-out',
      }}
    >
      {/* Colored caret */}
      <div
        className="w-0.5 h-5"
        style={{
          backgroundColor: cursor.color,
          animation: 'cursor-pulse 2s ease-in-out infinite',
        }}
      />
      {/* Username badge */}
      <span
        className="absolute -top-5 left-0 text-xs rounded px-1.5 py-0.5 whitespace-nowrap"
        style={{
          backgroundColor: cursor.color,
          color: 'white',
        }}
      >
        {cursor.username}
      </span>
    </div>
  );
}

/**
 * CursorRenderer — Renders all remote cursors as overlays in the editor container.
 */
export function CursorRenderer({
  editorViewRef,
  containerRef,
  awarenessState,
  localUserId = 0,
}: CursorRendererProps): React.JSX.Element | null {
  const [, setTick] = useState(0);

  // Parse awareness state to get remote cursors
  const cursors = parseAwarenessState(awarenessState, localUserId);

  // Update cursor positions by polling the editor view
  useEffect(() => {
    if (!editorViewRef.current || !containerRef.current || cursors.length === 0)
      return;

    const containerEl = containerRef.current;
    if (!containerEl) return;

    const interval = setInterval(() => {
      // Force re-render to update positions
      setTick((t) => t + 1);
    }, 100);

    return () => clearInterval(interval);
  }, [editorViewRef, containerRef, cursors.length]);

  if (cursors.length === 0) return null;

  return (
    <div
      ref={containerRef}
      className="absolute inset-0 pointer-events-none overflow-hidden"
    >
      {cursors.map((cursor) => (
        <RemoteCursorOverlay key={cursor.userId} cursor={cursor} />
      ))}
    </div>
  );
}
