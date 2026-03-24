//! Enforcement mechanism validation for rules.
//!
//! Validates that:
//! 1. Every rule artifact has an `enforcement` array with ≥1 entry
//! 2. Every enforcement entry has a `mechanism` field
//! 3. Every referenced mechanism key is registered by an installed plugin
//!
//! Rules without enforcement are the architectural heart of the enforcement
//! engine: enforcement is the CONFIGURATION, the engine is the EXECUTOR.

use std::collections::HashSet;

use crate::graph::ArtifactGraph;
use crate::platform::EnforcementMechanism;
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

/// Check enforcement entries on all rule artifacts.
///
/// Produces warnings/errors for:
/// - Rules missing the `enforcement` field entirely
/// - Rules with an empty enforcement array
/// - Enforcement entries without a `mechanism` field
/// - Enforcement entries referencing an unregistered mechanism
pub fn check_enforcement_mechanisms(
    graph: &ArtifactGraph,
    registered_mechanisms: &[EnforcementMechanism],
    checks: &mut Vec<IntegrityCheck>,
) {
    let registered_keys: HashSet<&str> = registered_mechanisms
        .iter()
        .map(|m| m.key.as_str())
        .collect();

    for node in graph.nodes.values() {
        if node.artifact_type != "rule" {
            continue;
        }

        check_rule_enforcement(node, &registered_keys, checks);
    }
}

/// Validate the `enforcement` field on a single rule node.
fn check_rule_enforcement(
    node: &crate::graph::ArtifactNode,
    registered_keys: &HashSet<&str>,
    checks: &mut Vec<IntegrityCheck>,
) {
    let enforcement = node.frontmatter.get("enforcement");

    let Some(enforcement) = enforcement else {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::SchemaViolation,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: "Rule has no enforcement field — every rule must have ≥1 enforcement entry"
                .to_owned(),
            auto_fixable: false,
            fix_description: None,
        });
        return;
    };

    let Some(entries) = enforcement.as_array() else {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::SchemaViolation,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: format!(
                "Rule enforcement field must be an array, got {}",
                value_type_name(enforcement)
            ),
            auto_fixable: false,
            fix_description: None,
        });
        return;
    };

    if entries.is_empty() {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::SchemaViolation,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: "Rule has empty enforcement array — must have ≥1 entry".to_owned(),
            auto_fixable: false,
            fix_description: None,
        });
        return;
    }

    for (i, entry) in entries.iter().enumerate() {
        check_enforcement_entry(node, i, entry, registered_keys, checks);
    }
}

/// Validate a single enforcement entry object.
fn check_enforcement_entry(
    node: &crate::graph::ArtifactNode,
    i: usize,
    entry: &serde_json::Value,
    registered_keys: &HashSet<&str>,
    checks: &mut Vec<IntegrityCheck>,
) {
    let Some(obj) = entry.as_object() else {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::SchemaViolation,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: format!(
                "Enforcement entry [{i}] is a {}, not an object — needs migration to structured format",
                value_type_name(entry)
            ),
            auto_fixable: false,
            fix_description: None,
        });
        return;
    };

    let Some(mechanism) = obj.get("mechanism").and_then(serde_json::Value::as_str) else {
        if obj.contains_key("event") {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::SchemaViolation,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "Enforcement entry [{i}] uses legacy 'event' field — migrate to 'mechanism' field"
                ),
                auto_fixable: false,
                fix_description: None,
            });
        } else {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::SchemaViolation,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!("Enforcement entry [{i}] missing required 'mechanism' field"),
                auto_fixable: false,
                fix_description: None,
            });
        }
        return;
    };

    if !registered_keys.contains(mechanism) {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::SchemaViolation,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: format!(
                "Enforcement entry [{i}] references mechanism '{mechanism}' which is not registered by any installed plugin — enforcement degraded"
            ),
            auto_fixable: false,
            fix_description: None,
        });
    }
}

/// Get a human-readable type name for a JSON value.
fn value_type_name(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{ArtifactGraph, ArtifactNode};
    use std::collections::HashMap;

    fn make_rule(id: &str, enforcement: serde_json::Value) -> ArtifactNode {
        let mut frontmatter = serde_json::json!({
            "id": id,
            "status": "active",
        });
        frontmatter
            .as_object_mut()
            .expect("object")
            .insert("enforcement".to_owned(), enforcement);
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            artifact_type: "rule".to_owned(),
            path: format!(".orqa/process/rules/{id}.md"),
            title: id.to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter,
            body: None,
            references_out: vec![],
            references_in: vec![],
        }
    }

    fn make_graph(nodes: Vec<ArtifactNode>) -> ArtifactGraph {
        let mut graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        for node in nodes {
            graph.path_index.insert(node.path.clone(), node.id.clone());
            graph.nodes.insert(node.id.clone(), node);
        }
        graph
    }

    fn test_mechanisms() -> Vec<EnforcementMechanism> {
        vec![
            EnforcementMechanism {
                key: "behavioral".to_owned(),
                description: "Prompt injection".to_owned(),
                strength: 1,
            },
            EnforcementMechanism {
                key: "hook".to_owned(),
                description: "Lifecycle hooks".to_owned(),
                strength: 5,
            },
            EnforcementMechanism {
                key: "lint".to_owned(),
                description: "Linters".to_owned(),
                strength: 6,
            },
        ]
    }

    #[test]
    fn valid_enforcement_passes() {
        let rule = make_rule(
            "RULE-a1b2c3d4",
            serde_json::json!([
                { "mechanism": "behavioral", "message": "Always do X" },
                { "mechanism": "hook", "type": "PreAction", "action": "block" }
            ]),
        );
        let graph = make_graph(vec![rule]);
        let mut checks = Vec::new();
        check_enforcement_mechanisms(&graph, &test_mechanisms(), &mut checks);
        assert!(checks.is_empty(), "Expected no errors: {checks:?}");
    }

    #[test]
    fn missing_enforcement_field() {
        let mut node = ArtifactNode {
            id: "RULE-a1b2c3d4".to_owned(),
            project: None,
            artifact_type: "rule".to_owned(),
            path: ".orqa/process/rules/RULE-a1b2c3d4.md".to_owned(),
            title: "RULE-a1b2c3d4".to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({ "id": "RULE-a1b2c3d4", "status": "active" }),
            body: None,
            references_out: vec![],
            references_in: vec![],
        };
        // No enforcement field at all
        node.frontmatter
            .as_object_mut()
            .unwrap()
            .remove("enforcement");
        let graph = make_graph(vec![node]);
        let mut checks = Vec::new();
        check_enforcement_mechanisms(&graph, &test_mechanisms(), &mut checks);
        assert_eq!(checks.len(), 1);
        assert!(checks[0].message.contains("no enforcement field"));
    }

    #[test]
    fn string_enforcement_entry_warns() {
        let rule = make_rule("RULE-a1b2c3d4", serde_json::json!(["event: bash"]));
        let graph = make_graph(vec![rule]);
        let mut checks = Vec::new();
        check_enforcement_mechanisms(&graph, &test_mechanisms(), &mut checks);
        assert_eq!(checks.len(), 1);
        assert!(checks[0].message.contains("string"));
        assert_eq!(checks[0].severity, IntegritySeverity::Error);
    }

    #[test]
    fn unregistered_mechanism_warns() {
        let rule = make_rule(
            "RULE-a1b2c3d4",
            serde_json::json!([
                { "mechanism": "behavioral", "message": "Test" },
                { "mechanism": "unknown-mech", "action": "block" }
            ]),
        );
        let graph = make_graph(vec![rule]);
        let mut checks = Vec::new();
        check_enforcement_mechanisms(&graph, &test_mechanisms(), &mut checks);
        assert_eq!(checks.len(), 1);
        assert!(checks[0].message.contains("unknown-mech"));
        assert!(checks[0].message.contains("not registered"));
    }

    #[test]
    fn legacy_event_field_suggests_migration() {
        let rule = make_rule(
            "RULE-a1b2c3d4",
            serde_json::json!([
                { "event": "bash", "action": "block", "pattern": "--no-verify" }
            ]),
        );
        let graph = make_graph(vec![rule]);
        let mut checks = Vec::new();
        check_enforcement_mechanisms(&graph, &test_mechanisms(), &mut checks);
        assert_eq!(checks.len(), 1);
        assert!(checks[0].message.contains("legacy"));
        assert!(checks[0].message.contains("mechanism"));
    }

    #[test]
    fn non_rule_artifacts_are_skipped() {
        let mut node = ArtifactNode {
            id: "EPIC-a1b2c3d4".to_owned(),
            project: None,
            artifact_type: "epic".to_owned(),
            path: ".orqa/delivery/epics/EPIC-a1b2c3d4.md".to_owned(),
            title: "EPIC-a1b2c3d4".to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({ "id": "EPIC-a1b2c3d4", "status": "active" }),
            body: None,
            references_out: vec![],
            references_in: vec![],
        };
        // No enforcement field — but it's an epic, not a rule
        node.frontmatter
            .as_object_mut()
            .unwrap()
            .remove("enforcement");
        let graph = make_graph(vec![node]);
        let mut checks = Vec::new();
        check_enforcement_mechanisms(&graph, &test_mechanisms(), &mut checks);
        assert!(
            checks.is_empty(),
            "Epics should not be checked for enforcement"
        );
    }
}
