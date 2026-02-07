use thiserror::Error;

pub type Result<T> = std::result::Result<T, AgentError>;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Core error: {0}")]
    CoreError(#[from] hlavi_core::error::HlaviError),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Model API error: {0}")]
    ModelApiError(String),

    #[error("Planning failed: {0}")]
    PlanningError(String),

    #[error("Execution failed: {0}")]
    ExecutionError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Agent not configured. Please configure the agent first.")]
    NotConfigured,

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}
