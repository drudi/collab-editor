# P4-T01 — Document Persistence

**Status:** TODO

## Goal

Implement periodic and event-driven document snapshot persistence to the database.

## Description

Implement document persistence that saves the collaborative document state to the SQLite database periodically and on room close. This ensures work is not lost if the server restarts or clients disconnect.

Backend changes:
- Add `save_document(pool, room_id, content_snapshot, version)` to `db/queries.rs`
- Add `get_latest_document(pool, room_id)` to `db/queries.rs`
- In `collaboration/room_state.rs`, add a periodic save task

The save task:
1. Runs every 30 seconds
2. Serializes `yrs::Doc` state to bytes via `encode_state_as_update_v1`
3. Stores as BLOB in `documents` table via `INSERT OR REPLACE`
4. On room disconnect: force a save before cleanup

The load flow (on room connect):
1. Query `get_latest_document(room_id)` for last saved state
2. If snapshot exists, apply via `doc.transact_mut().apply_update(snapshot)`
3. Set state vector to match loaded version

## Acceptance Criteria

- AC1: `save_document` function in `db/queries.rs`
- AC2: `get_latest_document` function in `db/queries.rs`
- AC3: Periodic save task runs every 30 seconds per room
- AC4: Save on room disconnect before cleanup
- AC5: `INSERT OR REPLACE` for upsert behavior
- AC6: `content_snapshot` stored as BLOB
- AC7: Version field incremented on each save
- AC8: Load flow applies snapshot via `apply_update()`
- AC9: `cargo check` passes

## Technical Hints

- Periodic save pattern:
  ```rust
  tokio::spawn(async move {
      let mut interval = tokio::time::interval(Duration::from_secs(30));
      loop {
          interval.tick().await;
          save_room_document(&room_state).await;
      }
  });
  ```
- Yrs state encoding: `doc.transact().encode_state_as_update_v1(&StateVector::default())`
- State decoding: `Update::decode_v1(&bytes)` then `doc.transact_mut().apply_update(update)`
- BLOB in SQLx: `query!("INSERT OR REPLACE INTO documents (room_id, content_snapshot, version, saved_at) VALUES (?, ?, ?, ?)", room_id, snapshot, version, chrono::Utc::now())`
- Refer to: https://docs.rs/sqlx/latest/sqlx/macro.query.html
