import { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../contexts/AuthContext';
import { RoomCard } from '../components/RoomCard';
import { CreateRoomForm } from '../components/CreateRoomForm';
import { JoinRoomForm } from '../components/JoinRoomForm';

export interface RoomInfo {
  id: number;
  code: string;
  name: string;
  description: string | null;
  language: string | null;
  owner_id: number;
  owner_username: string;
  created_at: string;
}

export function HomePage() {
  const { logout, user } = useAuth();
  const [rooms, setRooms] = useState<RoomInfo[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchRooms = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/rooms', {
        credentials: 'include',
      });
      if (!res.ok) {
        if (res.status === 401) {
          window.location.href = '/login';
          return;
        }
        throw new Error('Failed to fetch rooms');
      }
      const data = await res.json();
      setRooms(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load rooms');
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchRooms();
  }, [fetchRooms]);

  const handleCreate = async (name: string, description: string, language: string) => {
    setError(null);
    try {
      const res = await fetch('/api/rooms', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ name, description, language }),
      });
      if (!res.ok) {
        const data = await res.json().catch(() => ({}));
        throw new Error(data.message || 'Failed to create room');
      }
      const data = await res.json();
      setRooms((prev) => [
        ...prev,
        {
          id: data.room_id,
          code: data.room_code,
          name: data.name,
          description: null,
          language: data.language,
          owner_id: 0,
          owner_username: user?.username || '',
          created_at: new Date().toISOString(),
        },
      ]);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create room');
    }
  };

  if (isLoading && rooms.length === 0) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-950">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-950">
      {/* Header */}
      <header className="border-b border-gray-800">
        <div className="max-w-4xl mx-auto px-4 py-4 flex items-center justify-between">
          <div>
            <h1 className="text-xl font-bold text-white">CollabEditor</h1>
            {user && (
              <p className="text-sm text-gray-400">Welcome, {user.username}</p>
            )}
          </div>
          <button
            onClick={() => logout().then(() => {
              window.location.href = '/login';
            })}
            className="px-4 py-2 rounded-lg bg-gray-800 hover:bg-gray-700 text-gray-300 text-sm transition-colors"
          >
            Logout
          </button>
        </div>
      </header>

      {/* Main content */}
      <main className="max-w-4xl mx-auto px-4 py-8 space-y-8">
        {/* Create / Join forms */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h2 className="text-lg font-semibold text-white mb-3">Create Room</h2>
            <CreateRoomForm onCreate={handleCreate} error={error} />
          </div>
          <div>
            <h2 className="text-lg font-semibold text-white mb-3">Join Room</h2>
            <JoinRoomForm error={error} />
          </div>
        </div>

        {/* Room list */}
        <div>
          <h2 className="text-lg font-semibold text-white mb-3">
            Your Rooms ({rooms.length})
          </h2>
          {rooms.length === 0 ? (
            <div className="text-center py-12 rounded-lg bg-gray-900/50 border border-gray-800">
              <p className="text-gray-400">No rooms yet.</p>
              <p className="text-gray-500 text-sm">Create one above to get started!</p>
            </div>
          ) : (
            <div className="space-y-3">
              {rooms.map((room) => (
                <RoomCard key={room.id} room={room} />
              ))}
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
