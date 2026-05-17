# P1-T08 — Migration 004: Documents Table

**Status:** TODO

## Goal

Create the database migration for the documents table.

## Description

Write `migrations/004_create_documents.sql` that creates the documents table. This table stores periodic snapshots of each room's document content for persistence and recovery.

```sql
CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id INTEGER NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    content_snapshot BLOB NOT NULL,
    version INTEGER NOT NULL DEFAULT 0,
    saved_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_documents_room ON documents(room_id);
```

The `content_snapshot` is a BLOB that stores the CRDT document state serialized as bytes (using yrs encoding). The `version` field tracks update sequence for conflict resolution during sync.

## Acceptance Criteria

- AC1: `004_create_documents.sql` exists in `migrations/` directory
- AC2: `documents` table created with all five columns
- AC3: `content_snapshot` column type is BLOB
- AC4: `version` has default of 0
- AC5: Foreign key on `room_id` with `ON DELETE CASCADE`
- AC6: Index on `room_id` for fast document retrieval
- AC7: SQL validates with `sqlite3` syntax

## Technical Hints

- BLOB type stores the binary CRDT state from `yrs::updates::encoder::Encode::encode_v1()`
- Version tracks the yrs state vector for differential sync
- On room delete, `ON DELETE CASCADE` cleans up associated documents automatically
- Refer to: https://www.sqlite.org/datatype3.html#blobs
