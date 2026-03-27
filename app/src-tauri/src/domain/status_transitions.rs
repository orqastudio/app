// Status transition evaluation — ProposedTransition re-exported from orqa-engine.
//
// This module evaluates the artifact graph against status definitions loaded from
// project settings and returns proposed status transitions. The ProposedTransition
// type is defined in the engine; business logic lives here in the app.

use crate::domain::artifact_graph::ArtifactGraph;
use crate::domain::project_settings::StatusDefinition;

pub use orqa_engine::types::workflow::ProposedTransition;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Evaluate the artifact graph and return all transitions that should occur.
///
/// The function is pure — it only reads `graph` and `statuses`, and never
/// modifies either. Each `StatusDefinition` carries `auto_rules` that declare
/// which named conditions apply to which status. The condition-evaluation
/// logic lives here; the config drives which conditions are active.
///
/// When `statuses` is empty the function returns an empty list — without
/// config there are no rules to evaluate.
pub fn evaluate_transitions(
    graph: &ArtifactGraph,
    statuses: &[StatusDefinition],
) -> Vec<ProposedTransition> {
    if statuses.is_empty() {
        return Vec::new();
    }

    let mut proposals: Vec<ProposedTransition> = Vec::new();

    for node in graph.nodes.values() {
        let Some(current_status) = node.status.as_deref() else {
            continue;
        };

        // Find the status definition for this node's current status.
        let Some(status_def) = statuses.iter().find(|s| s.key == current_status) else {
            continue;
        };

        for rule in &status_def.auto_rules {
            if let Some(proposal) =
                evaluate_condition(graph, node, current_status, &rule.condition, &rule.target)
            {
                proposals.push(proposal);
            }
        }
    }

    proposals
}

// ---------------------------------------------------------------------------
// Condition evaluation
// ---------------------------------------------------------------------------

/// Evaluate a single named condition for a given artifact node.
///
/// Returns `Some(ProposedTransition)` when the condition is met, `None` otherwise.
///
/// ### Supported conditions
///
/// | Condition | Meaning | auto_apply |
/// |-----------|---------|-----------|
/// | `all-children-completed` | All child artifacts (tasks linked by `epic` field or `delivers` edge) are completed | `false` |
/// | `all-p1-children-completed` | All P1 child epics (linked by `milestone` field) are completed | `false` |
/// | `dependency-blocked` | At least one `depends-on` item is not completed | `true` |
/// | `dependencies-met` | All `depends-on` items are completed; node must currently be `blocked` | `true` |
/// | `recurrence-threshold` | `recurrence` frontmatter field is ≥ 2 | `false` |
fn evaluate_condition(
    graph: &ArtifactGraph,
    node: &crate::domain::artifact_graph::ArtifactNode,
    current_status: &str,
    condition: &str,
    target: &str,
) -> Option<ProposedTransition> {
    match condition {
        "all-children-completed" => {
            check_all_children_completed(graph, node, current_status, target)
        }
        "all-p1-children-completed" => {
            check_all_p1_children_completed(graph, node, current_status, target)
        }
        "dependency-blocked" => check_dependency_blocked(graph, node, current_status, target),
        "dependencies-met" => check_dependencies_met(graph, node, current_status, target),
        "recurrence-threshold" => check_recurrence_threshold(node, current_status, target),
        other => {
            tracing::debug!(
                "[transitions] unknown condition '{}' on node '{}' — skipping",
                other,
                node.id
            );
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Condition: all-children-completed
// ---------------------------------------------------------------------------

/// Proposes `target` when all child artifacts (tasks referencing this node via
/// `delivers` relationship edge) are completed.
fn check_all_children_completed(
    graph: &ArtifactGraph,
    node: &crate::domain::artifact_graph::ArtifactNode,
    current_status: &str,
    target: &str,
) -> Option<ProposedTransition> {
    // Collect all tasks that reference this node via a `delivers` relationship.
    let children: Vec<&crate::domain::artifact_graph::ArtifactNode> = graph
        .nodes
        .values()
        .filter(|n| {
            if n.artifact_type != "task" {
                return false;
            }
            n.references_out.iter().any(|r| {
                r.relationship_type.as_deref() == Some("delivers") && r.target_id == node.id
            })
        })
        .collect();

    if children.is_empty() {
        return None;
    }

    let all_completed = children
        .iter()
        .all(|t| t.status.as_deref() == Some("completed") || t.status.as_deref() == Some("done"));

    if all_completed {
        Some(ProposedTransition {
            artifact_id: node.id.clone(),
            artifact_path: node.path.clone(),
            current_status: current_status.to_owned(),
            proposed_status: target.to_owned(),
            reason: format!("All {} related task(s) are completed", children.len()),
            auto_apply: false,
        })
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Condition: all-p1-children-completed
// ---------------------------------------------------------------------------

/// Proposes `target` when all P1 child epics (referencing this node via
/// `delivers` relationship) are completed.
fn check_all_p1_children_completed(
    graph: &ArtifactGraph,
    node: &crate::domain::artifact_graph::ArtifactNode,
    current_status: &str,
    target: &str,
) -> Option<ProposedTransition> {
    let p1_epics: Vec<&crate::domain::artifact_graph::ArtifactNode> = graph
        .nodes
        .values()
        .filter(|n| {
            if n.artifact_type != "epic" {
                return false;
            }
            // Epic must have a `delivers` relationship to this milestone.
            let targets_this = n.references_out.iter().any(|r| {
                r.relationship_type.as_deref() == Some("delivers") && r.target_id == node.id
            });
            if !targets_this {
                return false;
            }
            n.frontmatter
                .get("priority")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                == "P1"
        })
        .collect();

    if p1_epics.is_empty() {
        return None;
    }

    let all_completed = p1_epics
        .iter()
        .all(|e| e.status.as_deref() == Some("completed") || e.status.as_deref() == Some("done"));

    if all_completed {
        Some(ProposedTransition {
            artifact_id: node.id.clone(),
            artifact_path: node.path.clone(),
            current_status: current_status.to_owned(),
            proposed_status: target.to_owned(),
            reason: format!(
                "All {} P1 epic(s) for this milestone are completed",
                p1_epics.len()
            ),
            auto_apply: false,
        })
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Condition: dependency-blocked
// ---------------------------------------------------------------------------

/// Proposes `target` when at least one `depends-on` relationship target is not completed.
/// Auto-apply is `true`.
fn check_dependency_blocked(
    graph: &ArtifactGraph,
    node: &crate::domain::artifact_graph::ArtifactNode,
    current_status: &str,
    target: &str,
) -> Option<ProposedTransition> {
    let depends_on = collect_depends_on_from_relationships(node);
    if depends_on.is_empty() {
        return None;
    }

    let blocking: Vec<&str> = depends_on
        .iter()
        .filter(|dep_id| match graph.nodes.get(dep_id.as_str()) {
            Some(dep) => {
                dep.status.as_deref() != Some("completed") && dep.status.as_deref() != Some("done")
            }
            None => true,
        })
        .map(String::as_str)
        .collect();

    if blocking.is_empty() {
        return None;
    }

    Some(ProposedTransition {
        artifact_id: node.id.clone(),
        artifact_path: node.path.clone(),
        current_status: current_status.to_owned(),
        proposed_status: target.to_owned(),
        reason: format!("Dependency {} not completed", blocking.join(", ")),
        auto_apply: true,
    })
}

// ---------------------------------------------------------------------------
// Condition: dependencies-met
// ---------------------------------------------------------------------------

/// Proposes `target` when all `depends-on` relationship targets are completed.
/// Only fires when the node is currently `blocked`.
/// Auto-apply is `true`.
fn check_dependencies_met(
    graph: &ArtifactGraph,
    node: &crate::domain::artifact_graph::ArtifactNode,
    current_status: &str,
    target: &str,
) -> Option<ProposedTransition> {
    if current_status != "blocked" {
        return None;
    }

    let depends_on = collect_depends_on_from_relationships(node);
    if depends_on.is_empty() {
        return None;
    }

    let all_complete = depends_on.iter().all(|dep_id| {
        graph
            .nodes
            .get(dep_id.as_str())
            .and_then(|n| n.status.as_deref())
            .is_some_and(|s| s == "completed" || s == "done")
    });

    if all_complete {
        Some(ProposedTransition {
            artifact_id: node.id.clone(),
            artifact_path: node.path.clone(),
            current_status: current_status.to_owned(),
            proposed_status: target.to_owned(),
            reason: format!(
                "All {} dependency/dependencies are now completed",
                depends_on.len()
            ),
            auto_apply: true,
        })
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Condition: recurrence-threshold
// ---------------------------------------------------------------------------

/// Proposes `target` when the artifact's `recurrence` frontmatter field is ≥ 2.
fn check_recurrence_threshold(
    node: &crate::domain::artifact_graph::ArtifactNode,
    current_status: &str,
    target: &str,
) -> Option<ProposedTransition> {
    let recurrence = node
        .frontmatter
        .get("recurrence")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);

    if recurrence >= 2 {
        Some(ProposedTransition {
            artifact_id: node.id.clone(),
            artifact_path: node.path.clone(),
            current_status: current_status.to_owned(),
            proposed_status: target.to_owned(),
            reason: format!(
                "Lesson recurrence is {recurrence} (threshold: 2) — ready for promotion"
            ),
            auto_apply: false,
        })
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Extract dependency target IDs from a node's `depends-on` relationship edges.
fn collect_depends_on_from_relationships(
    node: &crate::domain::artifact_graph::ArtifactNode,
) -> Vec<String> {
    node.references_out
        .iter()
        .filter(|r| r.relationship_type.as_deref() == Some("depends-on"))
        .map(|r| r.target_id.clone())
        .collect()
}

// ---------------------------------------------------------------------------
// Condition: child-exploring / child-active (parent follows children)
// ---------------------------------------------------------------------------

/// Proposes `target` when any child artifact has a status matching `child_status`.
///

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::domain::artifact_graph::{ArtifactNode, ArtifactRef};
    use crate::domain::project_settings::{StatusAutoRule, StatusDefinition};

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_node(
        id: &str,
        artifact_type: &str,
        status: &str,
        frontmatter: serde_json::Value,
    ) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: format!(".orqa/{id}.md"),
            artifact_type: artifact_type.to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some(status.to_owned()),
            priority: None,
            frontmatter,
            body: None,
            references_out: Vec::new(),
            references_in: Vec::new(),
        }
    }

    /// Create a node with a `delivers` relationship to the given target.
    fn make_child(id: &str, artifact_type: &str, status: &str, delivers_to: &str) -> ArtifactNode {
        let mut node = make_node(id, artifact_type, status, serde_json::json!({}));
        node.references_out.push(ArtifactRef {
            target_id: delivers_to.to_owned(),
            field: "relationships".to_owned(),
            source_id: id.to_owned(),
            relationship_type: Some("delivers".to_owned()),
        });
        node
    }

    /// Create a node with a `depends-on` relationship to the given target.
    fn make_dependent(
        id: &str,
        artifact_type: &str,
        status: &str,
        depends_on: &str,
    ) -> ArtifactNode {
        let mut node = make_node(id, artifact_type, status, serde_json::json!({}));
        node.references_out.push(ArtifactRef {
            target_id: depends_on.to_owned(),
            field: "relationships".to_owned(),
            source_id: id.to_owned(),
            relationship_type: Some("depends-on".to_owned()),
        });
        node
    }

    /// Create a node with a `delivers` relationship and priority set.
    fn make_child_with_priority(
        id: &str,
        artifact_type: &str,
        status: &str,
        delivers_to: &str,
        priority: &str,
    ) -> ArtifactNode {
        let mut node = make_node(
            id,
            artifact_type,
            status,
            serde_json::json!({"priority": priority}),
        );
        node.references_out.push(ArtifactRef {
            target_id: delivers_to.to_owned(),
            field: "relationships".to_owned(),
            source_id: id.to_owned(),
            relationship_type: Some("delivers".to_owned()),
        });
        node
    }

    fn make_graph(nodes: Vec<ArtifactNode>) -> ArtifactGraph {
        let mut map = HashMap::new();
        for n in nodes {
            map.insert(n.id.clone(), n);
        }
        ArtifactGraph {
            nodes: map,
            path_index: HashMap::new(),
        }
    }

    fn status(key: &str, auto_rules: Vec<(&str, &str)>) -> StatusDefinition {
        StatusDefinition {
            key: key.to_owned(),
            label: key.to_owned(),
            icon: "circle".to_owned(),
            spin: false,
            transitions: Vec::new(),
            auto_rules: auto_rules
                .into_iter()
                .map(|(condition, target)| StatusAutoRule {
                    condition: condition.to_owned(),
                    target: target.to_owned(),
                })
                .collect(),
        }
    }

    // -----------------------------------------------------------------------
    // Rule 1 — epic to review
    // -----------------------------------------------------------------------

    #[test]
    fn rule1_proposes_epic_review_when_all_tasks_done() {
        let epic = make_node("EPIC-001", "epic", "in-progress", serde_json::json!({}));
        let task1 = make_child("TASK-001", "task", "completed", "EPIC-001");
        let task2 = make_child("TASK-002", "task", "done", "EPIC-001");
        let graph = make_graph(vec![epic, task1, task2]);
        let statuses = vec![
            status("in-progress", vec![("all-children-completed", "review")]),
            status("completed", vec![]),
            status("done", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        let epic_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "EPIC-001")
            .collect();

        assert_eq!(epic_proposals.len(), 1);
        assert_eq!(epic_proposals[0].proposed_status, "review");
        assert!(!epic_proposals[0].auto_apply);
    }

    #[test]
    fn rule1_no_proposal_when_task_still_active() {
        let epic = make_node("EPIC-002", "epic", "in-progress", serde_json::json!({}));
        let task1 = make_child("TASK-003", "task", "completed", "EPIC-002");
        let task2 = make_child("TASK-004", "task", "in-progress", "EPIC-002");
        let graph = make_graph(vec![epic, task1, task2]);
        let statuses = vec![
            status("in-progress", vec![("all-children-completed", "review")]),
            status("completed", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals
            .iter()
            .all(|p| p.artifact_id != "EPIC-002" || p.proposed_status != "review"));
    }

    #[test]
    fn rule1_no_proposal_when_no_related_tasks() {
        let epic = make_node("EPIC-003", "epic", "active", serde_json::json!({}));
        let graph = make_graph(vec![epic]);
        let statuses = vec![status("active", vec![("all-children-completed", "review")])];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "EPIC-003"));
    }

    #[test]
    fn rule1_accepts_delivers_relationship() {
        let epic = make_node("EPIC-010", "epic", "active", serde_json::json!({}));
        let mut task = make_node("TASK-010", "task", "completed", serde_json::json!({}));
        task.references_out.push(ArtifactRef {
            target_id: "EPIC-010".to_owned(),
            field: "relationships".to_owned(),
            source_id: "TASK-010".to_owned(),
            relationship_type: Some("delivers".to_owned()),
        });
        let graph = make_graph(vec![epic, task]);
        let statuses = vec![
            status("active", vec![("all-children-completed", "review")]),
            status("completed", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        let epic_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "EPIC-010")
            .collect();

        assert_eq!(epic_proposals.len(), 1);
        assert_eq!(epic_proposals[0].proposed_status, "review");
    }

    // -----------------------------------------------------------------------
    // Rule 2 — milestone to review
    // -----------------------------------------------------------------------

    #[test]
    fn rule2_proposes_milestone_review_when_all_p1_epics_done() {
        let ms = make_node("MS-001", "milestone", "active", serde_json::json!({}));
        let epic1 = make_child_with_priority("EPIC-011", "epic", "completed", "MS-001", "P1");
        let epic2 = make_child_with_priority("EPIC-012", "epic", "completed", "MS-001", "P1");
        let graph = make_graph(vec![ms, epic1, epic2]);
        let statuses = vec![
            status("active", vec![("all-p1-children-completed", "review")]),
            status("completed", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        let ms_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "MS-001")
            .collect();

        assert_eq!(ms_proposals.len(), 1);
        assert_eq!(ms_proposals[0].proposed_status, "review");
        assert!(!ms_proposals[0].auto_apply);
    }

    #[test]
    fn rule2_no_proposal_when_p1_epic_not_done() {
        let ms = make_node("MS-002", "milestone", "active", serde_json::json!({}));
        let epic1 = make_child_with_priority("EPIC-013", "epic", "completed", "MS-002", "P1");
        let epic2 = make_child_with_priority("EPIC-014", "epic", "in-progress", "MS-002", "P1");
        let graph = make_graph(vec![ms, epic1, epic2]);
        let statuses = vec![
            status("active", vec![("all-p1-children-completed", "review")]),
            status("completed", vec![]),
            status("in-progress", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "MS-002"));
    }

    #[test]
    fn rule2_ignores_p2_epics_when_checking_completion() {
        let ms = make_node("MS-003", "milestone", "active", serde_json::json!({}));
        let epic_p1 = make_child_with_priority("EPIC-015", "epic", "completed", "MS-003", "P1");
        // P2 epic is still in-progress — should not block milestone review.
        let epic_p2 = make_child_with_priority("EPIC-016", "epic", "in-progress", "MS-003", "P2");
        let graph = make_graph(vec![ms, epic_p1, epic_p2]);
        let statuses = vec![
            status("active", vec![("all-p1-children-completed", "review")]),
            status("completed", vec![]),
            status("in-progress", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        let ms_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "MS-003")
            .collect();

        assert_eq!(ms_proposals.len(), 1);
        assert_eq!(ms_proposals[0].proposed_status, "review");
    }

    #[test]
    fn rule2_no_proposal_when_no_p1_epics() {
        let ms = make_node("MS-004", "milestone", "active", serde_json::json!({}));
        let epic_p2 = make_child_with_priority("EPIC-017", "epic", "completed", "MS-004", "P2");
        let graph = make_graph(vec![ms, epic_p2]);
        let statuses = vec![
            status("active", vec![("all-p1-children-completed", "review")]),
            status("completed", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "MS-004"));
    }

    // -----------------------------------------------------------------------
    // Rule 3 — task to blocked
    // -----------------------------------------------------------------------

    #[test]
    fn rule3_proposes_blocked_when_dependency_not_done() {
        let dep = make_node("TASK-020", "task", "in-progress", serde_json::json!({}));
        let task = make_dependent("TASK-021", "task", "ready", "TASK-020");
        let graph = make_graph(vec![dep, task]);
        let statuses = vec![
            status("ready", vec![("dependency-blocked", "blocked")]),
            status("in-progress", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        let blocked: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "TASK-021")
            .collect();

        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].proposed_status, "blocked");
        assert!(blocked[0].auto_apply);
    }

    #[test]
    fn rule3_no_proposal_when_all_dependencies_done() {
        let dep = make_node("TASK-022", "task", "completed", serde_json::json!({}));
        let task = make_dependent("TASK-023", "task", "ready", "TASK-022");
        let graph = make_graph(vec![dep, task]);
        let statuses = vec![
            status("ready", vec![("dependency-blocked", "blocked")]),
            status("completed", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-023"));
    }

    #[test]
    fn rule3_no_proposal_when_no_dependencies() {
        let task = make_node("TASK-024", "task", "ready", serde_json::json!({}));
        let graph = make_graph(vec![task]);
        let statuses = vec![status("ready", vec![("dependency-blocked", "blocked")])];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-024"));
    }

    #[test]
    fn rule3_handles_depends_on_relationship() {
        let dep = make_node("TASK-025", "task", "in-progress", serde_json::json!({}));
        let task = make_dependent("TASK-026", "task", "todo", "TASK-025");
        let graph = make_graph(vec![dep, task]);
        let statuses = vec![
            status("todo", vec![("dependency-blocked", "blocked")]),
            status("in-progress", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        let blocked: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "TASK-026")
            .collect();

        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].proposed_status, "blocked");
    }

    // -----------------------------------------------------------------------
    // Rule 4 — task unblocked
    // -----------------------------------------------------------------------

    #[test]
    fn rule4_proposes_ready_when_all_deps_complete() {
        let dep = make_node("TASK-030", "task", "completed", serde_json::json!({}));
        let task = make_dependent("TASK-031", "task", "blocked", "TASK-030");
        let graph = make_graph(vec![dep, task]);
        let statuses = vec![
            status("blocked", vec![("dependencies-met", "ready")]),
            status("completed", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        let unblocked: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "TASK-031")
            .collect();

        assert_eq!(unblocked.len(), 1);
        assert_eq!(unblocked[0].proposed_status, "ready");
        assert!(unblocked[0].auto_apply);
    }

    #[test]
    fn rule4_no_proposal_when_dep_still_incomplete() {
        let dep = make_node("TASK-032", "task", "in-progress", serde_json::json!({}));
        let task = make_dependent("TASK-033", "task", "blocked", "TASK-032");
        let graph = make_graph(vec![dep, task]);
        let statuses = vec![
            status("blocked", vec![("dependencies-met", "ready")]),
            status("in-progress", vec![]),
        ];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-033"));
    }

    #[test]
    fn rule4_no_proposal_for_blocked_task_with_no_deps() {
        let task = make_node("TASK-034", "task", "blocked", serde_json::json!({}));
        let graph = make_graph(vec![task]);
        let statuses = vec![status("blocked", vec![("dependencies-met", "ready")])];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "TASK-034"));
    }

    // -----------------------------------------------------------------------
    // Rule 5 — lesson to review
    // -----------------------------------------------------------------------

    #[test]
    fn rule5_proposes_review_when_recurrence_at_threshold() {
        let lesson = make_node(
            "IMPL-001",
            "lesson",
            "active",
            serde_json::json!({ "recurrence": 2 }),
        );
        let graph = make_graph(vec![lesson]);
        let statuses = vec![status("active", vec![("recurrence-threshold", "review")])];

        let proposals = evaluate_transitions(&graph, &statuses);
        let lesson_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "IMPL-001")
            .collect();

        assert_eq!(lesson_proposals.len(), 1);
        assert_eq!(lesson_proposals[0].proposed_status, "review");
        assert!(!lesson_proposals[0].auto_apply);
    }

    #[test]
    fn rule5_proposes_review_when_recurrence_exceeds_threshold() {
        let lesson = make_node(
            "IMPL-002",
            "lesson",
            "recurring",
            serde_json::json!({ "recurrence": 5 }),
        );
        let graph = make_graph(vec![lesson]);
        let statuses = vec![status(
            "recurring",
            vec![("recurrence-threshold", "review")],
        )];

        let proposals = evaluate_transitions(&graph, &statuses);
        let lesson_proposals: Vec<_> = proposals
            .iter()
            .filter(|p| p.artifact_id == "IMPL-002")
            .collect();

        assert_eq!(lesson_proposals.len(), 1);
        assert_eq!(lesson_proposals[0].proposed_status, "review");
    }

    #[test]
    fn rule5_no_proposal_when_recurrence_below_threshold() {
        let lesson = make_node(
            "IMPL-003",
            "lesson",
            "active",
            serde_json::json!({ "recurrence": 1 }),
        );
        let graph = make_graph(vec![lesson]);
        let statuses = vec![status("active", vec![("recurrence-threshold", "review")])];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "IMPL-003"));
    }

    #[test]
    fn rule5_no_proposal_when_lesson_already_promoted() {
        let lesson = make_node(
            "IMPL-004",
            "lesson",
            "promoted",
            serde_json::json!({ "recurrence": 3 }),
        );
        let graph = make_graph(vec![lesson]);
        // "promoted" has no auto_rules — so no proposal even though recurrence is high
        let statuses = vec![status("promoted", vec![])];

        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.iter().all(|p| p.artifact_id != "IMPL-004"));
    }

    // -----------------------------------------------------------------------
    // Empty graph / empty statuses
    // -----------------------------------------------------------------------

    #[test]
    fn empty_graph_returns_no_proposals() {
        let graph = make_graph(vec![]);
        let statuses = vec![status("active", vec![("all-children-completed", "review")])];
        let proposals = evaluate_transitions(&graph, &statuses);
        assert!(proposals.is_empty());
    }

    #[test]
    fn empty_statuses_returns_no_proposals() {
        let epic = make_node("EPIC-099", "epic", "in-progress", serde_json::json!({}));
        let graph = make_graph(vec![epic]);
        let proposals = evaluate_transitions(&graph, &[]);
        assert!(proposals.is_empty());
    }

    #[test]
    fn unknown_condition_is_skipped_without_panic() {
        let node = make_node("TASK-099", "task", "active", serde_json::json!({}));
        let graph = make_graph(vec![node]);
        let statuses = vec![status("active", vec![("not-a-real-condition", "review")])];
        let proposals = evaluate_transitions(&graph, &statuses);
        // Unknown condition silently skipped — no panic, no proposals.
        assert!(proposals.is_empty());
    }
}
