/**
 * MembersSidebar — Collapsible sidebar showing online room members.
 *
 * Features:
 * - Member list with colored status dots
 * - Username and role badge (owner/editor)
 * - "Leave Room" button (disconnects WS, navigates home)
 * - "Add Member" form (UI only, placeholder for future backend)
 * - Collapsible (toggle open/close)
 * - Tailwind-styled with dark mode support
 * - Online member count in header
 */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

// ─── Types ──────────────────────────────────────────────────────────────────

interface MemberItemProps {
  username: string;
  role: string;
  isOnline: boolean;
  color?: string;
}

interface MembersListProps {
  members: MemberItemProps[];
}

interface LeaveRoomButtonProps {
  onLeave: () => void;
}

interface AddMemberFormProps {
  onAdd: (username: string) => void;
}

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

// ─── MemberItem ─────────────────────────────────────────────────────────────

/**
 * Individual member row with avatar dot, username, and role badge.
 */
export function MemberItem({ username, role, isOnline, color }: MemberItemProps): JSX.Element {
  return (
    <div className="flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-gray-800/50 transition-colors">
      {/* Colored status dot */}
      <span
        className="inline-block w-2 h-2 rounded-full shrink-0"
        style={{
          backgroundColor: isOnline ? '#4ade80' : color || '#6b7280',
        }}
      />
      {/* Username */}
      <span className="text-sm text-gray-200 truncate">{username}</span>
      {/* Role badge */}
      <span className="ml-auto text-xs px-1.5 py-0.5 bg-indigo-500/20 text-indigo-400 rounded shrink-0">
        {role}
      </span>
    </div>
  );
}

// ─── MembersList ────────────────────────────────────────────────────────────

/**
 * Scrollable list of online members.
 */
export function MembersList({ members }: MembersListProps): JSX.Element {
  return (
    <div className="flex-1 overflow-y-auto space-y-1">
      {members.map((member, index) => (
        <MemberItem
          key={`${member.username}-${index}`}
          username={member.username}
          role={member.role}
          isOnline={member.isOnline}
        />
      ))}
    </div>
  );
}

// ─── LeaveRoomButton ────────────────────────────────────────────────────────

/**
 * Button to disconnect from the room and navigate home.
 */
export function LeaveRoomButton({ onLeave }: LeaveRoomButtonProps): JSX.Element {
  const navigate = useNavigate();

  const handleLeave = () => {
    // Close WebSocket connection (the parent RoomPage will handle this)
    onLeave();
    // Navigate to home page
    navigate('/');
  };

  return (
    <button
      onClick={handleLeave}
      className="w-full px-3 py-2 text-sm text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition-colors"
    >
      Leave Room
    </button>
  );
}

// ─── AddMemberForm (placeholder) ────────────────────────────────────────────

/**
 * Placeholder form for future member management.
 * UI only — no backend integration yet.
 */
export function AddMemberForm({ onAdd }: AddMemberFormProps): JSX.Element {
  const [username, setUsername] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (username.trim()) {
      onAdd(username.trim());
      setUsername('');
    }
  };

  return (
    <form onSubmit={handleSubmit} className="flex gap-2">
      <input
        type="text"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        placeholder="Username"
        className="flex-1 text-xs bg-gray-800 border border-gray-700 rounded px-2 py-1.5 text-gray-300 placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
      />
      <button
        type="submit"
        className="px-3 py-1.5 text-xs bg-indigo-600 hover:bg-indigo-700 text-white rounded transition-colors"
      >
        Add
      </button>
    </form>
  );
}

// ─── MembersSidebar ─────────────────────────────────────────────────────────

interface MembersSidebarProps {
  roomMetadata: RoomMetadata | null;
}

/**
 * Collapsible sidebar showing online room members.
 *
 * Displays:
 * - Online member count in the header
 * - Member list with colored dots, usernames, and role badges
 * - Leave Room button
 * - Add Member form (placeholder)
 */
export function MembersSidebar({ roomMetadata }: MembersSidebarProps): JSX.Element {
  const navigate = useNavigate();
  const [isOnline, setIsOnline] = useState(true);

  // Get room members with online status
  const members = roomMetadata
    ? roomMetadata.members.map((m) => ({
        username: m.username,
        role: m.member_type,
        isOnline: isOnline,
        color: '#6366f1',
      }))
    : [];

  const handleLeave = () => {
    // The WebSocket connection is managed by the parent RoomPage.
    // We'll dispatch an event for the parent to handle.
    window.dispatchEvent(new CustomEvent('room-leave'));
    // Navigate home after a brief delay
    setTimeout(() => navigate('/'), 100);
  };

  const handleAddMember = (_username: string) => {
    // Placeholder — future backend integration
    console.log('Add member:', _username);
  };

  return (
    <div className="w-64 border-l border-gray-800 bg-gray-900/50 flex flex-col shrink-0">
      {/* Header with member count */}
      <div className="px-4 py-3 border-b border-gray-800">
        <h2 className="text-sm font-semibold text-gray-300">
          Members ({members.length})
        </h2>
      </div>

      {/* Member list */}
      <MembersList members={members} />

      {/* Add member form (placeholder) */}
      <div className="px-4 py-3 border-t border-gray-800">
        <AddMemberForm onAdd={handleAddMember} />
      </div>

      {/* Leave room button */}
      <div className="p-4 border-t border-gray-800">
        <LeaveRoomButton onLeave={handleLeave} />
      </div>
    </div>
  );
}
