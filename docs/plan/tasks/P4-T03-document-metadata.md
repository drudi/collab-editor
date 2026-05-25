# P4-T03 — Document Metadata

**Status:** Done

## Goal

Implement room metadata retrieval and editing endpoints.

## Description

Extend the room endpoints with metadata management: a detailed metadata endpoint and an edit endpoint for room properties.

New endpoints:
- `GET /api/rooms/:id/metadata` — detailed room metadata
- `PATCH /api/rooms/:id` — edit room properties (name, description, language)

`GET /api/rooms/:id/metadata` returns:
```json
{
  "id": 1,
  "code": "abc123",
  "name": "My Room",
  "description": "Room description",
  "language": "javascript",
  "owner": { "id": 1, "username": "alice" },
  "members": [
    { "id": 1, "username": "alice", "member_type": "owner" },
    { "id": 2, "username": "bob", "member_type": "editor" }
  ],
  "version": 42,
  "lastSavedAt": "2024-01-15T10:30:00Z"
}
```

`PATCH /api/rooms/:id` accepts:
```json
{
  "name": "New Name",
  "description": "New description",
  "language": "python"
}
```

Only the room owner can edit. Validates language against the `Language` enum.

## Acceptance Criteria

- AC1: `GET /api/rooms/:id/metadata` endpoint exists
- AC2: Returns full metadata with owner, members, version, lastSavedAt
- AC3: `PATCH /api/rooms/:id` endpoint exists
- AC4: Only owner can edit room properties (authorization check)
- AC5: Language validation against `Language` enum
- AC6: Partial updates supported (only provided fields updated)
- AC7: Returns `403 Forbidden` if non-owner tries to edit
- AC8: Returns `400 Bad Request` for invalid language
- AC9: Returns `404 Not Found` if room doesn't exist
- AC10: Handler wired into router

## Technical Hints

- Owner check: `if member.user_id != session.user.id || member.member_type != 'owner'`
- Partial update: use `serde(flatten)` or build SQL UPDATE dynamically
- SQL for edit: `UPDATE rooms SET name = COALESCE(?, name), description = COALESCE(?, description), language = COALESCE(?, language) WHERE id = ?`
- Metadata query: JOIN rooms + users + room_members + documents
- Refer to: https://docs.rs/axum/latest/axum/extract/fn.Patch.html
