use axum::{
    Router,
    routing::{get, post},
};

use crate::app_state::AppState;

use super::handlers::{create_execution, get_execution, healthcheck};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(healthcheck))
        .route("/executions", post(create_execution))
        .route("/executions/{id}", get(get_execution))
        .with_state(state)
}
