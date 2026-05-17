# P1-T07 — Migration 003: Rooms Table

**Status:** TODO

## Goal

Create the database migration for the rooms and room_members tables.

## Description

Write `migrations/003_create_rooms.sql` that creates both the rooms and room_members tables. These tables manage collaborative editing rooms and their member relationships.

The rooms table stores room metadata. The room_members table is a junction table between users and rooms, supporting multiple members per room with different roles.

```sql
CREATE TABLE IF NOT EXISTS rooms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    language TEXT NOT NULL DEFAULT 'plaintext',
    owner_id INTEGER NOT NULL REFERENCES users(id),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS room_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id INTEGER NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    member_type TEXT NOT NULL DEFAULT 'editor',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(room_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_rooms_code ON rooms(code);
CREATE INDEX IF NOT EXISTS idx_room_members_room ON room_members(room_id);
CREATE INDEX IF NOT EXISTS idx_room_members_user ON room_members(user_id);
```

## Acceptance Criteria

- AC1: `003_create_rooms.sql` exists in `migrations/` directory
- AC2: `rooms` table created with all seven columns
- AC3: `room_members` table created with all five columns
- AC4: `code` has `UNIQUE` constraint on rooms
- AC5: `member_type` column with DEFAULT 'editor'
- AC6: `UNIQUE(room_id, user_id)` prevents duplicate memberships
- AC7: Foreign keys on both junction table columns
- AC8: All three indexes created
- AC9: SQL validates with `sqlite3` syntax

## Technical Hints

- Use `DEFAULT ''` for empty description to avoid NULL issues
- The junction table pattern supports future features like room invitations
- `member_type` values: 'owner', 'editor', 'viewer'
- Refer to: https://www.sqlite.org/indexes.html
