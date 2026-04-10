use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Accepted,
    Succeeded,
    Failed,
}

impl ExecutionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, AppError> {
        match value {
            "accepted" => Ok(Self::Accepted),
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            other => Err(AppError::StorageSetup(format!(
                "unknown execution status `{other}` in sqlite"
            ))),
        }
    }
}

impl Display for ExecutionStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionStatus;

    #[test]
    fn parses_valid_statuses() {
        assert_eq!(
            ExecutionStatus::from_db("succeeded").unwrap(),
            ExecutionStatus::Succeeded
        );
        assert_eq!(
            ExecutionStatus::from_db("failed").unwrap(),
            ExecutionStatus::Failed
        );
    }
}
