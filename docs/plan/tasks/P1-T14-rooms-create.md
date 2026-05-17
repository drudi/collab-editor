# P1-T14 — Rooms: Create

**Status:** TODO

## Goal

Implement the room creation endpoint.

## Description

Create `src/rooms/create.rs` with a `POST /api/rooms` handler. This endpoint allows authenticated users to create a new collaborative editing room.

The handler flow:
1. Validate that user has a valid session (uses session middleware)
2. Extract room name, description, language from JSON body
3. Generate a unique 6-character alphanumeric room code
4. INSERT into rooms table
5. INSERT into room_members table (user as owner)
6. Return 201 with room data

Input validation:
- Room name: 1-64 characters
- Language: valid enum value, defaults to 'plaintext'

## Acceptance Criteria

- AC1: `POST /api/rooms` handler exists in `src/rooms/create.rs`
- AC2: Protected by session middleware — requires authentication
- AC3: Room name validation: 1-64 characters, non-empty
- AC4: Language validation against `Language` enum, defaults to 'plaintext'
- AC5: Generates unique 6-character alphanumeric code
- AC6: Room INSERT succeeds
- AC7: Room member INSERT with user as owner
- AC8: Returns `201 Created` with `{roomId, roomCode, name, language}`
- AC9: Returns `409 Conflict` if room code collision (retry with new code)
- AC10: Returns `401 Unauthorized` if no valid session
- AC11: Handler wired into `main.rs` router

## Technical Hints

- Room code generation: `(&mut rand::thread_rng()).sample_iter(&Alphanumeric).take(6).collect()`
- Use `rand::distributions::Alphanumeric` for alphanumeric characters
- Collision handling: check `INSERT` result, retry up to 5 times
- Use `axum::Extension(session)` or `ValidSession` extract to get the user
- Refer to: https://docs.rs/rand/latest/rand/distributions/struct.Alphanumeric.html
