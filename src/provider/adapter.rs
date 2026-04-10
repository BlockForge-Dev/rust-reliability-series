use crate::domain::{errors::AppError, execution::CreateExecutionRequest};

use super::fake_provider::{FakeProvider, ProviderCallResult};

#[derive(Debug, Clone)]
pub struct ProviderAdapter {
    fake_provider: FakeProvider,
}

impl ProviderAdapter {
    pub fn new(fake_provider: FakeProvider) -> Self {
        Self { fake_provider }
    }

    pub async fn execute(
        &self,
        request: &CreateExecutionRequest,
        execution_id: &str,
        correlation_id: &str,
    ) -> Result<ProviderCallResult, AppError> {
        self.fake_provider
            .execute(request, execution_id, correlation_id)
            .await
    }
}
