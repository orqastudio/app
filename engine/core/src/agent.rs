// Agent module for the orqa-engine crate.
//
// Re-exports the public agent API from orqa-agent so that consumers can
// import everything through orqa_engine::agent instead of depending on
// orqa-agent directly.

pub use orqa_agent::types;
pub use orqa_agent::{AgentSpec, BaseRole, TaskAgent};
