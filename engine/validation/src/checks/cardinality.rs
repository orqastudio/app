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
