//! Database initialization and connection management.
//!
//! The database file lives at `.brain/brain.db` within the project root.
//! On first initialization, all migrations are run to create the schema.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use sentinel_arc_core::BrainError;

/// The main database handle for Sentinel Arc.
///
/// Wraps a SQLx connection pool to `.brain/brain.db`.
#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
    db_path: PathBuf,
}

impl Database {
    /// Initialize a new database at the given project root.
    ///
    /// This creates the `.brain/` directory and `brain.db` file if they
    /// don't exist, and runs all pending migrations.
    pub async fn init(project_root: &Path) -> Result<Self, BrainError> {
        let brain_dir = project_root.join(".brain");
        std::fs::create_dir_all(&brain_dir)
            .map_err(|e| BrainError::storage(format!("Failed to create .brain directory: {e}")))?;

        let db_path = brain_dir.join("brain.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        let options = SqliteConnectOptions::from_str(&db_url)
            .map_err(|e| BrainError::storage(format!("Invalid database URL: {e}")))?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| BrainError::storage(format!("Failed to connect to database: {e}")))?;

        let db = Self { pool, db_path };
        db.run_migrations().await?;
        Ok(db)
    }

    /// Initialize a database from a specific SQLite file path (for testing).
    pub async fn init_with_path(db_path: &Path) -> Result<Self, BrainError> {
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        let options = SqliteConnectOptions::from_str(&db_url)
            .map_err(|e| BrainError::storage(format!("Invalid database URL: {e}")))?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .foreign_keys(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .map_err(|e| BrainError::storage(format!("Failed to connect to database: {e}")))?;

        let db = Self {
            pool,
            db_path: db_path.to_path_buf(),
        };
        db.run_migrations().await?;
        Ok(db)
    }

    /// Run all database migrations.
    async fn run_migrations(&self) -> Result<(), BrainError> {
        // Create tables directly using SQL statements.
        // This approach avoids needing the sqlx migrate macro at compile time.
        let migrations = [
            include_str!("../migrations/001_create_nodes.sql"),
            include_str!("../migrations/002_create_relationships.sql"),
            include_str!("../migrations/003_create_events.sql"),
            include_str!("../migrations/004_create_rules.sql"),
            include_str!("../migrations/005_create_plugin_data.sql"),
        ];

        for migration in &migrations {
            sqlx::query(migration)
                .execute(&self.pool)
                .await
                .map_err(|e| BrainError::storage(format!("Migration failed: {e}")))?;
        }

        Ok(())
    }

    /// Return a reference to the connection pool.
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Return the path to the database file.
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    /// Close the database connection pool.
    pub async fn close(&self) {
        self.pool.close().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn database_init_creates_file_and_tables() {
        let tmp = TempDir::new().unwrap();
        let db = Database::init(tmp.path()).await.unwrap();

        // Verify the .brain directory and brain.db file exist
        assert!(tmp.path().join(".brain").exists());
        assert!(tmp.path().join(".brain/brain.db").exists());

        // Verify all 5 tables exist by querying sqlite_master
        let tables: Vec<(String,)> = sqlx::query_as(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
        )
        .fetch_all(db.pool())
        .await
        .unwrap();

        let table_names: Vec<&str> = tables.iter().map(|t| t.0.as_str()).collect();
        assert!(table_names.contains(&"nodes"), "Missing nodes table");
        assert!(
            table_names.contains(&"relationships"),
            "Missing relationships table"
        );
        assert!(table_names.contains(&"events"), "Missing events table");
        assert!(table_names.contains(&"rules"), "Missing rules table");
        assert!(
            table_names.contains(&"plugin_data"),
            "Missing plugin_data table"
        );

        db.close().await;
    }

    #[tokio::test]
    async fn database_init_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        let db1 = Database::init(tmp.path()).await.unwrap();
        db1.close().await;

        // Second init should succeed without errors
        let db2 = Database::init(tmp.path()).await.unwrap();
        db2.close().await;
    }
}
