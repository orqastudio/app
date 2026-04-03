//! Status validity and parent-child consistency checks.

use std::collections::HashMap;

use crate::graph::ArtifactGraph;
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity, ValidationContext};

/// Mapping of commonly seen legacy status values to their canonical replacements.
const LEGACY_STATUS_MAP: &[(&str, &str)] = &[
    ("draft", "captured"),
    ("todo", "ready"),
    ("done", "completed"),
    ("in-progress", "active"),
    ("wip", "active"),
    ("complete", "completed"),
    ("open", "captured"),
    ("closed", "completed"),
    ("pending", "ready"),
    ("backlog", "captured"),
];

/// Suggest a canonical replacement for a legacy status value.
fn suggest_status_fix<'a>(invalid: &str, valid: &'a [String]) -> Option<&'a str> {
    if let Some(v) = valid.iter().find(|s| s.eq_ignore_ascii_case(invalid)) {
        return Some(v.as_str());
    }
    let canonical_hint = LEGACY_STATUS_MAP
        .iter()
        .find(|(old, _)| old.eq_ignore_ascii_case(invalid))
        .map(|(_, new)| *new);

    let hint = canonical_hint?;
    valid
        .iter()
        .find(|s| s.eq_ignore_ascii_case(hint))
        .map(String::as_str)
}

/// Check that every artifact's status is in the valid status list.
pub fn check_valid_statuses(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        let Some(status) = &node.status else {
            continue;
        };

        if ctx.valid_statuses.iter().any(|s| s == status) {
            continue;
        }

        let valid_list = ctx.valid_statuses.join(", ");
        let suggestion = suggest_status_fix(status, &ctx.valid_statuses);
        let (auto_fixable, fix_description) = if let Some(replacement) = suggestion {
            (
                true,
                Some(format!("Change status from '{status}' to '{replacement}'")),
            )
        } else {
            (
                false,
                Some(format!(
                    "Set status to one of the valid values: {valid_list}"
                )),
            )
        };

        checks.push(IntegrityCheck {
            category: IntegrityCategory::InvalidStatus,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: format!(
                "{} has invalid status '{}'. Valid values: {}",
                node.id, status, valid_list
            ),
            auto_fixable,
            fix_description,
        });
    }
}

/// Canonical status ordering for parent-child consistency checks.
///
/// Lateral states (`hold`, `blocked`) share the same ordinal as `active` (4)
/// so that a blocked/held child does not falsely appear "further along" than
/// its parent.  `recurring` is treated as equivalent to `completed` (6).
/// `surpassed` (7) is intentionally higher than `completed` (6) so that a
/// surpassed child of a completed parent is not flagged — that is a valid
/// end-state transition.
fn status_ordinal(status: &str) -> Option<usize> {
    match status {
        "captured" => Some(0),
        "exploring" => Some(1),
        "ready" => Some(2),
        "prioritised" => Some(3),
        "active" | "hold" | "blocked" => Some(4),
        "review" => Some(5),
        "completed" | "recurring" => Some(6),
        "surpassed" => Some(7),
        "archived" => Some(8),
        _ => None,
    }
}

/// Check parent-child status consistency using the delivery hierarchy.
pub fn check_parent_child_consistency(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    // Build a position map using the canonical ordering so that lateral states
    // (hold, blocked) are not treated as "further along" than active.
    let status_pos: HashMap<&str, usize> = ctx
        .valid_statuses
        .iter()
        .filter_map(|s| status_ordinal(s.as_str()).map(|pos| (s.as_str(), pos)))
        .collect();

    for dtype in &ctx.delivery.types {
        let Some(parent_cfg) = &dtype.parent else {
            continue;
        };
        check_child_type_consistency(
            graph,
            &dtype.key,
            &parent_cfg.relationship,
            &parent_cfg.parent_type,
            &status_pos,
            checks,
        );
    }
}

/// Check all artifacts of a child type for parent-child status inconsistencies.
fn check_child_type_consistency(
    graph: &ArtifactGraph,
    child_type: &str,
    parent_relationship: &str,
    parent_label: &str,
    status_pos: &HashMap<&str, usize>,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == child_type)
    {
        let Some(child_status) = node.status.as_deref() else {
            continue;
        };
        let Some(&child_pos) = status_pos.get(child_status) else {
            continue;
        };
        let parent_ref = node
            .references_out
            .iter()
            .find(|r| r.relationship_type.as_deref() == Some(parent_relationship));
        let Some(parent_ref) = parent_ref else {
            continue;
        };
        let Some(parent) = graph.nodes.get(&parent_ref.target_id) else {
            continue;
        };
        let Some(parent_status) = &parent.status else {
            continue;
        };
        let Some(&parent_pos) = status_pos.get(parent_status.as_str()) else {
            continue;
        };
        // Only flag when parent is in a very early state (before active, ordinal < 4)
        // and child is in a late state (completed or later, ordinal >= 6).
        // An active epic with completed tasks is normal workflow — the epic has more tasks.
        if child_pos > parent_pos && parent_pos < 4 && child_pos >= 6 {
            checks.push(IntegrityCheck {
                artifact_id: node.id.clone(),
                category: IntegrityCategory::ParentChildInconsistency,
                severity: IntegritySeverity::Error,
                message: format!(
                    "{} is '{}' but {} {} is '{}' \u{2014} child is further along than parent",
                    node.id, child_status, parent_label, parent_ref.target_id, parent_status,
                ),
                auto_fixable: false,
                fix_description: Some(format!(
                    "Either advance {} to at least '{}', or move {} to a different {}",
                    parent_ref.target_id, child_status, node.id, parent_label,
                )),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{DeliveryConfig, DeliveryParentConfig, DeliveryTypeConfig};
    use crate::types::ValidationContext;
    use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef};
    use std::collections::{HashMap, HashSet};

    fn make_node(id: &str, artifact_type: &str, status: &str) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: format!(".orqa/test/{id}.md"),
            artifact_type: artifact_type.to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some(status.to_owned()),
            priority: None,
            frontmatter: serde_json::json!({"type": artifact_type, "status": status}),
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

    fn make_ctx_with_statuses(statuses: &[&str]) -> ValidationContext {
        ValidationContext {
            relationships: vec![],
            inverse_map: HashMap::new(),
            valid_statuses: statuses.iter().map(|s| s.to_string()).collect(),
            delivery: DeliveryConfig { types: vec![] },
            dependency_keys: HashSet::new(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        }
    }

    fn make_ctx_with_delivery(
        statuses: &[&str],
        delivery_types: Vec<DeliveryTypeConfig>,
    ) -> ValidationContext {
        ValidationContext {
            relationships: vec![],
            inverse_map: HashMap::new(),
            valid_statuses: statuses.iter().map(|s| s.to_string()).collect(),
            delivery: DeliveryConfig { types: delivery_types },
            dependency_keys: HashSet::new(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        }
    }

    // --- check_valid_statuses ---

    #[test]
    fn invalid_status_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-A", "task", "unknown-status");
        graph.nodes.insert("TASK-A".to_owned(), node);

        let ctx = make_ctx_with_statuses(&["active", "completed"]);
        let mut checks = vec![];
        check_valid_statuses(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::InvalidStatus);
        assert!(checks[0].message.contains("unknown-status"));
    }

    #[test]
    fn valid_status_produces_no_check() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-A", "task", "active");
        graph.nodes.insert("TASK-A".to_owned(), node);

        let ctx = make_ctx_with_statuses(&["active", "completed"]);
        let mut checks = vec![];
        check_valid_statuses(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn legacy_status_maps_to_canonical_replacement() {
        // "draft" is a legacy status; "captured" is its canonical replacement.
        let valid = vec!["captured".to_string(), "active".to_string()];
        let suggestion = suggest_status_fix("draft", &valid);
        assert_eq!(suggestion, Some("captured"));
    }

    #[test]
    fn case_insensitive_status_match_is_suggested() {
        // "Active" should match "active" in the valid list.
        let valid = vec!["active".to_string(), "completed".to_string()];
        let suggestion = suggest_status_fix("Active", &valid);
        assert_eq!(suggestion, Some("active"));
    }

    #[test]
    fn node_with_no_status_is_skipped_for_status_check() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task", "active");
        node.status = None;
        graph.nodes.insert("TASK-A".to_owned(), node);

        let ctx = make_ctx_with_statuses(&["active"]);
        let mut checks = vec![];
        check_valid_statuses(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_parent_child_consistency ---

    #[test]
    fn completed_task_with_captured_epic_is_flagged() {
        // Task is "completed" (ordinal 6) but its parent epic is "captured" (ordinal 0).
        let mut graph = ArtifactGraph::default();
        let mut task = make_node("TASK-A", "task", "completed");
        let epic = make_node("EPIC-B", "epic", "captured");
        task.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), task);
        graph.nodes.insert("EPIC-B".to_owned(), epic);

        let ctx = make_ctx_with_delivery(
            &["captured", "active", "completed"],
            vec![DeliveryTypeConfig {
                key: "task".to_owned(),
                label: "Tasks".to_owned(),
                path: ".orqa/implementation/tasks/".to_owned(),
                parent: Some(DeliveryParentConfig {
                    parent_type: "epic".to_owned(),
                    relationship: "delivers".to_owned(),
                }),
                gate_field: None,
            }],
        );

        let mut checks = vec![];
        check_parent_child_consistency(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::ParentChildInconsistency);
        assert!(checks[0].message.contains("TASK-A"));
    }

    #[test]
    fn completed_task_with_active_epic_is_not_flagged() {
        // An active epic with a completed task is normal — the epic has more work to do.
        let mut graph = ArtifactGraph::default();
        let mut task = make_node("TASK-A", "task", "completed");
        let epic = make_node("EPIC-B", "epic", "active");
        task.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), task);
        graph.nodes.insert("EPIC-B".to_owned(), epic);

        let ctx = make_ctx_with_delivery(
            &["captured", "active", "completed"],
            vec![DeliveryTypeConfig {
                key: "task".to_owned(),
                label: "Tasks".to_owned(),
                path: ".orqa/implementation/tasks/".to_owned(),
                parent: Some(DeliveryParentConfig {
                    parent_type: "epic".to_owned(),
                    relationship: "delivers".to_owned(),
                }),
                gate_field: None,
            }],
        );

        let mut checks = vec![];
        check_parent_child_consistency(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn delivery_type_without_parent_config_skips_consistency_check() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-A", "task", "completed");
        graph.nodes.insert("TASK-A".to_owned(), node);

        let ctx = make_ctx_with_delivery(
            &["captured", "active", "completed"],
            vec![DeliveryTypeConfig {
                key: "task".to_owned(),
                label: "Tasks".to_owned(),
                path: ".orqa/implementation/tasks/".to_owned(),
                parent: None,
                gate_field: None,
            }],
        );

        let mut checks = vec![];
        check_parent_child_consistency(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }
}
