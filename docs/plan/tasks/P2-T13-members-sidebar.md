# P2-T13 — Members Sidebar

**Status:** TODO

## Goal

Create the members sidebar showing online users and their status.

## Description

Create `src/components/MembersSidebar.tsx` — a collapsible sidebar component that shows all online room members with their cursor indicators and roles.

The sidebar:
1. Lists online members with colored status dots
2. Shows username and role badge (owner/editor)
3. Has a "Leave Room" button
4. Has an "Add Member" form with username input (future feature)

The member list is derived from the room's awareness state — each connected client with a valid awareness state is shown as online.

Components:
- `MemberItem` — individual member row with avatar dot, username, role
- `MembersList` — scrollable list of members
- `LeaveRoomButton` — disconnects WS and navigates home
- `AddMemberForm` — placeholder for future member management

## Acceptance Criteria

- AC1: `MembersSidebar` component exists
- AC2: `MemberItem` shows colored dot, username, role badge
- AC3: Member list updates in real-time from awareness state
- AC4: "Leave Room" button disconnects WebSocket and navigates to `/`
- AC5: "Add Member" form with username input (UI only, no backend yet)
- AC6: Collapsible sidebar (toggle open/close)
- AC7: Tailwind-styled with dark mode support
- AC8: Online member count in sidebar header
- AC9: No lint errors
- AC10: Sidebar integrated into RoomPage

## Technical Hints

- Colored dot: `<span className="inline-block w-2 h-2 rounded-full bg-green-400" />`
- Role badge: `<span className="text-xs px-1 py-0.5 bg-blue-500/20 text-blue-400 rounded">editor</span>`
- Sidebar toggle: `useState(false)` for open/closed state
- Leave room: `wsRef.current?.close(); navigate('/')`
- Layout: `className="w-64 border-l bg-gray-900/50 flex flex-col"`
- Member count header: `Online ({members.length})`
- Refer to: https://tailwindcss.com/docs/hover-focus-and-other-states
