-- P1-T08: Create documents table
CREATE TABLE documents (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id           INTEGER NOT NULL REFERENCES rooms(id),
    content_snapshot  BLOB,
    version           INTEGER DEFAULT 0,
    saved_at          DATETIME DEFAULT CURRENT_TIMESTAMP
);
