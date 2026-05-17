# P1-T09 — DB Connection Pool + Queries

**Status:** TODO

## Goal

Create the database connection function and basic query helpers.

## Description

Implement `src/db/mod.rs` with a `connect(url)` function that creates and returns a `sqlx::SqlitePool`. The function runs all pending migrations via `sqlx::migrate!().run()` before returning.

Also implement `src/db/queries.rs` with raw sqlx query helpers:

- `create_user(pool, username, password_hash) -> User` — INSERT into users
- `get_user_by_username(pool, username) -> Option<User>` — SELECT by username
- `insert_session(pool, user_id, token, expires_at) -> Session` — INSERT into sessions
- `get_session_by_token(pool, token) -> Option<Session>` — SELECT by token

Use parameterized queries with `sqlx::query_as!()` for type safety. Each function takes `&SqlitePool` as first argument.

## Acceptance Criteria

- AC1: `connect(url)` function creates pool and runs migrations
- AC2: Pool uses `PoolOptions::new().max_connections(10)`
- AC3: Migrations run from `migrations/` directory
- AC4: `create_user` function with `query_as!` macro
- AC5: `get_user_by_username` with parameterized query
- AC6: `insert_session` function returns the created session
- AC7: `get_session_by_token` returns `Option<Session>`
- AC8: `cargo check` passes
- AC9: Migrations directory path is configurable or uses `env!("CARGO_MANIFEST_DIR")`

## Technical Hints

- Use `sqlx::migrate!("./migrations")` for compile-time migration scanning
- Use `env!("CARGO_MANIFEST_DIR")` to find migrations directory regardless of cwd
- `query_as!` macro maps SQL columns to struct fields by name — field names must match column names
- For datetime parsing, use `chrono::NaiveDateTime` with `chrono::sqlite::ChronoExtension`
- Pattern: each query function takes `pool: &SqlitePool` as argument for flexibility
- Refer to: https://docs.rs/sqlx/latest/sqlx/
