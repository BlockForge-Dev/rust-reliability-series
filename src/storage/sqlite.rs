use std::{fs, path::Path};

use rusqlite::Connection;

use crate::domain::errors::AppError;

pub fn init_sqlite(database_path: &str) -> Result<Connection, AppError> {
    if let Some(parent) = Path::new(database_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let connection = Connection::open(database_path)?;
    connection.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS executions (
            id TEXT PRIMARY KEY,
            action_type TEXT NOT NULL,
            target_id TEXT NOT NULL,
            amount INTEGER NOT NULL,
            status TEXT NOT NULL,
            correlation_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS provider_responses (
            execution_id TEXT PRIMARY KEY,
            provider_status TEXT NOT NULL,
            raw_message TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(execution_id) REFERENCES executions(id)
        );
        "#,
    )?;

    Ok(connection)
}
