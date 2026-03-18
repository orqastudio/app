//! Schema-driven integrity engine.
//!
//! Replaces the 25 hardcoded check functions with a generic engine that reads
//! constraints from relationship schemas (core.json + project.json + plugin
//! manifests). Every check is derived from schema metadata — no relationship
//! keys or artifact types are hardcoded.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::domain::artifact_graph::{
    ArtifactGraph, ArtifactNode, IntegrityCategory, IntegrityCheck, IntegritySeverity,
};
use crate::domain::platform_config::PLATFORM;
use crate::domain::project_settings::{DeliveryConfig, ProjectRelationshipConfig};

// ---------------------------------------------------------------------------
// Schema types
// ---------------------------------------------------------------------------

/// A status rule constraint from the relationship schema.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusRule {
    /// Which side to evaluate: `"source"` or `"target"`.
    pub evaluate: String,
    /// Condition to test: `"all-targets-in"`, `"any-target-in"`, `"no-targets-in"`.
    pub condition: String,
    /// The status values to check against.
    pub statuses: Vec<String>,
    /// The status to propose when the condition is met.
    #[serde(rename = "proposedStatus")]
    pub proposed_status: String,
    /// Human-readable description of this rule.
    pub description: String,
}

/// Constraint block on a relationship definition.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RelationshipConstraints {
    /// Whether artifacts of the `from` type must have at least one instance of this relationship.
    #[serde(default)]
    pub required: Option<bool>,
    /// Minimum number of relationships of this type required (only when `required` is true).
    #[serde(rename = "minCount", default)]
    pub min_count: Option<usize>,
    /// Maximum number of relationships of this type allowed.
    #[serde(rename = "maxCount", default)]
    pub max_count: Option<usize>,
    /// Whether the inverse edge must exist. Defaults to schema-level behaviour.
    #[serde(rename = "requireInverse", default)]
    pub require_inverse: Option<bool>,
    /// Status-based transition rules.
    #[serde(rename = "statusRules", default)]
    pub status_rules: Vec<StatusRule>,
}

/// A relationship schema entry — combines platform, project, and plugin definitions.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationshipSchema {
    pub key: String,
    pub inverse: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub from: Vec<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub semantic: Option<String>,
    #[serde(default)]
    pub constraints: Option<RelationshipConstraints>,
}

/// The full validation context loaded from core.json + project.json + plugins.
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// All relationship schemas, keyed by relationship key for fast lookup.
    pub relationships: Vec<RelationshipSchema>,
    /// Maps each relationship key to its inverse.
    pub inverse_map: HashMap<String, String>,
    /// The valid status values from project.json.
    pub valid_statuses: Vec<String>,
    /// The delivery config from project.json (kept for delivery-path checks).
    pub delivery: DeliveryConfig,
    /// Relationship keys that have the "dependency" semantic.
    pub dependency_keys: HashSet<String>,
}

// ---------------------------------------------------------------------------
// Context loading
// ---------------------------------------------------------------------------

/// Build a `ValidationContext` by merging platform config, project relationships,
/// and (in future) plugin manifests.
pub fn build_validation_context(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
) -> ValidationContext {
    let mut relationships: Vec<RelationshipSchema> = Vec::new();
    let mut inverse_map: HashMap<String, String> = HashMap::new();

    // 1. Platform relationships from the embedded core.json.
    for rel in &PLATFORM.relationships {
        relationships.push(RelationshipSchema {
            key: rel.key.clone(),
            inverse: rel.inverse.clone(),
            description: rel.description.clone(),
            from: rel.from.clone(),
            to: rel.to.clone(),
            semantic: rel.semantic.clone(),
            constraints: None,
        });
        inverse_map.insert(rel.key.clone(), rel.inverse.clone());
        if rel.inverse != rel.key {
            inverse_map.insert(rel.inverse.clone(), rel.key.clone());
        }
    }

    // 2. Plugin relationships — extend existing definitions or add new ones.
    // When a plugin declares a key that already exists (e.g. extending core's
    // `merged-into` to also allow research→research), the from/to arrays are
    // unioned. Collision detection happens at plugin install time, not here.
    for pr in plugin_relationships {
        if let Some(existing) = relationships.iter_mut().find(|r| r.key == pr.key) {
            // Extend from/to constraints (union)
            for t in &pr.from {
                if !existing.from.contains(t) {
                    existing.from.push(t.clone());
                }
            }
            for t in &pr.to {
                if !existing.to.contains(t) {
                    existing.to.push(t.clone());
                }
            }
            // Merge constraints if plugin provides them and existing doesn't
            if pr.constraints.is_some() && existing.constraints.is_none() {
                existing.constraints = pr.constraints.clone();
            }
        } else {
            // New relationship from plugin
            relationships.push(pr.clone());
        }
        inverse_map.insert(pr.key.clone(), pr.inverse.clone());
        if pr.inverse != pr.key {
            inverse_map.insert(pr.inverse.clone(), pr.key.clone());
        }
    }

    // 3. Project relationships from project.json.
    for pr in project_relationships {
        // Only add if not already defined by platform or plugin.
        if !inverse_map.contains_key(&pr.key) {
            relationships.push(RelationshipSchema {
                key: pr.key.clone(),
                inverse: pr.inverse.clone(),
                description: String::new(),
                from: vec![],
                to: vec![],
                semantic: None,
                constraints: None,
            });
        }
        inverse_map.insert(pr.key.clone(), pr.inverse.clone());
        if pr.inverse != pr.key {
            inverse_map.insert(pr.inverse.clone(), pr.key.clone());
        }
    }

    // 4. Collect dependency keys from semantics.
    let mut dependency_keys = HashSet::new();
    if let Some(sem) = PLATFORM.semantics.get("dependency") {
        for k in &sem.keys {
            dependency_keys.insert(k.clone());
        }
    }
    // Also check plugin/project semantics: any relationship with "dependency" semantic.
    for rel in &relationships {
        if rel.semantic.as_deref() == Some("dependency") {
            dependency_keys.insert(rel.key.clone());
            dependency_keys.insert(rel.inverse.clone());
        }
    }

    ValidationContext {
        relationships,
        inverse_map,
        valid_statuses: valid_statuses.to_vec(),
        delivery: delivery.clone(),
        dependency_keys,
    }
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

/// Run all schema-driven integrity checks on the graph.
///
/// Returns a list of findings (errors and warnings).
pub fn run_schema_checks(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck> {
    let mut checks = Vec::new();

    check_broken_refs(graph, &mut checks);
    check_missing_inverses(graph, ctx, &mut checks);
    check_relationship_type_constraints(graph, ctx, &mut checks);
    check_required_relationships(graph, ctx, &mut checks);
    check_cardinality(graph, ctx, &mut checks);
    check_circular_dependencies(graph, ctx, &mut checks);
    check_body_text_refs_without_relationships(graph, &mut checks);

    if !ctx.valid_statuses.is_empty() {
        check_valid_statuses(graph, ctx, &mut checks);
        check_parent_child_consistency(graph, ctx, &mut checks);
    }

    if !ctx.delivery.types.is_empty() {
        check_delivery_paths(graph, ctx, &mut checks);
    }

    checks
}

// ---------------------------------------------------------------------------
// Generic checks
// ---------------------------------------------------------------------------

/// Check for broken references — target_id doesn't exist in the graph.
fn check_broken_refs(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
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
fn check_missing_inverses(
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
                r.relationship_type.as_deref() == Some(expected_inverse)
                    && r.target_id == node.id
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
fn check_relationship_type_constraints(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    // Build a lookup: relationship key → schema.
    let schema_map: HashMap<&str, &RelationshipSchema> =
        ctx.relationships.iter().map(|r| (r.key.as_str(), r)).collect();

    for node in graph.nodes.values() {
        for ref_entry in &node.references_out {
            let rel_type = match &ref_entry.relationship_type {
                Some(t) => t.as_str(),
                None => continue,
            };

            let Some(schema) = schema_map.get(rel_type) else {
                continue; // Unknown relationship — not a type-constraint issue.
            };

            // Check `from` constraint: the source type must be in the `from` list.
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

            // Check `to` constraint: the target type must be in the `to` list.
            if !schema.to.is_empty() {
                if let Some(target) = graph.nodes.get(&ref_entry.target_id) {
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
            }
        }
    }
}

/// Check that required relationships are present with minimum counts.
fn check_required_relationships(
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

/// Check that maxCount cardinality constraints are not exceeded.
fn check_cardinality(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    let schema_map: HashMap<&str, &RelationshipSchema> =
        ctx.relationships.iter().map(|r| (r.key.as_str(), r)).collect();

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
                            severity: IntegritySeverity::Warning,
                            artifact_id: node.id.clone(),
                            message: format!(
                                "{} has {} '{}' relationships but maximum is {}",
                                node.id, count, rel_type, max
                            ),
                            auto_fixable: false,
                            fix_description: Some(format!(
                                "Remove excess '{}' relationships to comply with max count {}",
                                rel_type, max
                            )),
                        });
                    }
                }
            }
        }
    }
}

/// Detect circular dependencies on any relationship with the "dependency" semantic.
fn check_circular_dependencies(
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

        detect_cycles_from(graph, &node.id, &deps, &forward_dep_keys, &mut reported, checks);
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
    let mut stack = Vec::new();

    for dep_id in initial_dep_ids {
        stack.push((dep_id.clone(), vec![start_id.to_string()]));
    }

    while let Some((current_id, path)) = stack.pop() {
        if current_id == start_id {
            let mut cycle_parts = path.clone();
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
                    fix_description: Some(
                        "Break the dependency cycle by removing one edge".to_string(),
                    ),
                });
            }
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

/// Check that body-text references have corresponding relationship edges.
fn check_body_text_refs_without_relationships(
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
                    severity: IntegritySeverity::Warning,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} references {} in body text but has no relationship edge to it",
                        node.id, target_id
                    ),
                    auto_fixable: true,
                    fix_description: Some(format!(
                        "Add {{ target: \"{}\", type: \"informed-by\" }} to {}'s relationships array",
                        target_id, node.id
                    )),
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Status checks
// ---------------------------------------------------------------------------

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
fn check_valid_statuses(
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
            severity: IntegritySeverity::Warning,
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

/// Check parent-child status consistency using the delivery hierarchy.
fn check_parent_child_consistency(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    let status_pos: HashMap<&str, usize> = ctx
        .valid_statuses
        .iter()
        .enumerate()
        .map(|(i, s)| (s.as_str(), i))
        .collect();

    if ctx.delivery.types.is_empty() {
        check_parent_child_consistency_hardcoded(graph, &status_pos, checks);
        return;
    }

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
        if child_pos > parent_pos {
            checks.push(IntegrityCheck {
                artifact_id: node.id.clone(),
                category: IntegrityCategory::ParentChildInconsistency,
                severity: IntegritySeverity::Warning,
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

/// Hardcoded fallback for parent-child consistency when no delivery config is present.
fn check_parent_child_consistency_hardcoded(
    graph: &ArtifactGraph,
    status_pos: &HashMap<&str, usize>,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph.nodes.values() {
        let Some(child_status) = node.status.as_deref() else {
            continue;
        };
        let Some(&child_pos) = status_pos.get(child_status) else {
            continue;
        };

        // Check epic parent.
        if let Some(parent_id) = node.frontmatter.get("epic").and_then(|v| v.as_str()) {
            if let Some(parent) = graph.nodes.get(parent_id) {
                if let Some(parent_status) = &parent.status {
                    if let Some(&parent_pos) = status_pos.get(parent_status.as_str()) {
                        if child_pos > parent_pos {
                            push_parent_child_inconsistency(
                                checks,
                                &node.id,
                                child_status,
                                parent_id,
                                parent_status,
                                "epic",
                            );
                        }
                    }
                }
            }
        }

        // Check milestone parent.
        if let Some(parent_id) = node.frontmatter.get("milestone").and_then(|v| v.as_str()) {
            if let Some(parent) = graph.nodes.get(parent_id) {
                if let Some(parent_status) = &parent.status {
                    if let Some(&parent_pos) = status_pos.get(parent_status.as_str()) {
                        if child_pos > parent_pos {
                            push_parent_child_inconsistency(
                                checks,
                                &node.id,
                                child_status,
                                parent_id,
                                parent_status,
                                "milestone",
                            );
                        }
                    }
                }
            }
        }
    }
}

fn push_parent_child_inconsistency(
    checks: &mut Vec<IntegrityCheck>,
    child_id: &str,
    child_status: &str,
    parent_id: &str,
    parent_status: &str,
    parent_label: &str,
) {
    checks.push(IntegrityCheck {
        artifact_id: child_id.to_owned(),
        category: IntegrityCategory::ParentChildInconsistency,
        severity: IntegritySeverity::Warning,
        message: format!(
            "{child_id} is '{child_status}' but {parent_label} {parent_id} is '{parent_status}' \u{2014} child is further along than parent",
        ),
        auto_fixable: false,
        fix_description: Some(format!(
            "Either advance {parent_id} to at least '{child_status}', or move {child_id} to a different {parent_label}",
        )),
    });
}

// ---------------------------------------------------------------------------
// Delivery path checks
// ---------------------------------------------------------------------------

/// Validate delivery artifacts against the `DeliveryConfig`.
fn check_delivery_paths(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    for node in graph
        .nodes
        .values()
        .filter(|n| n.path.starts_with(".orqa/delivery/"))
    {
        let matched = ctx
            .delivery
            .types
            .iter()
            .find(|dt| node.path.starts_with(dt.path.trim_end_matches('/')));

        let Some(dtype) = matched else {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::DeliveryPathMismatch,
                severity: IntegritySeverity::Warning,
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
                severity: IntegritySeverity::Warning,
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
    dtype: &crate::domain::project_settings::DeliveryTypeConfig,
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
        checks.push(IntegrityCheck {
            category: IntegrityCategory::DeliveryPathMismatch,
            severity: IntegritySeverity::Warning,
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
        return;
    };

    let Some(parent_node) = graph.nodes.get(&parent_ref.target_id) else {
        return; // broken ref, caught by check_broken_refs
    };

    if parent_node.artifact_type != parent_cfg.parent_type {
        checks.push(IntegrityCheck {
            category: IntegrityCategory::DeliveryPathMismatch,
            severity: IntegritySeverity::Warning,
            artifact_id: node.id.clone(),
            message: format!(
                "{} has {} relationship to '{}' but {} is a '{}', expected '{}'",
                node.id,
                parent_cfg.relationship,
                parent_ref.target_id,
                parent_ref.target_id,
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
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::artifact_graph::{build_artifact_graph, ArtifactGraph, ArtifactNode, ArtifactRef};
    use std::collections::HashMap;

    fn empty_ctx() -> ValidationContext {
        ValidationContext {
            relationships: vec![],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: DeliveryConfig::default(),
            dependency_keys: HashSet::new(),
        }
    }

    fn ctx_with_statuses(statuses: Vec<String>) -> ValidationContext {
        ValidationContext {
            valid_statuses: statuses,
            ..empty_ctx()
        }
    }

    fn ctx_with_inverse(key: &str, inverse: &str) -> ValidationContext {
        let mut inverse_map = HashMap::new();
        inverse_map.insert(key.to_string(), inverse.to_string());
        inverse_map.insert(inverse.to_string(), key.to_string());
        ValidationContext {
            inverse_map,
            relationships: vec![RelationshipSchema {
                key: key.to_string(),
                inverse: inverse.to_string(),
                description: String::new(),
                from: vec![],
                to: vec![],
                semantic: None,
                constraints: None,
            }],
            ..empty_ctx()
        }
    }

    fn make_node(id: &str, art_type: &str) -> ArtifactNode {
        ArtifactNode {
            id: id.to_string(),
            project: None,
            path: format!(".orqa/test/{id}.md"),
            artifact_type: art_type.to_string(),
            title: id.to_string(),
            description: None,
            status: None,
            priority: None,
            frontmatter: serde_json::json!({}),
            references_out: vec![],
            references_in: vec![],
        }
    }

    fn make_ref(source: &str, target: &str, rel_type: &str) -> ArtifactRef {
        ArtifactRef {
            target_id: target.to_string(),
            field: "relationships".to_string(),
            source_id: source.to_string(),
            relationship_type: Some(rel_type.to_string()),
        }
    }

    #[test]
    fn broken_ref_detected() {
        let mut node = make_node("TASK-001", "task");
        node.references_out.push(make_ref("TASK-001", "EPIC-MISSING", "delivers"));

        let mut nodes = HashMap::new();
        nodes.insert("TASK-001".to_string(), node);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let checks = run_schema_checks(&graph, &empty_ctx());
        assert!(checks.iter().any(|c| matches!(c.category, IntegrityCategory::BrokenLink)));
    }

    #[test]
    fn missing_inverse_detected() {
        let mut rule = make_node("RULE-001", "rule");
        rule.references_out.push(make_ref("RULE-001", "AD-001", "enforces"));

        let decision = make_node("AD-001", "decision");

        let mut nodes = HashMap::new();
        nodes.insert("RULE-001".to_string(), rule);
        nodes.insert("AD-001".to_string(), decision);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let ctx = ctx_with_inverse("enforces", "enforced-by");
        let checks = run_schema_checks(&graph, &ctx);
        let missing: Vec<_> = checks.iter().filter(|c| matches!(c.category, IntegrityCategory::MissingInverse)).collect();
        assert_eq!(missing.len(), 1);
        assert!(missing[0].auto_fixable);
    }

    #[test]
    fn no_missing_inverse_when_present() {
        let mut rule = make_node("RULE-001", "rule");
        rule.references_out.push(make_ref("RULE-001", "AD-001", "enforces"));

        let mut decision = make_node("AD-001", "decision");
        decision.references_out.push(make_ref("AD-001", "RULE-001", "enforced-by"));

        let mut nodes = HashMap::new();
        nodes.insert("RULE-001".to_string(), rule);
        nodes.insert("AD-001".to_string(), decision);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let ctx = ctx_with_inverse("enforces", "enforced-by");
        let checks = run_schema_checks(&graph, &ctx);
        assert!(!checks.iter().any(|c| matches!(c.category, IntegrityCategory::MissingInverse)));
    }

    #[test]
    fn type_constraint_violation_detected() {
        let mut task = make_node("TASK-001", "task");
        task.references_out.push(make_ref("TASK-001", "AD-001", "enforces"));

        let decision = make_node("AD-001", "decision");

        let mut nodes = HashMap::new();
        nodes.insert("TASK-001".to_string(), task);
        nodes.insert("AD-001".to_string(), decision);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let ctx = ValidationContext {
            relationships: vec![RelationshipSchema {
                key: "enforces".to_string(),
                inverse: "enforced-by".to_string(),
                description: String::new(),
                from: vec!["rule".to_string()],
                to: vec!["decision".to_string()],
                semantic: None,
                constraints: None,
            }],
            ..empty_ctx()
        };

        let checks = run_schema_checks(&graph, &ctx);
        assert!(checks.iter().any(|c| matches!(c.category, IntegrityCategory::TypeConstraintViolation)));
    }

    #[test]
    fn required_relationship_missing_detected() {
        let rule = make_node("RULE-001", "rule");
        let mut nodes = HashMap::new();
        nodes.insert("RULE-001".to_string(), rule);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let ctx = ValidationContext {
            relationships: vec![RelationshipSchema {
                key: "enforces".to_string(),
                inverse: "enforced-by".to_string(),
                description: String::new(),
                from: vec!["rule".to_string()],
                to: vec!["decision".to_string()],
                semantic: None,
                constraints: Some(RelationshipConstraints {
                    required: Some(true),
                    min_count: Some(1),
                    ..Default::default()
                }),
            }],
            ..empty_ctx()
        };

        let checks = run_schema_checks(&graph, &ctx);
        assert!(checks.iter().any(|c| matches!(c.category, IntegrityCategory::RequiredRelationshipMissing)));
    }

    #[test]
    fn cardinality_violation_detected() {
        let mut node = make_node("TASK-001", "task");
        node.references_out.push(make_ref("TASK-001", "EPIC-001", "delivers"));
        node.references_out.push(make_ref("TASK-001", "EPIC-002", "delivers"));
        node.references_out.push(make_ref("TASK-001", "EPIC-003", "delivers"));

        // Add target nodes so they aren't broken refs.
        let epic1 = make_node("EPIC-001", "epic");
        let epic2 = make_node("EPIC-002", "epic");
        let epic3 = make_node("EPIC-003", "epic");

        let mut nodes = HashMap::new();
        nodes.insert("TASK-001".to_string(), node);
        nodes.insert("EPIC-001".to_string(), epic1);
        nodes.insert("EPIC-002".to_string(), epic2);
        nodes.insert("EPIC-003".to_string(), epic3);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let ctx = ValidationContext {
            relationships: vec![RelationshipSchema {
                key: "delivers".to_string(),
                inverse: "delivered-by".to_string(),
                description: String::new(),
                from: vec![],
                to: vec![],
                semantic: None,
                constraints: Some(RelationshipConstraints {
                    max_count: Some(2),
                    ..Default::default()
                }),
            }],
            ..empty_ctx()
        };

        let checks = run_schema_checks(&graph, &ctx);
        assert!(checks.iter().any(|c| matches!(c.category, IntegrityCategory::CardinalityViolation)));
    }

    #[test]
    fn circular_dependency_detected() {
        let mut a = make_node("TASK-A", "task");
        a.references_out.push(make_ref("TASK-A", "TASK-B", "depends-on"));

        let mut b = make_node("TASK-B", "task");
        b.references_out.push(make_ref("TASK-B", "TASK-A", "depends-on"));

        let mut nodes = HashMap::new();
        nodes.insert("TASK-A".to_string(), a);
        nodes.insert("TASK-B".to_string(), b);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let mut dep_keys = HashSet::new();
        dep_keys.insert("depends-on".to_string());
        dep_keys.insert("depended-on-by".to_string());

        let ctx = ValidationContext {
            relationships: vec![RelationshipSchema {
                key: "depends-on".to_string(),
                inverse: "depended-on-by".to_string(),
                description: String::new(),
                from: vec![],
                to: vec![],
                semantic: Some("dependency".to_string()),
                constraints: None,
            }],
            dependency_keys: dep_keys,
            ..empty_ctx()
        };

        let checks = run_schema_checks(&graph, &ctx);
        assert!(checks.iter().any(|c| matches!(c.category, IntegrityCategory::CircularDependency)));
    }

    #[test]
    fn invalid_status_detected() {
        let mut node = make_node("EPIC-001", "epic");
        node.status = Some("wip".to_string());

        let mut nodes = HashMap::new();
        nodes.insert("EPIC-001".to_string(), node);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let ctx = ctx_with_statuses(vec!["active".to_string(), "completed".to_string()]);
        let checks = run_schema_checks(&graph, &ctx);
        let invalid: Vec<_> = checks.iter().filter(|c| matches!(c.category, IntegrityCategory::InvalidStatus)).collect();
        assert_eq!(invalid.len(), 1);
        assert!(invalid[0].auto_fixable);
    }

    #[test]
    fn valid_status_no_finding() {
        let mut node = make_node("EPIC-001", "epic");
        node.status = Some("active".to_string());

        let mut nodes = HashMap::new();
        nodes.insert("EPIC-001".to_string(), node);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let ctx = ctx_with_statuses(vec!["active".to_string(), "completed".to_string()]);
        let checks = run_schema_checks(&graph, &ctx);
        assert!(!checks.iter().any(|c| matches!(c.category, IntegrityCategory::InvalidStatus)));
    }

    #[test]
    fn body_text_ref_without_relationship_detected() {
        let mut node = make_node("RULE-001", "rule");
        node.references_out.push(ArtifactRef {
            target_id: "AD-001".to_string(),
            field: "body".to_string(),
            source_id: "RULE-001".to_string(),
            relationship_type: None,
        });
        let target = make_node("AD-001", "decision");

        let mut nodes = HashMap::new();
        nodes.insert("RULE-001".to_string(), node);
        nodes.insert("AD-001".to_string(), target);
        let graph = ArtifactGraph { nodes, path_index: HashMap::new() };

        let checks = run_schema_checks(&graph, &empty_ctx());
        assert!(checks.iter().any(|c| matches!(c.category, IntegrityCategory::BodyTextRefWithoutRelationship)));
    }

    #[test]
    fn schema_checks_on_empty_graph() {
        let graph = ArtifactGraph { nodes: HashMap::new(), path_index: HashMap::new() };
        let checks = run_schema_checks(&graph, &empty_ctx());
        assert!(checks.is_empty());
    }

    #[test]
    fn build_context_includes_platform_relationships() {
        let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[], &[]);
        assert!(!ctx.relationships.is_empty());
        assert!(ctx.inverse_map.contains_key("enforces"));
    }
}
