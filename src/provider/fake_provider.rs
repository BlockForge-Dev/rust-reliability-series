use std::time::Duration;

use tokio::time::sleep;
use tracing::{info, warn};

use crate::domain::{errors::AppError, execution::CreateExecutionRequest, status::ExecutionStatus};

#[derive(Debug, Clone)]
pub struct ProviderCallResult {
    pub execution_status: ExecutionStatus,
    pub provider_status: String,
    pub raw_message: String,
}

#[derive(Debug, Clone)]
pub struct FakeProvider {
    baseline_latency_ms: u64,
}

impl Default for FakeProvider {
    fn default() -> Self {
        Self {
            baseline_latency_ms: 250,
        }
    }
}

impl FakeProvider {
    pub async fn execute(
        &self,
        request: &CreateExecutionRequest,
        execution_id: &str,
        correlation_id: &str,
    ) -> Result<ProviderCallResult, AppError> {
        let simulated_latency_ms = if request.action_type.contains("slow") {
            self.baseline_latency_ms + 2_500
        } else {
            self.baseline_latency_ms
        };

        info!(
            %correlation_id,
            %execution_id,
            simulated_latency_ms,
            "calling fake provider directly in request path"
        );

        sleep(Duration::from_millis(simulated_latency_ms)).await;

        if request.action_type.contains("error") {
            warn!(
                %correlation_id,
                %execution_id,
                "simulated fake provider transport error"
            );

            return Err(AppError::Provider(
                "simulated provider transport error".to_string(),
            ));
        }

        let (execution_status, provider_status, raw_message) =
            if request.action_type.contains("fail") {
                (
                    ExecutionStatus::Failed,
                    "declined".to_string(),
                    format!(
                        "fake provider declined action_type={} execution_id={} correlation_id={}",
                        request.action_type, execution_id, correlation_id
                    ),
                )
            } else {
                (
                    ExecutionStatus::Succeeded,
                    "approved".to_string(),
                    format!(
                        "fake provider approved action_type={} execution_id={} correlation_id={}",
                        request.action_type, execution_id, correlation_id
                    ),
                )
            };

        Ok(ProviderCallResult {
            execution_status,
            provider_status,
            raw_message,
        })
    }
}
