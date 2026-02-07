use crate::{config::AgentConfig, error::Result};
use hlavi_core::domain::ticket::Ticket;

/// Task planner that generates acceptance criteria for tickets
pub struct Planner {
    _config: AgentConfig,
}

impl Planner {
    pub fn new(config: AgentConfig) -> Self {
        Self { _config: config }
    }

    /// Generates acceptance criteria for a ticket based on its title and description
    pub async fn generate_plan(&self, _ticket: &Ticket) -> Result<Vec<String>> {
        // TODO: Implement AI-powered planning
        // This will call the configured model API to generate acceptance criteria
        // based on the ticket title and description

        Ok(vec![
            "Implement core functionality".to_string(),
            "Add tests".to_string(),
            "Update documentation".to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_planner_creation() {
        let config = AgentConfig::default();
        let _planner = Planner::new(config);
    }
}
