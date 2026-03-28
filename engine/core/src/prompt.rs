// Prompt module for the orqa-engine crate.
//
// Re-exports the public prompt API from orqa-prompt so that consumers
// can import everything through orqa_engine::prompt.

pub mod builder {
    pub use orqa_prompt::builder::*;
}

pub mod knowledge {
    pub use orqa_prompt::knowledge::*;
}

pub mod session_title {
    pub use orqa_prompt::session_title::*;
}

pub use orqa_prompt::{
    build_system_prompt, collect_plugin_agent_definitions, list_knowledge_catalog,
    read_governance_file, read_rules, resolve_system_prompt,
};
