# P1-T04 — Shared Models

**Status:** Done

## Goal

Define all domain structs used across the application.

## Description

Create `src/models/mod.rs` with all shared domain types. These structs are the data contracts between the database, API layer, and frontend. Each must be `serde::Serialize + serde::Deserialize` for API responses.

Structs to define:

- **User**: id, username, password_hash (excluded from API responses), created_at
- **Session**: id, user_id, token, expires_at, created_at
- **Room**: id, code, name, description, language, owner_id, created_at
- **RoomMember**: id, room_id, user_id, user (nested User), member_type
- **Document**: id, room_id, content_snapshot (Vec<u8>), version, saved_at
- **RoomMemberType**: enum with Owner, Editor, Viewer variants
- **Language**: enum with Javascript, Python, Rust, Typescript, PlainText variants

Add a `UserResponse` struct (without password_hash) for API serialization. Add `RoomMetadataResponse` for the room metadata endpoint.

## Acceptance Criteria

- AC1: All six structs (User, Session, Room, RoomMember, Document, RoomMemberType) defined
- AC2: All structs derive `Serialize`, `Deserialize`, and `Clone`
- AC3: `User` has `#[serde(skip_serializing)]` on `password_hash`
- AC4: `Language` enum with all language variants and `FromStr` impl
- AC5: `UserResponse` struct excludes `password_hash`
- AC6: `RoomMetadataResponse` struct for API endpoint output
- AC7: All types re-exported from `src/models/mod.rs`
- AC8: `cargo check` passes

## Technical Hints

- Use `chrono::NaiveDateTime` for datetime fields
- Use `#[serde(default)]` on optional fields
- For `Language`, implement `strum::Display` for easy formatting
- Password hash should use `String` type but never be serialized to API responses
- Consider `#[derive(sqlx::FromRow)]` if using sqlx derive macros
- Pattern: domain structs in models, response types in their respective handler modules
