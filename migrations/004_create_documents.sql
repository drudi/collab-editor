-- P1-T08: Create documents table
-- P4-T01: Add unique constraint on room_id for INSERT OR REPLACE upsert
CREATE TABLE documents (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id           INTEGER NOT NULL UNIQUE REFERENCES rooms(id),
    content_snapshot  BLOB,
    version           INTEGER DEFAULT 0,
    saved_at          DATETIME DEFAULT CURRENT_TIMESTAMP
);
