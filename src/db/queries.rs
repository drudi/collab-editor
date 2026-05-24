//! Database query helpers — raw sqlx CRUD operations.
//!
//! Provides strongly-typed functions for common data access patterns
//! across users, sessions, rooms, and members.
//!
//! SQLite's type system reports some columns as nullable at reflection time,
//! so we use `query!` with explicit `.map()` to unwrap `Option<T>` where
//! needed (all our columns are NOT NULL or have defaults).

use sqlx::{query, SqlitePool};

use crate::models::{Document, Room, RoomMemberType, Session, User};

// ── Users ────────────────────────────────────────────────────────────────

/// Insert a new user. Returns the new row's `id`.
pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
) -> Result<i64, sqlx::Error> {
    let id = query!(
        r#"INSERT INTO users (username, password_hash) VALUES ($1, $2)"#,
        username,
        password_hash,
    )
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(id)
}

/// Look up a user by username (for login / registration conflict check).
pub async fn get_user_by_username(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    query!(
        r#"SELECT id, username, password_hash, created_at FROM users WHERE username = $1"#,
        username,
    )
    .map(|row| User {
        id: row.id.expect("id is PK"),
        username: row.username,
        password_hash: row.password_hash,
        created_at: row.created_at.expect("created_at has DEFAULT"),
    })
    .fetch_optional(pool)
    .await
}

/// Look up a user by id.
pub async fn get_user_by_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Option<User>, sqlx::Error> {
    query!(
        r#"SELECT id, username, password_hash, created_at FROM users WHERE id = $1"#,
        user_id,
    )
    .map(|row| User {
        id: row.id,
        username: row.username,
        password_hash: row.password_hash,
        created_at: row.created_at.expect("created_at has DEFAULT"),
    })
    .fetch_optional(pool)
    .await
}

// ── Sessions ─────────────────────────────────────────────────────────────

/// Insert a new session and return the full row.
pub async fn insert_session(
    pool: &SqlitePool,
    user_id: i64,
    token: &str,
    expires_at: &str,
) -> Result<Session, sqlx::Error> {
    query!(
        r#"INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, expires_at, created_at"#,
        user_id,
        token,
        expires_at,
    )
    .map(|row| Session {
        id: row.id.expect("id is PK"),
        user_id: row.user_id,
        token: row.token,
        expires_at: row.expires_at,
        created_at: row.created_at.expect("created_at has DEFAULT"),
    })
    .fetch_one(pool)
    .await
}

/// Look up a session by token (for auth middleware).
pub async fn get_session_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<Session>, sqlx::Error> {
    query!(
        r#"SELECT id, user_id, token, expires_at, created_at
            FROM sessions
            WHERE token = $1"#,
        token,
    )
    .map(|row| Session {
        id: row.id.expect("id is PK"),
        user_id: row.user_id,
        token: row.token,
        expires_at: row.expires_at,
        created_at: row.created_at.expect("created_at has DEFAULT"),
    })
    .fetch_optional(pool)
    .await
}

/// Delete a session by token (for logout).
pub async fn delete_session_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<(), sqlx::Error> {
    query!(
        r#"DELETE FROM sessions WHERE token = $1"#,
        token,
    )
    .execute(pool)
    .await?;
    Ok(())
}

// ── Rooms ────────────────────────────────────────────────────────────────

/// Create a new room. Returns the inserted `Room`.
pub async fn create_room(
    pool: &SqlitePool,
    code: &str,
    name: &str,
    description: Option<&str>,
    language: &str,
    owner_id: i64,
) -> Result<Room, sqlx::Error> {
    query!(
        r#"INSERT INTO rooms (code, name, description, language, owner_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, code, name, description, language, owner_id, created_at"#,
        code,
        name,
        description,
        language,
        owner_id,
    )
    .map(|row| Room {
        id: row.id.expect("id is PK"),
        code: row.code,
        name: row.name,
        description: row.description,
        language: row.language.as_deref().map(|s| s.parse()).transpose().expect("language parse failed"),
        owner_id: row.owner_id,
        created_at: row.created_at.expect("created_at has DEFAULT"),
    })
    .fetch_one(pool)
    .await
}

/// Fetch room by id. Returns `None` if room doesn't exist.
pub async fn get_room_by_id(
    pool: &SqlitePool,
    room_id: i64,
) -> Result<Option<Room>, sqlx::Error> {
    query!(
        r#"SELECT id, code, name, description, language, owner_id, created_at
            FROM rooms WHERE id = $1"#,
        room_id,
    )
    .map(|row| Room {
        id: row.id,
        code: row.code,
        name: row.name,
        description: row.description,
        language: row.language.as_deref().map(|s| s.parse()).transpose().expect("language parse failed"),
        owner_id: row.owner_id,
        created_at: row.created_at.expect("created_at has DEFAULT"),
    })
    .fetch_optional(pool)
    .await
}

/// Add a user as a member of a room.
pub async fn add_room_member(
    pool: &SqlitePool,
    room_id: i64,
    user_id: i64,
    member_type: &str,
) -> Result<(), sqlx::Error> {
    query!(
        r#"INSERT INTO room_members (room_id, user_id, member_type)
            VALUES ($1, $2, $3)"#,
        room_id,
        user_id,
        member_type,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Fetch all members of a room as (user_id, username, member_type).
pub async fn get_room_members(
    pool: &SqlitePool,
    room_id: i64,
) -> Result<Vec<(i64, String, RoomMemberType)>, sqlx::Error> {
    query!(
        r#"SELECT rm.user_id, u.username, rm.member_type
            FROM room_members rm
            JOIN users u ON u.id = rm.user_id
            WHERE rm.room_id = $1"#,
        room_id,
    )
    .map(|row| {
        (
            row.user_id,
            row.username,
            row.member_type.parse().expect("member_type parse failed"),
        )
    })
    .fetch_all(pool)
    .await
}

// ── Documents ────────────────────────────────────────────────────────────

/// Insert a document snapshot for a room.
pub async fn save_document(
    pool: &SqlitePool,
    room_id: i64,
    content_snapshot: &[u8],
    version: i32,
) -> Result<(), sqlx::Error> {
    let version_i64 = version as i64;
    query!(
        r#"INSERT INTO documents (room_id, content_snapshot, version)
            VALUES ($1, $2, $3)"#,
        room_id,
        content_snapshot,
        version_i64,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Fetch the latest document for a room.
pub async fn get_latest_document(
    pool: &SqlitePool,
    room_id: i64,
) -> Result<Option<Document>, sqlx::Error> {
    query!(
        r#"SELECT id, room_id, content_snapshot, version, saved_at
            FROM documents
            WHERE room_id = $1
            ORDER BY version DESC
            LIMIT 1"#,
        room_id,
    )
    .map(|row| Document {
        id: row.id,
        room_id: row.room_id,
        content_snapshot: row.content_snapshot.unwrap_or_default(),
        version: row.version.expect("version has DEFAULT 0") as i32,
        saved_at: row.saved_at.expect("saved_at has DEFAULT"),
    })
    .fetch_optional(pool)
    .await
}

// ── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect;

    async fn test_pool() -> SqlitePool {
        connect("sqlite::memory:").await.expect("test pool connect")
    }

    #[tokio::test]
    async fn test_create_user_and_get_by_username() {
        let pool = test_pool().await;
        let id = create_user(&pool, "alice", "hash123")
            .await
            .expect("create user");
        assert_eq!(id, 1);

        let user = get_user_by_username(&pool, "alice")
            .await
            .expect("get user");
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.username, "alice");
        assert_eq!(user.password_hash, "hash123");
    }

    #[tokio::test]
    async fn test_get_user_by_nonexistent_username() {
        let pool = test_pool().await;
        let user = get_user_by_username(&pool, "nobody")
            .await
            .expect("get user");
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let pool = test_pool().await;
        let id = create_user(&pool, "bob", "hash456")
            .await
            .expect("create user");

        let user = get_user_by_id(&pool, id)
            .await
            .expect("get user");
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "bob");

        // nonexistent id
        let user = get_user_by_id(&pool, 99999)
            .await
            .expect("get user");
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_insert_and_get_session() {
        let pool = test_pool().await;
        let user_id = create_user(&pool, "charlie", "hash789")
            .await
            .expect("create user");

        let session = insert_session(&pool, user_id, "tok1", "2027-01-01T00:00:00")
            .await
            .expect("insert session");
        assert_eq!(session.token, "tok1");
        assert_eq!(session.user_id, user_id);

        let found = get_session_by_token(&pool, "tok1")
            .await
            .expect("get session");
        assert!(found.is_some());
        assert_eq!(found.unwrap().token, "tok1");
    }

    #[tokio::test]
    async fn test_delete_session() {
        let pool = test_pool().await;
        let user_id = create_user(&pool, "dave", "hash000")
            .await
            .expect("create user");
        insert_session(&pool, user_id, "tok2", "2027-01-01T00:00:00")
            .await
            .expect("insert session");

        delete_session_by_token(&pool, "tok2")
            .await
            .expect("delete session");

        let found = get_session_by_token(&pool, "tok2")
            .await
            .expect("get session");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_create_and_get_room() {
        let pool = test_pool().await;
        let user_id = create_user(&pool, "eve", "hash111")
            .await
            .expect("create user");

        let room = create_room(&pool, "ABC123", "My Room", Some("A test room"), "rust", user_id)
            .await
            .expect("create room");
        assert_eq!(room.code, "ABC123");
        assert_eq!(room.name, "My Room");
        assert_eq!(room.description, Some("A test room".into()));

        let found = get_room_by_id(&pool, room.id)
            .await
            .expect("get room");
        assert!(found.is_some());
        assert_eq!(found.unwrap().code, "ABC123");

        // nonexistent room
        let found = get_room_by_id(&pool, 99999)
            .await
            .expect("get room");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_add_and_get_room_members() {
        let pool = test_pool().await;
        let owner_id = create_user(&pool, "owner", "hash222")
            .await
            .expect("create user");
        let member_id = create_user(&pool, "member", "hash333")
            .await
            .expect("create user");

        let room = create_room(&pool, "XYZ789", "Team Room", None, "javascript", owner_id)
            .await
            .expect("create room");

        add_room_member(&pool, room.id, owner_id, "owner")
            .await
            .expect("add owner");
        add_room_member(&pool, room.id, member_id, "editor")
            .await
            .expect("add editor");

        let members = get_room_members(&pool, room.id)
            .await
            .expect("get members");
        assert_eq!(members.len(), 2);
        assert!(members.iter().any(|(_, u, t)| u == "owner" && *t == RoomMemberType::Owner));
        assert!(members.iter().any(|(_, u, t)| u == "member" && *t == RoomMemberType::Editor));

        // empty room
        let members = get_room_members(&pool, 99999)
            .await
            .expect("get members");
        assert!(members.is_empty());
    }

    #[tokio::test]
    async fn test_save_and_get_document() {
        let pool = test_pool().await;
        let user_id = create_user(&pool, "frank", "hash444")
            .await
            .expect("create user");
        let room = create_room(&pool, "DOC001", "Doc Room", None, "python", user_id)
            .await
            .expect("create room");

        let content = b"fn main() { println!(\"hello\"); }";
        save_document(&pool, room.id, content, 1)
            .await
            .expect("save doc");

        let doc = get_latest_document(&pool, room.id)
            .await
            .expect("get doc");
        assert!(doc.is_some());
        let doc = doc.unwrap();
        assert_eq!(doc.version, 1);
        assert_eq!(doc.content_snapshot, content.as_slice());

        // nonexistent room
        let doc = get_latest_document(&pool, 99999)
            .await
            .expect("get doc");
        assert!(doc.is_none());
    }

    #[tokio::test]
    async fn test_document_latest_version_wins() {
        let pool = test_pool().await;
        let user_id = create_user(&pool, "grace", "hash555")
            .await
            .expect("create user");
        let room = create_room(&pool, "DOC002", "Ver Room", None, "plain_text", user_id)
            .await
            .expect("create room");

        save_document(&pool, room.id, b"v1", 1)
            .await
            .expect("save v1");
        save_document(&pool, room.id, b"v2", 2)
            .await
            .expect("save v2");

        let doc = get_latest_document(&pool, room.id)
            .await
            .expect("get doc");
        let doc = doc.unwrap();
        assert_eq!(doc.version, 2);
        assert_eq!(doc.content_snapshot, b"v2");
    }

    #[tokio::test]
    async fn test_room_with_plain_text_language() {
        let pool = test_pool().await;
        let user_id = create_user(&pool, "hank", "hash666")
            .await
            .expect("create user");

        let room = create_room(&pool, "NLL001", "No Lang", None, "plain_text", user_id)
            .await
            .expect("create room");
        assert_eq!(room.language, Some(crate::models::Language::PlainText));
    }
}
