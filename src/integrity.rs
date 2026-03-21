//! Schema-driven integrity checks for the artifact graph.
//!
//! Self-contained subset of `app/backend/src-tauri/src/domain/integrity_engine.rs`
//! adapted for the standalone `orqa-mcp-server` crate. Only the checks used by the
//! MCP server's `graph_validate` tool are included.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::graph::{
    ArtifactGraph, ArtifactNode, IntegrityCategory, IntegrityCheck, IntegritySeverity,
};
use crate::platform::PLATFORM;
use crate::settings::{DeliveryConfig, ProjectRelationshipConfig};

// ---------------------------------------------------------------------------
// Schema types
// ---------------------------------------------------------------------------

/// A status rule constraint from the relationship schema.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusRule {
    pub evaluate: String,
    pub condition: String,
    pub statuses: Vec<String>,
    #[serde(rename = "proposedStatus")]
    pub proposed_status: String,
    pub description: String,
}

/// Constraint block on a relationship definition.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RelationshipConstraints {
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(rename = "minCount", default)]
    pub min_count: Option<usize>,
    #[serde(rename = "maxCount", default)]
    pub max_count: Option<usize>,
    #[serde(rename = "requireInverse", default)]
    pub require_inverse: Option<bool>,
    #[serde(rename = "statusRules", default)]
    pub status_rules: Vec<StatusRule>,
}

/// A relationship schema entry.
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

/// The full validation context.
#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub relationships: Vec<RelationshipSchema>,
    pub inverse_map: HashMap<String, String>,
    pub valid_statuses: Vec<String>,
    pub delivery: DeliveryConfig,
    pub dependency_keys: HashSet<String>,
}

// ---------------------------------------------------------------------------
// Context building
// ---------------------------------------------------------------------------

/// Build a `ValidationContext` from platform config + project relationships.
pub fn build_validation_context(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
) -> ValidationContext {
    let mut relationships: Vec<RelationshipSchema> = Vec::new();
    let mut inverse_map: HashMap<String, String> = HashMap::new();

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

    for pr in project_relationships {
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

    let mut dependency_keys = HashSet::new();
    if let Some(sem) = PLATFORM.semantics.get("dependency") {
        for k in &sem.keys {
            dependency_keys.insert(k.clone());
        }
    }
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
pub fn run_checks(
    graph: &ArtifactGraph,
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
) -> Vec<IntegrityCheck> {
    let ctx = build_validation_context(valid_statuses, delivery, project_relationships);
    run_schema_checks(graph, &ctx)
}

fn run_schema_checks(graph: &ArtifactGraph, ctx: &ValidationContext) -> Vec<IntegrityCheck> {
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
// Individual checks
// ---------------------------------------------------------------------------

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
                continue;
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

fn check_relationship_type_constraints(
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
            let rel_type = match &ref_entry.relationship_type {
                Some(t) => t.as_str(),
                None => continue,
            };

            let Some(schema) = schema_map.get(rel_type) else {
                continue;
            };

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

        if schema.from.is_empty() {
            continue;
        }

        for node in graph.nodes.values() {
            if !schema.from.contains(&node.artifact_type) {
                continue;
            }

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

fn check_cardinality(
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

fn check_circular_dependencies(
    graph: &ArtifactGraph,
    ctx: &ValidationContext,
    checks: &mut Vec<IntegrityCheck>,
) {
    if ctx.dependency_keys.is_empty() {
        return;
    }

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
        return;
    }

    for dtype in &ctx.delivery.types {
        let Some(parent_cfg) = &dtype.parent else {
            continue;
        };
        for node in graph
            .nodes
            .values()
            .filter(|n| n.artifact_type == dtype.key)
        {
            check_single_node_parent_consistency(
                node,
                &parent_cfg.relationship,
                &parent_cfg.parent_type,
                &status_pos,
                graph,
                checks,
            );
        }
    }
}

fn check_single_node_parent_consistency(
    node: &ArtifactNode,
    parent_relationship: &str,
    parent_label: &str,
    status_pos: &HashMap<&str, usize>,
    graph: &ArtifactGraph,
    checks: &mut Vec<IntegrityCheck>,
) {
    let Some(child_status) = node.status.as_deref() else {
        return;
    };
    let Some(&child_pos) = status_pos.get(child_status) else {
        return;
    };
    let parent_ref = node
        .references_out
        .iter()
        .find(|r| r.relationship_type.as_deref() == Some(parent_relationship));
    let Some(parent_ref) = parent_ref else {
        return;
    };
    let Some(parent) = graph.nodes.get(&parent_ref.target_id) else {
        return;
    };
    let Some(parent_status) = &parent.status else {
        return;
    };
    let Some(&parent_pos) = status_pos.get(parent_status.as_str()) else {
        return;
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

        if let Some(parent_cfg) = &dtype.parent {
            let parent_ref = node.references_out.iter().find(|r| {
                r.relationship_type.as_deref() == Some(&parent_cfg.relationship)
                    && graph
                        .nodes
                        .get(&r.target_id)
                        .is_some_and(|n| n.artifact_type == parent_cfg.parent_type)
            });

            if parent_ref.is_none() {
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
            }
        }
    }
}
