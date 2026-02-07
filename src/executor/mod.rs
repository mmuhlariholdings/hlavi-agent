use crate::{config::AgentConfig, error::Result, AgentMode, AgentState};
use hlavi_core::domain::ticket::Ticket;

/// Task executor that completes acceptance criteria
pub struct Executor {
    _config: AgentConfig,
    mode: AgentMode,
    state: AgentState,
}

impl Executor {
    pub fn new(config: AgentConfig, mode: AgentMode) -> Self {
        Self {
            _config: config,
            mode,
            state: AgentState::Idle,
        }
    }

    /// Gets the current agent state
    pub fn state(&self) -> &AgentState {
        &self.state
    }

    /// Executes a single acceptance criterion
    pub async fn execute_criterion(
        &mut self,
        _ticket: &Ticket,
        criterion_id: usize,
    ) -> Result<ExecutionResult> {
        self.state = AgentState::Executing;

        // TODO: Implement AI-powered execution
        // This will:
        // 1. Understand the acceptance criterion
        // 2. Generate a plan to complete it
        // 3. Execute the plan (modify files, run commands, etc.)
        // 4. Verify completion

        self.state = if self.mode == AgentMode::Attended {
            AgentState::AwaitingApproval
        } else {
            AgentState::Executing
        };

        Ok(ExecutionResult {
            success: true,
            message: format!("Completed criterion {}", criterion_id),
            changes: vec![],
        })
    }

    /// Completes all acceptance criteria for a ticket
    pub async fn execute_ticket(&mut self, ticket: &Ticket) -> Result<Vec<ExecutionResult>> {
        let mut results = Vec::new();

        for ac in &ticket.acceptance_criteria {
            let result = self.execute_criterion(ticket, ac.id).await?;
            results.push(result);

            // In attended mode, wait for approval before continuing
            if self.mode == AgentMode::Attended {
                self.state = AgentState::AwaitingApproval;
                break;
            }
        }

        if results.len() == ticket.acceptance_criteria.len() {
            self.state = AgentState::Completed;
        }

        Ok(results)
    }

    /// Resumes execution after user approval (attended mode only)
    pub fn approve_and_continue(&mut self) {
        if self.state == AgentState::AwaitingApproval {
            self.state = AgentState::Executing;
        }
    }
}

/// Result of executing an acceptance criterion
#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub message: String,
    pub changes: Vec<FileChange>,
}

/// Represents a file change made during execution
#[derive(Debug)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
}

#[derive(Debug)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_executor_creation() {
        let config = AgentConfig::default();
        let executor = Executor::new(config, AgentMode::Attended);
        assert_eq!(executor.state(), &AgentState::Idle);
    }
}
