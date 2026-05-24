# P1-T21 — Frontend: Home Page + Room Browser

**Status:** Done

## Goal

Implement the home page with room browser and inline forms.

## Description

Create `src/pages/HomePage.tsx` with the room browser interface. The home page shows the user's rooms and provides forms for creating new rooms and joining existing ones.

Components to create:
- `RoomCard` — displays room name, code, member count, language, with "Open" button
- `CreateRoomForm` — inline form with name, language select
- `JoinRoomForm` — inline form with room code input

The home page fetches user's rooms from `GET /api/rooms` (or a new endpoint that lists all rooms) on mount. `RoomCard` navigates to `/room/:id` on click.

## Acceptance Criteria

- AC1: `HomePage` component exists in `src/pages/HomePage.tsx`
- AC2: `RoomCard` component displays room metadata
- AC3: `CreateRoomForm` inline form with name + language fields
- AC4: `JoinRoomForm` inline form with room code field
- AC5: Fetches room list on mount
- AC6: `CreateRoomForm` calls `POST /api/rooms` on submit
- AC7: `JoinRoomForm` navigates to `/room/:code` on submit
- AC8: Empty state shown when no rooms exist
- AC9: Error handling with toast or inline message
- AC10: Tailwind-styled with dark mode support

## Technical Hints

- Room list fetch pattern:
  ```typescript
  const [rooms, setRooms] = useState<Room[]>([]);
  useEffect(() => {
    fetch('/api/rooms').then(r => r.json()).then(setRooms);
  }, []);
  ```
- Language select options: PlainText, JavaScript, Python, Rust
- Empty state: "No rooms yet. Create one to get started!"
- `RoomCard`: `className="p-4 border rounded-lg hover:bg-gray-800/50"`
- Refer to: https://react.dev/reference/react/useEffect
