// orqa-agent: Base role taxonomy and agent specification types for the OrqaStudio platform.
//
// Provides the structural roles for the hub-spoke orchestration model (P6) and
// the agent specification types used by the prompt generation pipeline. Agents are
// ephemeral — one context window per task (P2). The engine defines structural roles;
// plugins specialise them for specific domains.

pub mod types;

pub use types::{AgentSpec, BaseRole, TaskAgent};
