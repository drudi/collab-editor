# P1-T05 — Migration 001: Users Table

**Status:** TODO

## Goal

Create the database migration for the users table.

## Description

Write `migrations/001_create_users.sql` that creates the users table. This table stores registered users and their password hashes.

The table schema:
```sql
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL CHECK(length(username) >= 3),
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

Add an index on `username` for fast lookups during login. The `CHECK` constraint enforces minimum username length at the database level.

## Acceptance Criteria

- AC1: `001_create_users.sql` exists in `migrations/` directory
- AC2: Table `users` is created with all four columns
- AC3: `username` has `UNIQUE` constraint
- AC4: `password_hash` has `NOT NULL` constraint
- AC5: Index on `username` for lookup performance
- AC6: `CREATE TABLE IF NOT EXISTS` used for idempotent migration
- AC7: SQL validates with `sqlite3` syntax

## Technical Hints

- Use `IF NOT EXISTS` for safe repeated migration runs
- SQLite `AUTOINCREMENT` ensures unique integer IDs
- `DEFAULT CURRENT_TIMESTAMP` is standard SQLite for datetime defaults
- For UNIQUE constraints, also add an explicit `CREATE UNIQUE INDEX` for clarity
- Refer to: https://www.sqlite.org/lang_createtable.html
