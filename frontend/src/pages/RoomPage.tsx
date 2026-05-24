import { useParams } from 'react-router-dom';

/**
 * RoomPage — Collaborative editor for a room.
 * Phase 1 placeholder: displays a loading state.
 * Phase 2 will integrate WebSocket, Yjs, and CodeMirror.
 */
export function RoomPage() {
  const { id } = useParams<{ id: string }>();

  return (
    <div className="flex items-center justify-center min-h-screen bg-gray-950">
      <div className="text-center">
        <h1 className="text-2xl font-bold text-gray-300 mb-4">
          Room: {id}
        </h1>
        <p className="text-gray-500">
          Loading editor...
        </p>
      </div>
    </div>
  );
}
