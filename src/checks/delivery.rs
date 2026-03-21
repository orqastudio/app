//! Delivery path completeness checks.

use crate::graph::{ArtifactGraph, ArtifactNode};
use crate::settings::{DeliveryParentConfig, DeliveryTypeConfig};
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity, ValidationContext};

/// Validate delivery artifacts against the `DeliveryConfig`.
pub fn check_delivery_paths(
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
    dtype: &DeliveryTypeConfig,
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
        push_missing_parent_check(node, dtype, parent_cfg, checks);
        return;
    };

    let Some(parent_node) = graph.nodes.get(&parent_ref.target_id) else {
        return; // broken ref, caught by check_broken_refs
    };

    if parent_node.artifact_type != parent_cfg.parent_type {
        push_wrong_parent_type_check(node, parent_cfg, parent_ref.target_id.as_str(), parent_node, checks);
    }
}

fn push_missing_parent_check(
    node: &ArtifactNode,
    dtype: &DeliveryTypeConfig,
    parent_cfg: &DeliveryParentConfig,
    checks: &mut Vec<IntegrityCheck>,
) {
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

fn push_wrong_parent_type_check(
    node: &ArtifactNode,
    parent_cfg: &DeliveryParentConfig,
    parent_ref_target_id: &str,
    parent_node: &ArtifactNode,
    checks: &mut Vec<IntegrityCheck>,
) {
    checks.push(IntegrityCheck {
        category: IntegrityCategory::DeliveryPathMismatch,
        severity: IntegritySeverity::Warning,
        artifact_id: node.id.clone(),
        message: format!(
            "{} has {} relationship to '{}' but {} is a '{}', expected '{}'",
            node.id,
            parent_cfg.relationship,
            parent_ref_target_id,
            parent_ref_target_id,
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
