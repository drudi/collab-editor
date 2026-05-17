# P1-T15 — Rooms: Get Metadata

**Status:** TODO

## Goal

Implement the room metadata retrieval endpoint.

## Description

Create `src/rooms/get.rs` with a `GET /api/rooms/:id` handler. This endpoint returns full metadata about a room including owner info, member list, and latest document snapshot.

The handler flow:
1. Extract room ID from path parameter
2. Query rooms JOIN users ON owner to get owner info
3. Query room_members for member list with user details
4. Query latest document snapshot if exists
5. Return combined metadata

## Acceptance Criteria

- AC1: `GET /api/rooms/:id` handler exists in `src/rooms/get.rs`
- AC2: Room ID extracted from path parameter
- AC3: JOIN query retrieves room + owner info
- AC4: Member list query returns all members with usernames and roles
- AC5: Latest document snapshot query (most recent version)
- AC6: Returns `200 OK` with room metadata JSON
- AC7: Returns `404 Not Found` if room doesn't exist
- AC8: Response includes: id, code, name, description, language, owner, members, version
- AC9: Handler wired into `main.rs` router

## Technical Hints

- Use `axum::extract::Path<(u32,)>` for path parameter extraction
- SQL: `SELECT r.*, u.username as owner_name FROM rooms r JOIN users u ON r.owner_id = u.id WHERE r.id = ?`
- Member query: `SELECT rm.*, u.username FROM room_members rm JOIN users u ON rm.user_id = u.id WHERE rm.room_id = ?`
- Document query: `SELECT * FROM documents WHERE room_id = ? ORDER BY version DESC LIMIT 1`
- Consider using `sqlx::query_as!` for typed queries
- Refer to: https://docs.rs/axum/latest/axum/extract/fn.Path.html
