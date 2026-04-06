//! Cardinality constraint checks — maxCount violations.

use std::collections::HashMap;

use crate::graph::ArtifactGraph;
use crate::types::{
    IntegrityCategory, IntegrityCheck, IntegritySeverity, RelationshipSchema, ValidationContext,
};

/// Check that `maxCount` cardinality constraints are not exceeded.
pub fn check_cardinality(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    let schema_map: HashMap<&str, &RelationshipSchema> = ctx
        .relationships
        .iter()
        .map(|r| (r.key.as_str(), r))
        .collect();

    for node in graph.nodes.values() {
        // Count outgoing edges by relationship type.
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for ref_entry in &node.references_out {
            if let Some(rel_type) = &ref_entry.relationship_type {
                *counts.entry(rel_type.as_str()).or_default() += 1;
            }
        }

        for (rel_type, count) in &counts {
            let Some(schema) = schema_map.get(rel_type) else {
                continue;
            };
            if let Some(constraints) = &schema.constraints {
                if let Some(max) = constraints.max_count {
                    if *count > max {
                        checks.push(IntegrityCheck {
                            category: IntegrityCategory::CardinalityViolation,
                            severity: IntegritySeverity::Error,
                            artifact_id: node.id.clone(),
                            message: format!(
                                "{} has {} '{}' relationships but maximum is {}",
                                node.id, count, rel_type, max
                            ),
                            auto_fixable: false,
                            fix_description: Some(format!(
                                "Remove excess '{rel_type}' relationships to comply with max count {max}",
                            )),
                        });
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{RelationshipConstraints, RelationshipSchema, ValidationContext};
    use orqa_engine_types::config::DeliveryConfig;
    use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef};
    use std::collections::{HashMap, HashSet};

    fn make_context(rel_key: &str, max: usize) -> ValidationContext {
        let schema = RelationshipSchema {
            key: rel_key.to_owned(),
            inverse: format!("{rel_key}-by"),
            description: String::new(),
            from: vec![],
            to: vec![],
            semantic: None,
            constraints: Some(RelationshipConstraints {
                required: None,
                min_count: None,
                max_count: Some(max),
                require_inverse: None,
                status_rules: vec![],
            }),
        };
        ValidationContext {
            relationships: vec![schema],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: DeliveryConfig::default(),
            dependency_keys: HashSet::default(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        }
    }

    fn make_node_with_refs(id: &str, refs: Vec<(&str, &str)>) -> ArtifactNode {
        let references_out = refs
            .into_iter()
            .map(|(target, rel_type)| ArtifactRef {
                target_id: target.to_owned(),
                field: "relationships".to_owned(),
                source_id: id.to_owned(),
                relationship_type: Some(rel_type.to_owned()),
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
    fn no_checks_when_graph_empty() {
        let graph = ArtifactGraph::default();
        let ctx = make_context("delivers", 1);
        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn no_violation_when_count_within_max() {
        let mut graph = ArtifactGraph::default();
        let node = make_node_with_refs("TASK-001", vec![("EPIC-001", "delivers")]);
        graph.nodes.insert(node.id.clone(), node);

        let ctx = make_context("delivers", 2);
        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn violation_when_count_exceeds_max() {
        let mut graph = ArtifactGraph::default();
        // 3 "delivers" edges, but max is 1
        let node = make_node_with_refs(
            "TASK-001",
            vec![
                ("EPIC-001", "delivers"),
                ("EPIC-002", "delivers"),
                ("EPIC-003", "delivers"),
            ],
        );
        graph.nodes.insert(node.id.clone(), node);

        let ctx = make_context("delivers", 1);
        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].artifact_id, "TASK-001");
        assert!(checks[0].message.contains("delivers"));
        assert!(checks[0].message.contains('3'));
        assert!(checks[0].message.contains('1'));
    }

    #[test]
    fn exactly_at_max_produces_no_violation() {
        let mut graph = ArtifactGraph::default();
        let node = make_node_with_refs(
            "TASK-001",
            vec![("EPIC-001", "delivers"), ("EPIC-002", "delivers")],
        );
        graph.nodes.insert(node.id.clone(), node);

        let ctx = make_context("delivers", 2);
        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn unknown_relationship_type_is_skipped() {
        let mut graph = ArtifactGraph::default();
        // "grounded-by" is not in the context schema
        let node = make_node_with_refs(
            "TASK-001",
            vec![
                ("PILLAR-001", "grounded-by"),
                ("PILLAR-002", "grounded-by"),
                ("PILLAR-003", "grounded-by"),
            ],
        );
        graph.nodes.insert(node.id.clone(), node);

        let ctx = make_context("delivers", 1); // schema only knows "delivers"
        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn no_violation_when_schema_has_no_max_count() {
        let mut graph = ArtifactGraph::default();
        let node = make_node_with_refs(
            "TASK-001",
            vec![
                ("EPIC-001", "delivers"),
                ("EPIC-002", "delivers"),
                ("EPIC-003", "delivers"),
            ],
        );
        graph.nodes.insert(node.id.clone(), node);

        // Schema with no max_count set
        let schema = RelationshipSchema {
            key: "delivers".to_owned(),
            inverse: "delivered-by".to_owned(),
            description: String::new(),
            from: vec![],
            to: vec![],
            semantic: None,
            constraints: Some(RelationshipConstraints {
                required: None,
                min_count: None,
                max_count: None,
                require_inverse: None,
                status_rules: vec![],
            }),
        };
        let ctx = ValidationContext {
            relationships: vec![schema],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: DeliveryConfig::default(),
            dependency_keys: HashSet::default(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        };

        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn refs_without_relationship_type_are_ignored() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node_with_refs("TASK-001", vec![]);
        // Add a ref with no relationship_type
        node.references_out.push(ArtifactRef {
            target_id: "EPIC-001".to_owned(),
            field: "some-field".to_owned(),
            source_id: "TASK-001".to_owned(),
            relationship_type: None,
        });
        graph.nodes.insert(node.id.clone(), node);

        let ctx = make_context("delivers", 0); // max 0 would trigger if matched
        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn multiple_nodes_each_checked_independently() {
        let mut graph = ArtifactGraph::default();
        // Node A: 2 delivers edges (violates max=1)
        let node_a = make_node_with_refs(
            "TASK-A",
            vec![("EPIC-001", "delivers"), ("EPIC-002", "delivers")],
        );
        // Node B: 1 delivers edge (OK at max=1)
        let node_b = make_node_with_refs("TASK-B", vec![("EPIC-001", "delivers")]);
        graph.nodes.insert(node_a.id.clone(), node_a);
        graph.nodes.insert(node_b.id.clone(), node_b);

        let ctx = make_context("delivers", 1);
        let mut checks = vec![];
        check_cardinality(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].artifact_id, "TASK-A");
    }
}
