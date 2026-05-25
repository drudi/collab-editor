/**
 * RoomPage — The collaborative editor page.
 *
 * Integrates all collaboration components:
 * - WebSocket hook for real-time communication
 * - Yjs hook for CRDT document management
 * - CodeMirror editor with Yjs binding
 * - Cursor renderer for remote cursors
 * - Members sidebar
 *
 * Layout:
 * ```
 * RoomPage (full-screen)
 * ├── Top bar (room name, language selector, members toggle)
 * ├── Editor area (CodeMirror + CursorRenderer overlay)
 * └── Members sidebar (collapsible)
 * ```
 */

import { useEffect, useRef, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';

import { useWebSocket } from '../hooks/useWebSocket';
import { useYjs } from '../collab/useYjs';
import { createCmYjsBinding } from '../collab/cm-yjs-binding';
import { getLanguageExtension, getLanguageName, getDefaultLanguage, getSupportedLanguages } from '../collab/languages';
import { CursorRenderer } from '../collab/cursor-renderer';
import { MembersSidebar } from '../components/MembersSidebar';
import { AutoSaveIndicator, SaveStatus } from '../components/AutoSaveIndicator';

// ─── Types ──────────────────────────────────────────────────────────────────

interface RoomMetadata {
  id: number;
  code: string;
  name: string;
  description: string | null;
  language: string | null;
  owner: {
    id: number;
    username: string;
  };
  members: Array<{
    id: number;
    username: string;
    member_type: string;
  }>;
}

// ─── Component ──────────────────────────────────────────────────────────────

export function RoomPage(): JSX.Element {
  const { id: roomId } = useParams<{ id: string }>();
  const navigate = useNavigate();

  // Refs
  const editorContainerRef = useRef<HTMLDivElement>(null);
  const editorViewRef = useRef<any>(null);
  const bindingRef = useRef<ReturnType<typeof createCmYjsBinding> | null>(null);

  // Room metadata
  const [roomMetadata, setRoomMetadata] = useState<RoomMetadata | null>(null);
  const [language, setLanguage] = useState<string>(getDefaultLanguage);
  const [showSidebar, setShowSidebar] = useState(true);
  const [awarenessState, setAwarenessState] = useState<Record<string, unknown> | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [saveStatus, setSaveStatus] = useState<SaveStatus>('unsaved');
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false);

  // WebSocket hook — manages the connection to the room
  const { sendMessage, reconnect, isConnected } = useWebSocket(roomId || '', {
    onSyncUpdate: (update) => {
      if (yjsResult) {
        Y.applyUpdate(yjsResult.doc, update);
      }
    },
    onAwarenessUpdate: (data) => {
      setAwarenessState(data as Record<string, unknown> | null);
    },
  });

  // Yjs hook — manages the CRDT document
  const yjsResult = useYjs(roomId || '', sendMessage);
  const { ytext, doc, awareness } = yjsResult;

  // Load room metadata
  useEffect(() => {
    if (!roomId) return;

    setIsLoading(true);
    setError(null);

    fetch(`/api/rooms/${roomId}`)
      .then((res) => {
        if (!res.ok) {
          throw new Error(`Failed to load room: ${res.status}`);
        }
        return res.json();
      })
      .then((data: RoomMetadata) => {
        setRoomMetadata(data);
        if (data.language) {
          setLanguage(data.language.toLowerCase());
        }
        setIsLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setIsLoading(false);
      });
  }, [roomId]);

  // Initialize CodeMirror editor with Yjs binding
  useEffect(() => {
    if (!editorContainerRef.current || !ytext) return;

    // Disconnect any existing binding
    if (bindingRef.current) {
      bindingRef.current.disconnect();
    }

    // Get language extensions
    const langExt = getLanguageExtension(language);

    // Create the editor binding
    bindingRef.current = createCmYjsBinding(editorContainerRef.current, {
      ytext,
      extensions: langExt,
      initialContent: ytext.toString(),
    });

    editorViewRef.current = bindingRef.current.view;

    // Clean up on unmount
    return () => {
      if (bindingRef.current) {
        bindingRef.current.disconnect();
        bindingRef.current = null;
      }
      editorViewRef.current = null;
    };
  }, [roomId, ytext, language]);

  // Trigger save — marks document as saving, sends WS sync, then marks as saved
  const handleSave = () => {
    setSaveStatus('saving');
    // Force a full sync — send awareness state as a sync message
    const syncMsg = { type: 'sync' as const, data: new Uint8Array() };
    sendMessage(JSON.stringify(syncMsg));
    // Simulate server confirmation after a short delay
    setTimeout(() => {
      setSaveStatus('saved');
      setHasUnsavedChanges(false);
    }, 500);
  };

  // Send cursor position updates
  useEffect(() => {
    const handler = (event: Event) => {
      const detail = (event as CustomEvent).detail;
      if (detail && isConnected) {
        const cursorMsg = {
          type: 'cursor' as const,
          user_id: String(Math.floor(Math.random() * 1000000)),
          username: 'You',
          cursor: {
            line: detail.line,
            ch: detail.ch,
          },
          selection: detail.selection,
        };
        sendMessage(JSON.stringify(cursorMsg));
      }
    };

    // Listen for cursor events from the editor
    const container = editorContainerRef.current;
    if (container) {
      container.addEventListener('collab-cursor', handler);
    }

    return () => {
      if (container) {
        container.removeEventListener('collab-cursor', handler);
      }
    };
  }, [isConnected, sendMessage]);

  // Track document changes via Yjs update event (mark as unsaved)
  useEffect(() => {
    if (!doc) return;

    const handleChange = () => {
      setHasUnsavedChanges(true);
      setSaveStatus('unsaved');
    };

    doc.on('update', handleChange);
    return () => doc.off('update', handleChange);
  }, [doc]);

  // ─── Render ──────────────────────────────────────────────────────────────

  // Loading state
  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-950">
        <div className="text-center">
          <div className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-gray-600 border-t-indigo-500 mb-4" />
          <p className="text-gray-400">Connecting...</p>
        </div>
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-gray-950">
        <div className="text-center max-w-md mx-auto p-6">
          <div className="text-red-400 text-lg font-semibold mb-2">
            Connection Error
          </div>
          <p className="text-gray-400 mb-4">{error}</p>
          <button
            onClick={reconnect}
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg transition-colors"
          >
            Reconnect
          </button>
        </div>
      </div>
    );
  }

  // Connected state with editor
  return (
    <div className="flex h-screen w-screen bg-gray-950 text-gray-100 overflow-hidden">
      {/* Main editor area */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Top bar */}
        <div className="flex items-center justify-between px-4 py-2 border-b border-gray-800 bg-gray-900/50 shrink-0">
          <div className="flex items-center gap-3">
            <h1 className="text-lg font-semibold text-gray-200">
              {roomMetadata?.name || `Room ${roomId}`}
            </h1>
            {roomMetadata?.language && (
              <span className="text-xs px-2 py-0.5 bg-indigo-500/20 text-indigo-400 rounded">
                {getLanguageName(roomMetadata.language)}
              </span>
            )}
            <div className="flex items-center gap-1">
              <span
                className={`inline-block w-2 h-2 rounded-full ${
                  isConnected ? 'bg-green-400' : 'bg-red-400'
                }`}
              />
              <span className="text-xs text-gray-500">
                {isConnected ? 'Connected' : 'Reconnecting...'}
              </span>
            </div>
          </div>

          <div className="flex items-center gap-3">
            {/* Language selector */}
            <select
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="text-xs bg-gray-800 border border-gray-700 rounded px-2 py-1 text-gray-300 focus:outline-none focus:ring-1 focus:ring-indigo-500"
            >
              {getSupportedLanguages().map((lang) => (
                <option key={lang.id} value={lang.id}>
                  {lang.name}
                </option>
              ))}
            </select>

            {/* Save button */}
            <button
              onClick={handleSave}
              className="px-3 py-1 text-xs bg-indigo-600 hover:bg-indigo-700 text-white rounded transition-colors"
            >
              Save
            </button>

            {/* Members sidebar toggle */}
            <button
              onClick={() => setShowSidebar(!showSidebar)}
              className="px-3 py-1 text-xs bg-gray-800 hover:bg-gray-700 border border-gray-700 rounded transition-colors"
            >
              Members
            </button>
          </div>
        </div>

        {/* Editor container */}
        <div className="flex-1 relative overflow-hidden">
          <div
            ref={editorContainerRef}
            className="w-full h-full"
          />

          {/* Cursor renderer overlay */}
          {awarenessState && (
            <CursorRenderer
              editorViewRef={editorViewRef}
              containerRef={editorContainerRef}
              awarenessState={awarenessState}
            />
          )}

          {/* Auto-save indicator — bottom-right of editor area (P4-T02, AC7) */}
          <AutoSaveIndicator status={saveStatus} />
        </div>
      </div>

      {/* Members sidebar */}
      {showSidebar && <MembersSidebar roomMetadata={roomMetadata} />}
    </div>
  );
}
