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

/// Connect to the SQLite database and run pending migrations.
///
/// # Errors
///
/// Returns `sqlx::Error` if the connection or migration fails.
pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}