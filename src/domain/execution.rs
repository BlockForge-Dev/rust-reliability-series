use serde::{Deserialize, Serialize};

use crate::domain::{errors::AppError, status::ExecutionStatus};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateExecutionRequest {
    pub action_type: String,
    pub target_id: String,
    pub amount: i64,
}

impl CreateExecutionRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.action_type.trim().is_empty() {
            return Err(AppError::Validation(
                "action_type must not be empty".to_string(),
            ));
        }

        if self.target_id.trim().is_empty() {
            return Err(AppError::Validation(
                "target_id must not be empty".to_string(),
            ));
        }

        if self.amount <= 0 {
            return Err(AppError::Validation(
                "amount must be greater than zero".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionRecord {
    pub id: String,
    pub action_type: String,
    pub target_id: String,
    pub amount: i64,
    pub status: ExecutionStatus,
    pub provider_status: Option<String>,
    pub provider_message: Option<String>,
    pub correlation_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct NewExecution {
    pub id: String,
    pub action_type: String,
    pub target_id: String,
    pub amount: i64,
    pub status: ExecutionStatus,
    pub correlation_id: String,
}

#[derive(Debug, Clone)]
pub struct StoredProviderResponse {
    pub provider_status: String,
    pub raw_message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateExecutionResponse {
    pub correlation_id: String,
    pub execution: ExecutionRecord,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetExecutionResponse {
    pub correlation_id: String,
    pub execution: ExecutionRecord,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

impl HealthResponse {
    pub fn ok() -> Self {
        Self {
            status: "ok",
            service: "rust-reliability-series",
            version: "v0-weak-system",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CreateExecutionRequest;

    #[test]
    fn rejects_zero_amount() {
        let request = CreateExecutionRequest {
            action_type: "issue_refund".to_string(),
            target_id: "pay_123".to_string(),
            amount: 0,
        };

        assert!(request.validate().is_err());
    }
}
