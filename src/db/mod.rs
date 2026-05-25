//! Database module — connection pool, migrations, and query helpers.
//!
//! Manages the SQLite database connection lifecycle and provides
//! strongly-typed query functions for data access.
//!
//! - `connect()`: Initialize connection pool and run migrations
//! - `queries`: Raw sqlx query helpers for CRUD operations

pub mod queries;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

/// Ensure the SQLite database file exists by creating parent directories
/// and the file itself if needed. Skips in-memory databases (`:memory:`).
///
/// # Panics
///
/// Panics if parent directories or the file cannot be created.
fn ensure_db_file(database_url: &str) {
    // `sqlite::memory:` is in-memory — no file needed.
    if database_url == "sqlite::memory:" {
        return;
    }

    // Extract the file path from `sqlite:path`.
    let file_path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
    let path = Path::new(file_path);

    // Only handle file-based databases (must have a filename component).
    if path.has_root() || path.file_name().is_some() {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).expect("Failed to create database directories");
            }
        }
        if !path.exists() {
            fs::File::create(path).expect("Failed to create database file");
        }
    }
}

/// Connect to the SQLite database and run pending migrations.
///
/// # Errors
///
/// Returns `sqlx::Error` if the connection or migration fails.
pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    ensure_db_file(database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}