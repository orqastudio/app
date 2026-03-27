// Agent module for the orqa-engine crate.
//
// Provides the base role taxonomy and agent specification types used by the
// prompt generation pipeline. Agents are ephemeral — one context window per task (P2).
// The engine defines structural roles; plugins specialise them for specific domains.

pub mod types;

pub use types::{AgentSpec, BaseRole, TaskAgent};
