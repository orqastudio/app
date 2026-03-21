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
pub fn check_parent_child_consistency(
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
