# P1-T06 — Migration 002: Sessions Table

**Status:** TODO

## Goal

Create the database migration for the sessions table.

## Description

Write `migrations/002_create_sessions.sql` that creates the sessions table for managing user login sessions.

The table schema:
```sql
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT UNIQUE NOT NULL,
    expires_at DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

Add an index on `token` for fast session lookups. The foreign key on `user_id` ensures orphaned sessions are cleaned up when users are deleted.

## Acceptance Criteria

- AC1: `002_create_sessions.sql` exists in `migrations/` directory
- AC2: Table `sessions` is created with all five columns
- AC3: `token` has `UNIQUE` constraint
- AC4: `user_id` references `users(id)` with `ON DELETE CASCADE`
- AC5: Index on `token` for lookup performance
- AC6: `expires_at` has `NOT NULL` constraint
- AC7: Foreign key constraint is defined
- AC8: SQL validates with `sqlite3` syntax

## Technical Hints

- Foreign key enforcement requires `PRAGMA foreign_keys = ON;` in connection setup
- Use `CREATE UNIQUE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token);`
- The `ON DELETE CASCADE` means sessions are automatically cleaned up when a user is deleted
- Token will be a random UUID or cryptographically secure random string
- Refer to: https://www.sqlite.org/foreignkeys.html
