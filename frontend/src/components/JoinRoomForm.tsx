import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

interface JoinRoomFormProps {
  error: string | null;
}

export function JoinRoomForm({ error }: JoinRoomFormProps) {
  const navigate = useNavigate();
  const [code, setCode] = useState('');
  const [isJoining, setIsJoining] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!code.trim()) return;
    setIsJoining(true);
    try {
      navigate(`/room/${code.trim()}`);
    } finally {
      setIsJoining(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <input
        type="text"
        value={code}
        onChange={(e) => setCode(e.target.value)}
        placeholder="Enter room code"
        className="w-full px-3 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm uppercase font-mono"
        maxLength={6}
        required
      />
      {error && (
        <p className="text-red-400 text-sm">{error}</p>
      )}
      <button
        type="submit"
        disabled={isJoining || !code.trim()}
        className="w-full py-2 px-4 rounded-lg bg-purple-600 hover:bg-purple-700 disabled:bg-gray-700 disabled:text-gray-500 text-white text-sm font-medium transition-colors"
      >
        {isJoining ? 'Joining...' : 'Join Room'}
      </button>
    </form>
  );
}
