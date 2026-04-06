//! Unit tests for `engine/agent`.
//!
//! Tests verify the behavioral semantics of the agent type system:
//! - `BaseRole` serializes to the exact snake_case strings the schema expects
//! - `AgentSpec` round-trips through JSON faithfully
//! - `TaskAgent` preserves the spec it was built from
//! - The hub-spoke model constraints are reflected in role semantics

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::{AgentSpec, BaseRole, TaskAgent};

    // -------------------------------------------------------------------------
    // BaseRole serialization — must match the schema's expected keys
    // -------------------------------------------------------------------------

    #[test]
    fn base_role_serializes_to_snake_case() {
        // The CLAUDE.md role table uses snake_case keys — verify each variant.
        assert_eq!(
            serde_json::to_value(&BaseRole::Orchestrator).unwrap(),
            json!("orchestrator")
        );
        assert_eq!(
            serde_json::to_value(&BaseRole::Implementer).unwrap(),
            json!("implementer")
        );
        assert_eq!(
            serde_json::to_value(&BaseRole::Reviewer).unwrap(),
            json!("reviewer")
        );
        assert_eq!(
            serde_json::to_value(&BaseRole::Researcher).unwrap(),
            json!("researcher")
        );
        assert_eq!(
            serde_json::to_value(&BaseRole::Writer).unwrap(),
            json!("writer")
        );
        assert_eq!(
            serde_json::to_value(&BaseRole::Planner).unwrap(),
            json!("planner")
        );
        assert_eq!(
            serde_json::to_value(&BaseRole::Designer).unwrap(),
            json!("designer")
        );
        assert_eq!(
            serde_json::to_value(&BaseRole::GovernanceSteward).unwrap(),
            json!("governance_steward")
        );
    }

    #[test]
    fn base_role_deserializes_from_snake_case() {
        // Any string that the schema emits must round-trip back to the correct variant.
        let role: BaseRole = serde_json::from_value(json!("reviewer")).unwrap();
        assert_eq!(role, BaseRole::Reviewer);

        let role: BaseRole = serde_json::from_value(json!("governance_steward")).unwrap();
        assert_eq!(role, BaseRole::GovernanceSteward);
    }

    #[test]
    fn base_role_rejects_unknown_string() {
        // A string that does not match any variant must fail to deserialize.
        let result: Result<BaseRole, _> = serde_json::from_value(json!("unknown_role"));
        assert!(result.is_err(), "unknown role string must not deserialize");
    }

    // -------------------------------------------------------------------------
    // AgentSpec round-trip
    // -------------------------------------------------------------------------

    #[test]
    fn agent_spec_round_trips_through_json() {
        // A fully-populated AgentSpec must survive a serialize→deserialize cycle unchanged.
        let spec = AgentSpec {
            role: BaseRole::Implementer,
            tool_access: vec!["Read".to_owned(), "Edit".to_owned()],
            knowledge_refs: vec!["architecture/core".to_owned()],
            task_description: "Add unit tests to engine/graph".to_owned(),
        };

        let json_val = serde_json::to_value(&spec).expect("serialize");
        let recovered: AgentSpec = serde_json::from_value(json_val).expect("deserialize");

        assert_eq!(recovered.role, BaseRole::Implementer);
        assert_eq!(recovered.tool_access, vec!["Read", "Edit"]);
        assert_eq!(recovered.knowledge_refs, vec!["architecture/core"]);
        assert_eq!(recovered.task_description, "Add unit tests to engine/graph");
    }

    #[test]
    fn agent_spec_empty_tool_access_is_valid() {
        // An agent with no tools is valid — text-generation only (P5).
        let spec = AgentSpec {
            role: BaseRole::Reviewer,
            tool_access: vec![],
            knowledge_refs: vec![],
            task_description: "Verify acceptance criteria".to_owned(),
        };
        let json_val = serde_json::to_value(&spec).expect("serialize");
        let recovered: AgentSpec = serde_json::from_value(json_val).expect("deserialize");
        assert!(
            recovered.tool_access.is_empty(),
            "empty tool_access must round-trip as empty"
        );
    }

    // -------------------------------------------------------------------------
    // TaskAgent
    // -------------------------------------------------------------------------

    #[test]
    fn task_agent_preserves_spec_and_prompt() {
        // TaskAgent is the assembled artifact — it must hold the spec it was built from.
        let spec = AgentSpec {
            role: BaseRole::Writer,
            tool_access: vec!["Write".to_owned()],
            knowledge_refs: vec![],
            task_description: "Document the graph metrics module".to_owned(),
        };
        let agent = TaskAgent {
            spec: spec.clone(),
            generated_prompt: "You are a Writer. Your task: Document the graph metrics module."
                .to_owned(),
        };

        assert_eq!(agent.spec.role, BaseRole::Writer);
        assert!(
            agent.generated_prompt.contains("Document"),
            "generated_prompt must contain the task description content"
        );
    }

    #[test]
    fn task_agent_round_trips_through_json() {
        let spec = AgentSpec {
            role: BaseRole::Planner,
            tool_access: vec![],
            knowledge_refs: vec!["architecture/migration".to_owned()],
            task_description: "Plan phase 5".to_owned(),
        };
        let agent = TaskAgent {
            spec,
            generated_prompt: "System prompt for planner".to_owned(),
        };

        let json_val = serde_json::to_value(&agent).expect("serialize");
        let recovered: TaskAgent = serde_json::from_value(json_val).expect("deserialize");

        assert_eq!(recovered.spec.role, BaseRole::Planner);
        assert_eq!(recovered.generated_prompt, "System prompt for planner");
    }
}
