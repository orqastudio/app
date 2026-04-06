//! Delivery path completeness checks.

use crate::graph::{ArtifactGraph, ArtifactNode};
use crate::settings::{DeliveryParentConfig, DeliveryTypeConfig};
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity, ValidationContext};

/// Validate delivery artifacts against the `DeliveryConfig`.
pub fn check_delivery_paths(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph
        .nodes
        .values()
        .filter(|n| n.path.starts_with(".orqa/implementation/"))
    {
        let matched = ctx
            .delivery
            .types
            .iter()
            .find(|dt| node.path.starts_with(dt.path.trim_end_matches('/')));

        let Some(dtype) = matched else {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::DeliveryPathMismatch,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} is under '{}' but no delivery type in the config covers that path",
                    node.id, node.path
                ),
                auto_fixable: false,
                fix_description: Some(
                    "Add a delivery type entry to project.json covering this path, \
                     or move the artifact to a configured path"
                        .to_owned(),
                ),
            });
            continue;
        };

        // Check type mismatch.
        if node.artifact_type != dtype.key {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::DeliveryPathMismatch,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "{} is under path '{}' (delivery type '{}') but has artifact_type '{}'",
                    node.id, dtype.path, dtype.key, node.artifact_type
                ),
                auto_fixable: false,
                fix_description: Some(format!(
                    "Move the artifact to the correct directory, \
                     or update the delivery type key in project.json to '{}'",
                    node.artifact_type
                )),
            });
        }

        // Check parent relationship.
        check_delivery_node_parent(node, dtype, graph, checks);
    }
}

/// Validate the parent relationship for a delivery node.
fn check_delivery_node_parent(
    node: &ArtifactNode,
    dtype: &DeliveryTypeConfig,
    graph: &ArtifactGraph,
    checks: &mut Vec<IntegrityCheck>,
) {
    let Some(parent_cfg) = &dtype.parent else {
        return;
    };

    let parent_ref = node.references_out.iter().find(|r| {
        r.relationship_type.as_deref() == Some(&parent_cfg.relationship)
            && graph
                .nodes
                .get(&r.target_id)
                .is_some_and(|n| n.artifact_type == parent_cfg.parent_type)
    });

    let Some(parent_ref) = parent_ref else {
        push_missing_parent_check(node, dtype, parent_cfg, checks);
        return;
    };

    let Some(parent_node) = graph.nodes.get(&parent_ref.target_id) else {
        return; // broken ref, caught by check_broken_refs
    };

    if parent_node.artifact_type != parent_cfg.parent_type {
        push_wrong_parent_type_check(
            node,
            parent_cfg,
            parent_ref.target_id.as_str(),
            parent_node,
            checks,
        );
    }
}

fn push_missing_parent_check(
    node: &ArtifactNode,
    dtype: &DeliveryTypeConfig,
    parent_cfg: &DeliveryParentConfig,
    checks: &mut Vec<IntegrityCheck>,
) {
    checks.push(IntegrityCheck {
        category: IntegrityCategory::DeliveryPathMismatch,
        severity: IntegritySeverity::Error,
        artifact_id: node.id.clone(),
        message: format!(
            "{} (delivery type '{}') is missing required '{}' relationship to a {} artifact",
            node.id, dtype.key, parent_cfg.relationship, parent_cfg.parent_type
        ),
        auto_fixable: false,
        fix_description: Some(format!(
            "Add a '{}' relationship targeting a {} artifact",
            parent_cfg.relationship, parent_cfg.parent_type
        )),
    });
}

fn push_wrong_parent_type_check(
    node: &ArtifactNode,
    parent_cfg: &DeliveryParentConfig,
    parent_ref_target_id: &str,
    parent_node: &ArtifactNode,
    checks: &mut Vec<IntegrityCheck>,
) {
    checks.push(IntegrityCheck {
        category: IntegrityCategory::DeliveryPathMismatch,
        severity: IntegritySeverity::Error,
        artifact_id: node.id.clone(),
        message: format!(
            "{} has {} relationship to '{}' but {} is a '{}', expected '{}'",
            node.id,
            parent_cfg.relationship,
            parent_ref_target_id,
            parent_ref_target_id,
            parent_node.artifact_type,
            parent_cfg.parent_type
        ),
        auto_fixable: false,
        fix_description: Some(format!(
            "Update '{}' relationship to target a valid {} artifact",
            parent_cfg.relationship, parent_cfg.parent_type
        )),
    });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{DeliveryConfig, DeliveryParentConfig, DeliveryTypeConfig};
    use crate::types::{IntegrityCategory, ValidationContext};
    use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef};
    use std::collections::{HashMap, HashSet};

    fn make_node(id: &str, artifact_type: &str, path: &str) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: path.to_owned(),
            artifact_type: artifact_type.to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({"type": artifact_type, "status": "active"}),
            body: None,
            references_out: vec![],
            references_in: vec![],
        }
    }

    fn make_ref(source: &str, target: &str, rel_type: &str) -> ArtifactRef {
        ArtifactRef {
            target_id: target.to_owned(),
            field: "relationships".to_owned(),
            source_id: source.to_owned(),
            relationship_type: Some(rel_type.to_owned()),
        }
    }

    fn make_ctx_with_delivery(types: Vec<DeliveryTypeConfig>) -> ValidationContext {
        ValidationContext {
            relationships: vec![],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: DeliveryConfig { types },
            dependency_keys: HashSet::new(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        }
    }

    #[test]
    fn artifact_outside_configured_delivery_path_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let node = make_node(
            "TASK-A",
            "task",
            ".orqa/implementation/unknown-zone/TASK-A.md",
        );
        graph.nodes.insert("TASK-A".to_owned(), node);

        // Configure a delivery type that does NOT cover the node's path
        let ctx = make_ctx_with_delivery(vec![DeliveryTypeConfig {
            key: "task".to_owned(),
            label: "Tasks".to_owned(),
            path: ".orqa/implementation/tasks/".to_owned(),
            parent: None,
            gate_field: None,
        }]);

        let mut checks = vec![];
        check_delivery_paths(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::DeliveryPathMismatch);
        assert!(checks[0].message.contains("TASK-A"));
    }

    #[test]
    fn artifact_type_mismatch_for_delivery_path_is_flagged() {
        // Node is type "epic" but lives under the "task" delivery path
        let mut graph = ArtifactGraph::default();
        let node = make_node("EPIC-A", "epic", ".orqa/implementation/tasks/EPIC-A.md");
        graph.nodes.insert("EPIC-A".to_owned(), node);

        let ctx = make_ctx_with_delivery(vec![DeliveryTypeConfig {
            key: "task".to_owned(),
            label: "Tasks".to_owned(),
            path: ".orqa/implementation/tasks/".to_owned(),
            parent: None,
            gate_field: None,
        }]);

        let mut checks = vec![];
        check_delivery_paths(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::DeliveryPathMismatch);
        assert!(checks[0].message.contains("epic"));
    }

    #[test]
    fn artifact_with_valid_path_and_parent_produces_no_check() {
        let mut graph = ArtifactGraph::default();
        let mut task_node = make_node("TASK-A", "task", ".orqa/implementation/tasks/TASK-A.md");
        let epic_node = make_node("EPIC-B", "epic", ".orqa/epics/EPIC-B.md");

        // Task delivers to its epic parent
        task_node
            .references_out
            .push(make_ref("TASK-A", "EPIC-B", "delivers"));

        graph.nodes.insert("TASK-A".to_owned(), task_node);
        graph.nodes.insert("EPIC-B".to_owned(), epic_node);

        let ctx = make_ctx_with_delivery(vec![DeliveryTypeConfig {
            key: "task".to_owned(),
            label: "Tasks".to_owned(),
            path: ".orqa/implementation/tasks/".to_owned(),
            parent: Some(DeliveryParentConfig {
                parent_type: "epic".to_owned(),
                relationship: "delivers".to_owned(),
            }),
            gate_field: None,
        }]);

        let mut checks = vec![];
        check_delivery_paths(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn missing_parent_relationship_is_flagged() {
        let mut graph = ArtifactGraph::default();
        // Task has no outgoing "delivers" relationship
        let task_node = make_node("TASK-A", "task", ".orqa/implementation/tasks/TASK-A.md");
        graph.nodes.insert("TASK-A".to_owned(), task_node);

        let ctx = make_ctx_with_delivery(vec![DeliveryTypeConfig {
            key: "task".to_owned(),
            label: "Tasks".to_owned(),
            path: ".orqa/implementation/tasks/".to_owned(),
            parent: Some(DeliveryParentConfig {
                parent_type: "epic".to_owned(),
                relationship: "delivers".to_owned(),
            }),
            gate_field: None,
        }]);

        let mut checks = vec![];
        check_delivery_paths(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::DeliveryPathMismatch);
        assert!(checks[0].message.contains("delivers"));
    }

    #[test]
    fn artifacts_not_under_implementation_are_ignored() {
        // Nodes outside .orqa/implementation/ should not be checked at all
        let mut graph = ArtifactGraph::default();
        let node = make_node("EPIC-A", "epic", ".orqa/epics/EPIC-A.md");
        graph.nodes.insert("EPIC-A".to_owned(), node);

        let ctx = make_ctx_with_delivery(vec![]);
        let mut checks = vec![];
        check_delivery_paths(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn empty_delivery_config_flags_all_implementation_artifacts() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-A", "task", ".orqa/implementation/tasks/TASK-A.md");
        graph.nodes.insert("TASK-A".to_owned(), node);

        let ctx = make_ctx_with_delivery(vec![]); // no delivery types configured
        let mut checks = vec![];
        check_delivery_paths(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::DeliveryPathMismatch);
    }
}
