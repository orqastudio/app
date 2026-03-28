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
        .map(|id| (id.clone(), vec![start_id.to_string()]))
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
            artifact_id: start_id.to_string(),
            message: format!(
                "Circular dependency: {} \u{2192} {} \u{2192} {}",
                start_id,
                path[1..].join(" \u{2192} "),
                start_id
            ),
            auto_fixable: false,
            fix_description: Some("Break the dependency cycle by removing one edge".to_string()),
        });
    }
}
