//! Circular dependency detection for relationships with the "dependency" semantic.

use std::collections::HashSet;

use crate::graph::ArtifactGraph;
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity, ValidationContext};

/// Detect circular dependencies on any relationship with the "dependency" semantic.
pub fn check_circular_dependencies(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    if ctx.dependency_keys.is_empty() {
        return;
    }

    // Only forward dependency keys (not inverse like "depended-on-by").
    let forward_dep_keys: HashSet<&str> = ctx
        .relationships
        .iter()
        .filter(|r| r.semantic.as_deref() == Some("dependency"))
        .map(|r| r.key.as_str())
        .collect();

    if forward_dep_keys.is_empty() {
        return;
    }

    let mut reported: HashSet<String> = HashSet::new();

    for node in graph.nodes.values() {
        let deps: Vec<String> = node
            .references_out
            .iter()
            .filter(|r| {
                r.relationship_type
                    .as_deref()
                    .is_some_and(|t| forward_dep_keys.contains(t))
            })
            .map(|r| r.target_id.clone())
            .collect();

        if deps.is_empty() {
            continue;
        }

        detect_cycles_from(
            graph,
            &node.id,
            &deps,
            &forward_dep_keys,
            &mut reported,
            checks,
        );
    }
}

/// Run DFS cycle detection from a single node.
fn detect_cycles_from(
    graph: &ArtifactGraph,
    start_id: &str,
    initial_dep_ids: &[String],
    dep_keys: &HashSet<&str>,
    reported: &mut HashSet<String>,
    checks: &mut Vec<IntegrityCheck>,
) {
    let mut visited = HashSet::new();
    let mut stack: Vec<(String, Vec<String>)> = initial_dep_ids
        .iter()
        .map(|id| (id.clone(), vec![start_id.to_owned()]))
        .collect();

    while let Some((current_id, path)) = stack.pop() {
        if current_id == start_id {
            report_cycle(start_id, &path, reported, checks);
            continue;
        }
        if !visited.insert(current_id.clone()) {
            continue;
        }
        if let Some(dep_node) = graph.nodes.get(&current_id) {
            let next_deps: Vec<String> = dep_node
                .references_out
                .iter()
                .filter(|r| {
                    r.relationship_type
                        .as_deref()
                        .is_some_and(|t| dep_keys.contains(t))
                })
                .map(|r| r.target_id.clone())
                .collect();
            for next_id in next_deps {
                let mut new_path = path.clone();
                new_path.push(current_id.clone());
                stack.push((next_id, new_path));
            }
        }
    }
}

fn report_cycle(
    start_id: &str,
    path: &[String],
    reported: &mut HashSet<String>,
    checks: &mut Vec<IntegrityCheck>,
) {
    let mut cycle_parts = path.to_vec();
    cycle_parts.sort();
    let cycle_key = cycle_parts.join(",");
    if !reported.contains(&cycle_key) {
        reported.insert(cycle_key);
        checks.push(IntegrityCheck {
            category: IntegrityCategory::CircularDependency,
            severity: IntegritySeverity::Error,
            artifact_id: start_id.to_owned(),
            message: format!(
                "Circular dependency: {} \u{2192} {} \u{2192} {}",
                start_id,
                path[1..].join(" \u{2192} "),
                start_id
            ),
            auto_fixable: false,
            fix_description: Some("Break the dependency cycle by removing one edge".to_owned()),
        });
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{RelationshipSchema, ValidationContext};
    use orqa_engine_types::config::DeliveryConfig;
    use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef};
    use std::collections::HashMap;

    fn dep_schema() -> RelationshipSchema {
        RelationshipSchema {
            key: "depends-on".to_owned(),
            inverse: "depended-on-by".to_owned(),
            description: "dependency".to_owned(),
            from: vec![],
            to: vec![],
            semantic: Some("dependency".to_owned()),
            constraints: None,
        }
    }

    fn make_ctx_with_dep() -> ValidationContext {
        let schema = dep_schema();
        let mut dep_keys = HashSet::new();
        dep_keys.insert("depends-on".to_owned());
        dep_keys.insert("depended-on-by".to_owned());
        ValidationContext {
            relationships: vec![schema],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: DeliveryConfig::default(),
            dependency_keys: dep_keys,
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        }
    }

    fn make_node_with_deps(id: &str, deps: &[&str]) -> ArtifactNode {
        let references_out = deps
            .iter()
            .map(|target| ArtifactRef {
                target_id: target.to_string(),
                field: "relationships".to_owned(),
                source_id: id.to_owned(),
                relationship_type: Some("depends-on".to_owned()),
            })
            .collect();
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: format!(".orqa/test/{id}.md"),
            artifact_type: "task".to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({}),
            body: None,
            references_out,
            references_in: vec![],
        }
    }

    #[test]
    fn no_cycles_in_empty_graph() {
        let graph = ArtifactGraph::default();
        let ctx = make_ctx_with_dep();
        let mut checks = vec![];
        check_circular_dependencies(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn no_cycles_when_no_dependency_keys() {
        let mut graph = ArtifactGraph::default();
        let node_a = make_node_with_deps("A", &["B"]);
        let node_b = make_node_with_deps("B", &["A"]);
        graph.nodes.insert("A".to_owned(), node_a);
        graph.nodes.insert("B".to_owned(), node_b);

        let ctx = ValidationContext {
            relationships: vec![],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: DeliveryConfig::default(),
            dependency_keys: HashSet::new(), // no dep keys
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        };
        let mut checks = vec![];
        check_circular_dependencies(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn direct_cycle_detected() {
        // A -> B -> A
        let mut graph = ArtifactGraph::default();
        let node_a = make_node_with_deps("TASK-A", &["TASK-B"]);
        let node_b = make_node_with_deps("TASK-B", &["TASK-A"]);
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("TASK-B".to_owned(), node_b);

        let ctx = make_ctx_with_dep();
        let mut checks = vec![];
        check_circular_dependencies(&graph, &ctx, &mut checks);
        // The cycle should be reported (exactly once due to dedup)
        assert!(!checks.is_empty());
        assert_eq!(checks[0].category, IntegrityCategory::CircularDependency);
    }

    #[test]
    fn no_cycle_in_linear_chain() {
        // A -> B -> C (no cycle)
        let mut graph = ArtifactGraph::default();
        let node_a = make_node_with_deps("TASK-A", &["TASK-B"]);
        let node_b = make_node_with_deps("TASK-B", &["TASK-C"]);
        let node_c = make_node_with_deps("TASK-C", &[]);
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("TASK-B".to_owned(), node_b);
        graph.nodes.insert("TASK-C".to_owned(), node_c);

        let ctx = make_ctx_with_dep();
        let mut checks = vec![];
        check_circular_dependencies(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn longer_cycle_detected() {
        // A -> B -> C -> A
        let mut graph = ArtifactGraph::default();
        let node_a = make_node_with_deps("TASK-A", &["TASK-B"]);
        let node_b = make_node_with_deps("TASK-B", &["TASK-C"]);
        let node_c = make_node_with_deps("TASK-C", &["TASK-A"]);
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("TASK-B".to_owned(), node_b);
        graph.nodes.insert("TASK-C".to_owned(), node_c);

        let ctx = make_ctx_with_dep();
        let mut checks = vec![];
        check_circular_dependencies(&graph, &ctx, &mut checks);
        assert!(!checks.is_empty());
        assert_eq!(checks[0].category, IntegrityCategory::CircularDependency);
    }

    #[test]
    fn self_loop_detected() {
        // A -> A
        let mut graph = ArtifactGraph::default();
        let node_a = make_node_with_deps("TASK-A", &["TASK-A"]);
        graph.nodes.insert("TASK-A".to_owned(), node_a);

        let ctx = make_ctx_with_dep();
        let mut checks = vec![];
        check_circular_dependencies(&graph, &ctx, &mut checks);
        assert!(!checks.is_empty());
    }

    #[test]
    fn non_dependency_relationships_ignored() {
        // A has a "delivers" edge to B (not a dependency), no cycle should be reported
        let mut graph = ArtifactGraph::default();
        let mut node_a = make_node_with_deps("TASK-A", &[]);
        node_a.references_out.push(ArtifactRef {
            target_id: "TASK-B".to_owned(),
            field: "relationships".to_owned(),
            source_id: "TASK-A".to_owned(),
            relationship_type: Some("delivers".to_owned()),
        });
        let mut node_b = make_node_with_deps("TASK-B", &[]);
        node_b.references_out.push(ArtifactRef {
            target_id: "TASK-A".to_owned(),
            field: "relationships".to_owned(),
            source_id: "TASK-B".to_owned(),
            relationship_type: Some("delivers".to_owned()),
        });
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("TASK-B".to_owned(), node_b);

        let ctx = make_ctx_with_dep(); // dep keys = {depends-on, depended-on-by}
        let mut checks = vec![];
        check_circular_dependencies(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }
}
