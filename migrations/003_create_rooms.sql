-- P1-T07: Create rooms and room_members tables
CREATE TABLE rooms (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    code        TEXT    UNIQUE NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT,
    language    TEXT    DEFAULT 'plain_text',
    owner_id    INTEGER NOT NULL REFERENCES users(id),
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE room_members (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id      INTEGER NOT NULL REFERENCES rooms(id),
    user_id      INTEGER NOT NULL REFERENCES users(id),
    member_type  TEXT    NOT NULL DEFAULT 'viewer',
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (room_id, user_id)
);
