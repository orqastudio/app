//! Structural integrity checks: broken refs, missing inverses, type constraints, required relationships.

use std::collections::HashMap;

use crate::graph::{ArtifactGraph, ArtifactNode, ArtifactRef};
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity, RelationshipSchema, ValidationContext};

/// Check for broken references — target_id doesn't exist in the graph.
pub fn check_broken_refs(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        for ref_entry in &node.references_out {
            if !graph.nodes.contains_key(&ref_entry.target_id) {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::BrokenLink,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "Reference to {} (field: {}) does not resolve to any artifact",
                        ref_entry.target_id, ref_entry.field
                    ),
                    auto_fixable: false,
                    fix_description: None,
                });
            }
        }
    }
}

/// Check for missing bidirectional inverses on relationship edges.
pub fn check_missing_inverses(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        for ref_entry in &node.references_out {
            let rel_type = match &ref_entry.relationship_type {
                Some(t) => t.as_str(),
                None => continue,
            };

            let expected_inverse = match ctx.inverse_map.get(rel_type) {
                Some(inv) => inv.as_str(),
                None => continue,
            };

            let Some(target) = graph.nodes.get(&ref_entry.target_id) else {
                continue; // broken ref, caught by check_broken_refs
            };

            let has_inverse = target.references_out.iter().any(|r| {
                r.relationship_type.as_deref() == Some(expected_inverse) && r.target_id == node.id
            });

            if !has_inverse {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::MissingInverse,
                    severity: IntegritySeverity::Warning,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} --{}--> {} but {} has no {} edge back to {}",
                        node.id,
                        rel_type,
                        ref_entry.target_id,
                        ref_entry.target_id,
                        expected_inverse,
                        node.id
                    ),
                    auto_fixable: true,
                    fix_description: Some(format!(
                        "Add {{ target: \"{}\", type: \"{}\" }} to {}'s relationships array",
                        node.id, expected_inverse, ref_entry.target_id
                    )),
                });
            }
        }
    }
}

/// Check that from/to type constraints on relationships are satisfied.
pub fn check_relationship_type_constraints(
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
        for ref_entry in &node.references_out {
            let Some(rel_type) = ref_entry.relationship_type.as_deref() else {
                continue;
            };
            let Some(schema) = schema_map.get(rel_type) else {
                continue;
            };
            check_from_constraint(node, rel_type, schema, checks);
            check_to_constraint(node, ref_entry, rel_type, schema, graph, checks);
        }
    }
}

fn check_from_constraint(
    node: &ArtifactNode,
    rel_type: &str,
    schema: &RelationshipSchema,
    checks: &mut Vec<IntegrityCheck>,
) {
    if !schema.from.is_empty() && !schema.from.contains(&node.artifact_type) {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::TypeConstraintViolation,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: format!(
                "{} ({}) uses '{}' relationship but only [{}] types may use it as source",
                node.id,
                node.artifact_type,
                rel_type,
                schema.from.join(", ")
            ),
            auto_fixable: false,
            fix_description: Some(format!(
                "Change the relationship type or move the artifact to a valid type: {}",
                schema.from.join(", ")
            )),
        });
    }
}

fn check_to_constraint(
    node: &ArtifactNode,
    ref_entry: &ArtifactRef,
    rel_type: &str,
    schema: &RelationshipSchema,
    graph: &ArtifactGraph,
    checks: &mut Vec<IntegrityCheck>,
) {
    if schema.to.is_empty() {
        return;
    }
    let Some(target) = graph.nodes.get(&ref_entry.target_id) else {
        return;
    };
    if !schema.to.contains(&target.artifact_type) {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::TypeConstraintViolation,
            severity: IntegritySeverity::Error,
            artifact_id: node.id.clone(),
            message: format!(
                "{} --{}--> {} ({}) but '{}' only targets [{}] types",
                node.id,
                rel_type,
                ref_entry.target_id,
                target.artifact_type,
                rel_type,
                schema.to.join(", ")
            ),
            auto_fixable: false,
            fix_description: Some(format!(
                "Change the target to one of: {}",
                schema.to.join(", ")
            )),
        });
    }
}

/// Check that every artifact has a `type:` field in its frontmatter.
///
/// The inferred type (computed during graph construction) is used as the suggestion.
/// Artifacts whose inferred type is the generic `"doc"` fallback are excluded — they
/// either have no meaningful type to add or live outside a recognised path prefix.
pub fn check_missing_type_field(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        // Only flag if the frontmatter has NO `type` key at all.
        let has_type_field = node
            .frontmatter
            .get("type")
            .is_some_and(|v| !v.is_null() && v.as_str().is_some_and(|s| !s.is_empty()));

        if has_type_field {
            continue;
        }

        // Skip generic doc fallbacks — they live outside configured paths.
        if node.artifact_type == "doc" {
            continue;
        }

        checks.push(IntegrityCheck {
            category: IntegrityCategory::MissingType,
            severity: IntegritySeverity::Warning,
            artifact_id: node.id.clone(),
            message: format!(
                "{} has no 'type:' field — inferred as '{}'",
                node.id, node.artifact_type
            ),
            auto_fixable: true,
            fix_description: Some(format!("Add type: {} to frontmatter", node.artifact_type)),
        });
    }
}

/// Check that every artifact has a `status:` field in its frontmatter.
///
/// Artifacts living in the `doc` type fallback are excluded since they often have
/// no lifecycle status.
pub fn check_missing_status_field(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    // Types that don't require lifecycle status.
    const EXCLUDED_TYPES: &[&str] = &["doc", "pillar", "persona", "knowledge"];

    for node in graph.nodes.values() {
        if EXCLUDED_TYPES.contains(&node.artifact_type.as_str()) {
            continue;
        }

        let has_status = node
            .frontmatter
            .get("status")
            .is_some_and(|v| !v.is_null() && v.as_str().is_some_and(|s| !s.is_empty()));

        if has_status {
            continue;
        }

        checks.push(IntegrityCheck {
            category: IntegrityCategory::MissingStatus,
            severity: IntegritySeverity::Warning,
            artifact_id: node.id.clone(),
            message: format!("{} has no 'status:' field", node.id),
            auto_fixable: true,
            fix_description: Some("Add status: captured to frontmatter".to_owned()),
        });
    }
}

/// Check for duplicate relationship entries (same target + type appearing more than once).
pub fn check_duplicate_relationships(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        let mut seen: std::collections::HashMap<(&str, &str), usize> =
            std::collections::HashMap::new();

        for ref_entry in &node.references_out {
            if ref_entry.field != "relationships" {
                continue;
            }
            let rel_type = ref_entry.relationship_type.as_deref().unwrap_or("");
            *seen
                .entry((ref_entry.target_id.as_str(), rel_type))
                .or_insert(0) += 1;
        }

        for ((target_id, rel_type), count) in &seen {
            if *count > 1 {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::DuplicateRelationship,
                    severity: IntegritySeverity::Warning,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} has {} duplicate '{}' relationship entries to {}",
                        node.id, count - 1, rel_type, target_id
                    ),
                    auto_fixable: true,
                    fix_description: Some(format!(
                        "Remove {} duplicate relationship entry/entries (target: {target_id}, type: {rel_type})",
                        count - 1
                    )),
                });
            }
        }
    }
}

/// Check that required relationships are present with minimum counts.
pub fn check_required_relationships(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    for schema in &ctx.relationships {
        let constraints = match &schema.constraints {
            Some(c) if c.required == Some(true) => c,
            _ => continue,
        };

        let min_count = constraints.min_count.unwrap_or(1);

        // Only check artifacts whose type is in the `from` list.
        // If `from` is empty, this constraint applies to all types (skip — too broad).
        if schema.from.is_empty() {
            continue;
        }

        for node in graph.nodes.values() {
            if !schema.from.contains(&node.artifact_type) {
                continue;
            }

            // Skip terminal/archived statuses — completed artifacts don't need new edges.
            if let Some(status) = &node.status {
                let s = status.as_str();
                if s == "completed" || s == "surpassed" || s == "archived" {
                    continue;
                }
            }

            let count = node
                .references_out
                .iter()
                .filter(|r| r.relationship_type.as_deref() == Some(&schema.key))
                .count();

            if count < min_count {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::RequiredRelationshipMissing,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} ({}) requires at least {} '{}' relationship(s) but has {}",
                        node.id, node.artifact_type, min_count, schema.key, count
                    ),
                    auto_fixable: false,
                    fix_description: Some(format!(
                        "Add a '{}' relationship targeting a {} artifact",
                        schema.key,
                        if schema.to.is_empty() {
                            "valid".to_owned()
                        } else {
                            schema.to.join(" or ")
                        }
                    )),
                });
            }
        }
    }
}
