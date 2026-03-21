//! Body text reference validation — checks that body-text artifact references
//! are noted as informational.
//!
//! Body text mentions are NOT knowledge flow relationships. They're citations —
//! "this ID appeared in prose." The graph records them as informational edges
//! with `field: "body"` and no relationship type, but they should NOT be
//! auto-fixed into formal relationships (that caused incorrect `informed-by`
//! edges on non-decision types).

use crate::graph::ArtifactGraph;
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

/// Note body-text references that don't have formal relationship edges.
///
/// These are INFO-level, NOT auto-fixable. Body text mentions are references,
/// not knowledge flow. Adding formal relationships from body text creates
/// semantically incorrect edges (e.g., `informed-by` on epics).
pub fn check_body_text_refs_without_relationships(
    graph: &ArtifactGraph,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        for body_ref in node.references_out.iter().filter(|r| r.field == "body") {
            let target_id = &body_ref.target_id;

            let has_relationship = node
                .references_out
                .iter()
                .any(|r| r.field != "body" && &r.target_id == target_id);

            if !has_relationship {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::BodyTextRefWithoutRelationship,
                    severity: IntegritySeverity::Info,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} mentions {} in body text (informational — not a knowledge flow relationship)",
                        node.id, target_id
                    ),
                    auto_fixable: false,
                    fix_description: None,
                });
            }
        }
    }
}
