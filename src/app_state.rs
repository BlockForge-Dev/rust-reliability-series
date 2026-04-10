use std::sync::Arc;

use crate::{provider::adapter::ProviderAdapter, storage::repository::ExecutionRepository};

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<ExecutionRepository>,
    pub provider_adapter: ProviderAdapter,
}

impl AppState {
    pub fn new(repository: Arc<ExecutionRepository>, provider_adapter: ProviderAdapter) -> Self {
        Self {
            repository,
            provider_adapter,
        }
    }
}
