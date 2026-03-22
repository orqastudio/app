//! Build a `ValidationContext` from project settings and plugin manifests.

use std::collections::{HashMap, HashSet};

use crate::platform::{ArtifactTypeDef, EnforcementMechanism, SchemaExtension, PLATFORM};
use crate::settings::{DeliveryConfig, ProjectRelationshipConfig};
use crate::types::{RelationshipConstraints, RelationshipSchema, StatusRule, ValidationContext};

/// Build a `ValidationContext` by merging platform config, project relationships,
/// plugin-provided relationship schemas, and plugin-provided artifact type definitions.
///
/// `plugin_artifact_types` is used to populate schema-violation checks (frontmatter
/// requirements and status transitions). Pass an empty slice when plugin manifests
/// have not been scanned.
pub fn build_validation_context(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
) -> ValidationContext {
    build_validation_context_with_types(
        valid_statuses,
        delivery,
        project_relationships,
        plugin_relationships,
        &[],
    )
}

/// Build a `ValidationContext` with full plugin contributions including artifact types
/// and schema extensions.
pub fn build_validation_context_with_types(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
    plugin_artifact_types: &[ArtifactTypeDef],
) -> ValidationContext {
    build_validation_context_full(
        valid_statuses,
        delivery,
        project_relationships,
        plugin_relationships,
        plugin_artifact_types,
        &[],
    )
}

/// Build a `ValidationContext` with all plugin contributions including schema extensions
/// and enforcement mechanisms.
pub fn build_validation_context_full(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
    plugin_artifact_types: &[ArtifactTypeDef],
    plugin_schema_extensions: &[SchemaExtension],
) -> ValidationContext {
    build_validation_context_complete(
        valid_statuses,
        delivery,
        project_relationships,
        plugin_relationships,
        plugin_artifact_types,
        plugin_schema_extensions,
        &[],
    )
}

/// Build a `ValidationContext` with all plugin contributions.
pub fn build_validation_context_complete(
    valid_statuses: &[String],
    delivery: &DeliveryConfig,
    project_relationships: &[ProjectRelationshipConfig],
    plugin_relationships: &[RelationshipSchema],
    plugin_artifact_types: &[ArtifactTypeDef],
    plugin_schema_extensions: &[SchemaExtension],
    plugin_enforcement_mechanisms: &[EnforcementMechanism],
) -> ValidationContext {
    let (mut relationships, mut inverse_map) =
        collect_platform_and_plugin_relationships(plugin_relationships);
    collect_project_relationships(project_relationships, &mut relationships, &mut inverse_map);
    let dependency_keys = collect_dependency_keys(&relationships);

    ValidationContext {
        relationships,
        inverse_map,
        valid_statuses: valid_statuses.to_vec(),
        delivery: delivery.clone(),
        dependency_keys,
        artifact_types: plugin_artifact_types.to_vec(),
        schema_extensions: plugin_schema_extensions.to_vec(),
        enforcement_mechanisms: plugin_enforcement_mechanisms.to_vec(),
    }
}

fn collect_platform_and_plugin_relationships(
    plugin_relationships: &[RelationshipSchema],
) -> (Vec<RelationshipSchema>, HashMap<String, String>) {
    let mut relationships: Vec<RelationshipSchema> = Vec::new();
    let mut inverse_map: HashMap<String, String> = HashMap::new();

    for rel in &PLATFORM.relationships {
        // Convert platform ConstraintsDef to schema RelationshipConstraints if present.
        let constraints = rel.constraints.as_ref().map(|c| RelationshipConstraints {
            required: c.required,
            min_count: c.min_count,
            max_count: c.max_count,
            require_inverse: c.require_inverse,
            status_rules: c
                .status_rules
                .iter()
                .map(|sr| StatusRule {
                    evaluate: sr.evaluate.clone(),
                    condition: sr.condition.clone(),
                    statuses: sr.statuses.clone(),
                    proposed_status: sr.proposed_status.clone(),
                    description: sr.description.clone(),
                })
                .collect(),
        });

        relationships.push(RelationshipSchema {
            key: rel.key.clone(),
            inverse: rel.inverse.clone(),
            description: rel.description.clone(),
            from: rel.from.clone(),
            to: rel.to.clone(),
            semantic: rel.semantic.clone(),
            constraints,
        });
        inverse_map.insert(rel.key.clone(), rel.inverse.clone());
        if rel.inverse != rel.key {
            inverse_map.insert(rel.inverse.clone(), rel.key.clone());
        }
    }

    // Plugin relationships — extend existing definitions or add new ones.
    for pr in plugin_relationships {
        if let Some(existing) = relationships.iter_mut().find(|r| r.key == pr.key) {
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
            if pr.constraints.is_some() && existing.constraints.is_none() {
                existing.constraints.clone_from(&pr.constraints);
            }
        } else {
            relationships.push(pr.clone());
        }
        inverse_map.insert(pr.key.clone(), pr.inverse.clone());
        if pr.inverse != pr.key {
            inverse_map.insert(pr.inverse.clone(), pr.key.clone());
        }
    }

    (relationships, inverse_map)
}

fn collect_project_relationships(
    project_relationships: &[ProjectRelationshipConfig],
    relationships: &mut Vec<RelationshipSchema>,
    inverse_map: &mut HashMap<String, String>,
) {
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
}

fn collect_dependency_keys(relationships: &[RelationshipSchema]) -> HashSet<String> {
    let mut dependency_keys = HashSet::new();
    if let Some(sem) = PLATFORM.semantics.get("dependency") {
        for k in &sem.keys {
            dependency_keys.insert(k.clone());
        }
    }
    for rel in relationships {
        if rel.semantic.as_deref() == Some("dependency") {
            dependency_keys.insert(rel.key.clone());
            dependency_keys.insert(rel.inverse.clone());
        }
    }
    dependency_keys
}
