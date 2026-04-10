use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use tracing::info;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    domain::{
        errors::AppError,
        execution::{
            CreateExecutionRequest, CreateExecutionResponse, GetExecutionResponse, HealthResponse,
            NewExecution, StoredProviderResponse,
        },
    },
};

pub async fn healthcheck() -> Json<HealthResponse> {
    Json(HealthResponse::ok())
}

pub async fn create_execution(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateExecutionRequest>,
) -> Result<(StatusCode, Json<CreateExecutionResponse>), AppError> {
    payload.validate()?;

    let correlation_id = resolve_correlation_id(&headers);
    let execution_id = Uuid::new_v4().to_string();

    info!(
        %correlation_id,
        %execution_id,
        action_type = %payload.action_type,
        target_id = %payload.target_id,
        amount = payload.amount,
        "received execution request"
    );

    let provider_result = state
        .provider_adapter
        .execute(&payload, &execution_id, &correlation_id)
        .await?;

    let stored_execution = state.repository.create_execution(
        NewExecution {
            id: execution_id,
            action_type: payload.action_type.clone(),
            target_id: payload.target_id.clone(),
            amount: payload.amount,
            status: provider_result.execution_status.clone(),
            correlation_id: correlation_id.clone(),
        },
        StoredProviderResponse {
            provider_status: provider_result.provider_status,
            raw_message: provider_result.raw_message,
        },
    )?;

    info!(
        %correlation_id,
        execution_status = %stored_execution.status,
        provider_status = stored_execution.provider_status.as_deref().unwrap_or("unknown"),
        "execution persisted"
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateExecutionResponse {
            correlation_id,
            execution: stored_execution,
        }),
    ))
}

pub async fn get_execution(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<GetExecutionResponse>, AppError> {
    let correlation_id = resolve_correlation_id(&headers);

    info!(%correlation_id, %id, "fetching execution");

    let execution = state
        .repository
        .get_execution(&id)?
        .ok_or_else(|| AppError::NotFound(id.clone()))?;

    Ok(Json(GetExecutionResponse {
        correlation_id,
        execution,
    }))
}

fn resolve_correlation_id(headers: &HeaderMap) -> String {
    headers
        .get("x-correlation-id")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("corr_{}", Uuid::new_v4()))
}
