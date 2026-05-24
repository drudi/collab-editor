import { Link } from 'react-router-dom';

interface RoomInfo {
  id: number;
  code: string;
  name: string;
  description: string | null;
  language: string | null;
  owner_id: number;
  owner_username: string;
  created_at: string;
}

interface RoomCardProps {
  room: RoomInfo;
}

const languageColors: Record<string, string> = {
  javascript: 'bg-yellow-500/20 text-yellow-400',
  typescript: 'bg-blue-500/20 text-blue-400',
  python: 'bg-green-500/20 text-green-400',
  rust: 'bg-orange-500/20 text-orange-400',
  plain_text: 'bg-gray-500/20 text-gray-400',
};

const languageLabels: Record<string, string> = {
  javascript: 'JS',
  typescript: 'TS',
  python: 'PY',
  rust: 'RS',
  plain_text: 'TXT',
};

export function RoomCard({ room }: RoomCardProps) {
  const lang = room.language || 'plain_text';
  const colorClass = languageColors[lang] || languageColors.plain_text;
  const label = languageLabels[lang] || 'TXT';

  return (
    <div className="p-4 rounded-lg bg-gray-900 border border-gray-800 hover:border-gray-600 transition-colors">
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <Link
              to={`/room/${room.code}`}
              className="text-lg font-semibold text-white hover:text-blue-400 transition-colors truncate"
            >
              {room.name}
            </Link>
            <span className={`px-1.5 py-0.5 rounded text-xs font-mono font-medium ${colorClass}`}>
              {label}
            </span>
          </div>
          <p className="text-sm text-gray-400 truncate">
            {room.description || 'No description'}
          </p>
          <div className="flex items-center gap-3 mt-2 text-xs text-gray-500">
            <span>Code: <code className="text-gray-400 font-mono">{room.code}</code></span>
            <span>by {room.owner_username}</span>
            <span>Created {new Date(room.created_at).toLocaleDateString()}</span>
          </div>
        </div>
        <Link
          to={`/room/${room.code}`}
          className="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium whitespace-nowrap transition-colors"
        >
          Open
        </Link>
      </div>
    </div>
  );
}
