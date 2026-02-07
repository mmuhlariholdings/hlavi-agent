//! # Hlavi Agent
//!
//! AI agent orchestration for automating kanban ticket completion.
//!
//! This crate provides the agent execution framework that can:
//! - Plan tasks by generating acceptance criteria
//! - Execute tasks in attended or unattended mode
//! - Handle rejections and retries
//! - Integrate with various AI model providers

pub mod config;
pub mod error;
pub mod executor;
pub mod planner;

pub use config::AgentConfig;
pub use error::{AgentError, Result};

/// Agent execution mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentMode {
    /// Agent waits for user approval before each step
    Attended,
    /// Agent completes all steps automatically
    Unattended,
}

/// Agent state during execution
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentState {
    Idle,
    Planning,
    Executing,
    AwaitingApproval,
    Completed,
    Failed,
}
