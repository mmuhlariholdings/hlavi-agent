use serde::{Deserialize, Serialize};

/// AI model provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum ModelProvider {
    Anthropic {
        api_key: String,
        model: String, // e.g., "claude-3-5-sonnet-20241022"
    },
    OpenAI {
        api_key: String,
        model: String, // e.g., "gpt-4"
    },
    Local {
        endpoint: String,
        model: String,
    },
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Model provider configuration
    pub provider: ModelProvider,

    /// Maximum number of retry attempts
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Temperature for model generation (0.0 - 1.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

fn default_max_retries() -> u32 {
    3
}

fn default_temperature() -> f32 {
    0.7
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            provider: ModelProvider::Anthropic {
                api_key: String::new(),
                model: "claude-3-5-sonnet-20241022".to_string(),
            },
            max_retries: default_max_retries(),
            temperature: default_temperature(),
        }
    }
}

impl AgentConfig {
    /// Validates the configuration
    pub fn validate(&self) -> Result<(), crate::error::AgentError> {
        match &self.provider {
            ModelProvider::Anthropic { api_key, .. } | ModelProvider::OpenAI { api_key, .. } => {
                if api_key.is_empty() {
                    return Err(crate::error::AgentError::ConfigError(
                        "API key is required".to_string(),
                    ));
                }
            }
            ModelProvider::Local { endpoint, .. } => {
                if endpoint.is_empty() {
                    return Err(crate::error::AgentError::ConfigError(
                        "Endpoint URL is required".to_string(),
                    ));
                }
            }
        }

        if !(0.0..=1.0).contains(&self.temperature) {
            return Err(crate::error::AgentError::ConfigError(
                "Temperature must be between 0.0 and 1.0".to_string(),
            ));
        }

        Ok(())
    }
}
