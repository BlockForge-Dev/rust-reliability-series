use std::sync::{Arc, Mutex};

use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, params, types::Type};

use crate::domain::{
    errors::AppError,
    execution::{ExecutionRecord, NewExecution, StoredProviderResponse},
    status::ExecutionStatus,
};

#[derive(Debug)]
pub struct ExecutionRepository {
    connection: Arc<Mutex<Connection>>,
}

impl ExecutionRepository {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub fn create_execution(
        &self,
        execution: NewExecution,
        provider_response: StoredProviderResponse,
    ) -> Result<ExecutionRecord, AppError> {
        let timestamp = Utc::now().to_rfc3339();
        let id = execution.id.clone();
        let mut connection = self.connection.lock()?;
        let transaction = connection.transaction()?;

        transaction.execute(
            r#"
            INSERT INTO executions (
                id,
                action_type,
                target_id,
                amount,
                status,
                correlation_id,
                created_at,
                updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                execution.id,
                execution.action_type,
                execution.target_id,
                execution.amount,
                execution.status.as_str(),
                execution.correlation_id,
                timestamp,
                timestamp,
            ],
        )?;

        transaction.execute(
            r#"
            INSERT INTO provider_responses (
                execution_id,
                provider_status,
                raw_message,
                created_at
            ) VALUES (?1, ?2, ?3, ?4)
            "#,
            params![
                id,
                provider_response.provider_status,
                provider_response.raw_message,
                Utc::now().to_rfc3339(),
            ],
        )?;

        transaction.commit()?;
        drop(connection);

        self.get_execution(&id)?
            .ok_or_else(|| AppError::NotFound(id.clone()))
    }

    pub fn get_execution(&self, id: &str) -> Result<Option<ExecutionRecord>, AppError> {
        let connection = self.connection.lock()?;
        let execution = connection
            .query_row(
                r#"
                SELECT
                    e.id,
                    e.action_type,
                    e.target_id,
                    e.amount,
                    e.status,
                    e.correlation_id,
                    e.created_at,
                    e.updated_at,
                    pr.provider_status,
                    pr.raw_message
                FROM executions e
                LEFT JOIN provider_responses pr
                    ON pr.execution_id = e.id
                WHERE e.id = ?1
                "#,
                params![id],
                |row| {
                    let raw_status: String = row.get(4)?;
                    let status = ExecutionStatus::from_db(&raw_status).map_err(|error| {
                        rusqlite::Error::FromSqlConversionFailure(4, Type::Text, Box::new(error))
                    })?;

                    Ok(ExecutionRecord {
                        id: row.get(0)?,
                        action_type: row.get(1)?,
                        target_id: row.get(2)?,
                        amount: row.get(3)?,
                        status,
                        correlation_id: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                        provider_status: row.get(8)?,
                        provider_message: row.get(9)?,
                    })
                },
            )
            .optional()?;

        Ok(execution)
    }
}
