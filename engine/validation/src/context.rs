//! Build a `ValidationContext` from project settings and plugin manifests.

use std::collections::{HashMap, HashSet};

use crate::platform::{
    ArtifactTypeDef, EnforcementMechanism, RelationshipDef, SchemaExtension, PLATFORM,
};
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

/// Collect all relationship schemas and their inverse mappings from the platform config and plugins.
///
/// Platform relationships are converted from the internal `ConstraintsDef` format to the schema
/// `RelationshipConstraints` type. Plugin relationships extend or supplement platform definitions.
fn collect_platform_and_plugin_relationships(
    plugin_relationships: &[RelationshipSchema],
) -> (Vec<RelationshipSchema>, HashMap<String, String>) {
    let mut relationships: Vec<RelationshipSchema> = Vec::new();
    let mut inverse_map: HashMap<String, String> = HashMap::new();

    for rel in &PLATFORM.relationships {
        relationships.push(platform_rel_to_schema(rel));
        register_inverse(&mut inverse_map, &rel.key, &rel.inverse);
    }

    merge_plugin_relationships(plugin_relationships, &mut relationships, &mut inverse_map);

    (relationships, inverse_map)
}

/// Convert a platform relationship definition to a `RelationshipSchema`.
///
/// Converts `ConstraintsDef` to `RelationshipConstraints` when constraints are present.
fn platform_rel_to_schema(rel: &RelationshipDef) -> RelationshipSchema {
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
    RelationshipSchema {
        key: rel.key.clone(),
        inverse: rel.inverse.clone(),
        description: rel.description.clone(),
        from: rel.from.clone(),
        to: rel.to.clone(),
        semantic: rel.semantic.clone(),
        constraints,
    }
}

/// Merge plugin-provided relationships into the existing relationship list and inverse map.
///
/// Extends existing definitions (from/to types) and adds new ones. Registers inverses for all.
fn merge_plugin_relationships(
    plugin_relationships: &[RelationshipSchema],
    relationships: &mut Vec<RelationshipSchema>,
    inverse_map: &mut HashMap<String, String>,
) {
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
        register_inverse(inverse_map, &pr.key, &pr.inverse);
    }
}

/// Register a relationship key and its inverse in the inverse map.
///
/// Inserts both directions unless the relationship is its own inverse.
fn register_inverse(inverse_map: &mut HashMap<String, String>, key: &str, inverse: &str) {
    inverse_map.insert(key.to_owned(), inverse.to_owned());
    if inverse != key {
        inverse_map.insert(inverse.to_owned(), key.to_owned());
    }
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::DeliveryConfig;

    #[test]
    fn empty_inputs_produce_valid_context() {
        let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[], &[]);
        // Platform relationships are always present from PLATFORM config.
        // We just verify the context is usable (no panic).
        let _ = ctx.relationships.len();
        let _ = ctx.inverse_map.len();
    }

    #[test]
    fn plugin_relationships_extend_platform_relationships() {
        let plugin_rel = RelationshipSchema {
            key: "test-custom-rel".to_owned(),
            inverse: "test-custom-rel-by".to_owned(),
            description: "A test plugin relationship".to_owned(),
            from: vec!["task".to_owned()],
            to: vec!["epic".to_owned()],
            semantic: None,
            constraints: None,
        };
        let ctx = build_validation_context(
            &[],
            &DeliveryConfig::default(),
            &[],
            std::slice::from_ref(&plugin_rel),
        );
        assert!(ctx.relationships.iter().any(|r| r.key == "test-custom-rel"));
    }

    #[test]
    fn plugin_relationship_inverse_is_registered() {
        let plugin_rel = RelationshipSchema {
            key: "test-custom-rel".to_owned(),
            inverse: "test-custom-rel-by".to_owned(),
            description: "Test".to_owned(),
            from: vec![],
            to: vec![],
            semantic: None,
            constraints: None,
        };
        let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[], &[plugin_rel]);
        assert!(ctx.inverse_map.contains_key("test-custom-rel"));
        assert!(ctx.inverse_map.contains_key("test-custom-rel-by"));
    }

    #[test]
    fn plugin_relationship_extends_existing_from_to_types() {
        // Find a platform relationship to extend (pick one that exists)
        let platform_rel_key = PLATFORM.relationships.first().map(|r| r.key.as_str());
        let Some(key) = platform_rel_key else {
            return; // skip if platform is empty
        };
        let inverse = PLATFORM
            .relationships
            .iter()
            .find(|r| r.key == key)
            .map_or("inverse", |r| r.inverse.as_str());

        let plugin_rel = RelationshipSchema {
            key: key.to_owned(),
            inverse: inverse.to_owned(),
            description: "Extended".to_owned(),
            from: vec!["plugin-type".to_owned()],
            to: vec!["plugin-target".to_owned()],
            semantic: None,
            constraints: None,
        };
        let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[], &[plugin_rel]);
        let rel = ctx.relationships.iter().find(|r| r.key == key).unwrap();
        // "plugin-type" should be added to the from list
        assert!(rel.from.contains(&"plugin-type".to_owned()));
        assert!(rel.to.contains(&"plugin-target".to_owned()));
    }

    #[test]
    fn project_relationships_are_added_when_not_already_present() {
        let project_rel = ProjectRelationshipConfig {
            key: "project-custom-link".to_owned(),
            inverse: "project-custom-link-by".to_owned(),
            label: "links".to_owned(),
            inverse_label: "linked by".to_owned(),
        };
        let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[project_rel], &[]);
        assert!(ctx
            .relationships
            .iter()
            .any(|r| r.key == "project-custom-link"));
        assert!(ctx.inverse_map.contains_key("project-custom-link"));
        assert!(ctx.inverse_map.contains_key("project-custom-link-by"));
    }

    #[test]
    fn dependency_keys_include_plugin_dependency_semantics() {
        let dep_rel = RelationshipSchema {
            key: "blocks".to_owned(),
            inverse: "blocked-by".to_owned(),
            description: "Blocks another task".to_owned(),
            from: vec![],
            to: vec![],
            semantic: Some("dependency".to_owned()),
            constraints: None,
        };
        let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[], &[dep_rel]);
        assert!(ctx.dependency_keys.contains("blocks"));
        assert!(ctx.dependency_keys.contains("blocked-by"));
    }

    #[test]
    fn valid_statuses_are_passed_through() {
        let statuses = vec![
            "active".to_owned(),
            "archived".to_owned(),
            "draft".to_owned(),
        ];
        let ctx = build_validation_context(&statuses, &DeliveryConfig::default(), &[], &[]);
        assert_eq!(ctx.valid_statuses, statuses);
    }

    #[test]
    fn self_inverse_relationship_only_registered_once() {
        // A self-inverse rel (key == inverse) should not create two entries
        let self_inv = ProjectRelationshipConfig {
            key: "peer-of".to_owned(),
            inverse: "peer-of".to_owned(),
            label: "peer of".to_owned(),
            inverse_label: "peer of".to_owned(),
        };
        let ctx = build_validation_context(&[], &DeliveryConfig::default(), &[self_inv], &[]);
        // key should appear exactly once in inverse_map
        assert!(ctx.inverse_map.contains_key("peer-of"));
        let entry_count = ctx
            .inverse_map
            .iter()
            .filter(|(k, _)| k.as_str() == "peer-of")
            .count();
        assert_eq!(entry_count, 1);
    }

    #[test]
    fn build_validation_context_with_types_populates_artifact_types() {
        use crate::platform::ArtifactTypeDef;

        let type_def = ArtifactTypeDef {
            key: "task".to_owned(),
            label: "Task".to_owned(),
            icon: "task".to_owned(),
            id_prefix: "TASK".to_owned(),
            frontmatter_schema: serde_json::json!({}),
            status_transitions: HashMap::new(),
            pipeline_category: None,
        };
        let ctx = build_validation_context_with_types(
            &[],
            &DeliveryConfig::default(),
            &[],
            &[],
            &[type_def],
        );
        assert_eq!(ctx.artifact_types.len(), 1);
        assert_eq!(ctx.artifact_types[0].key, "task");
    }

    #[test]
    fn build_validation_context_full_populates_schema_extensions() {
        use crate::platform::SchemaExtension;

        let ext = SchemaExtension {
            target_key: "task".to_owned(),
            frontmatter_schema: serde_json::json!({"properties": {"priority": {"type": "string"}}}),
        };
        let ctx =
            build_validation_context_full(&[], &DeliveryConfig::default(), &[], &[], &[], &[ext]);
        assert_eq!(ctx.schema_extensions.len(), 1);
        assert_eq!(ctx.schema_extensions[0].target_key, "task");
    }

    #[test]
    fn build_validation_context_complete_populates_enforcement_mechanisms() {
        use crate::platform::EnforcementMechanism;

        let mechanism = EnforcementMechanism {
            key: "eslint".to_owned(),
            description: "JavaScript linter".to_owned(),
            strength: 7,
        };
        let ctx = build_validation_context_complete(
            &[],
            &DeliveryConfig::default(),
            &[],
            &[],
            &[],
            &[],
            &[mechanism],
        );
        assert_eq!(ctx.enforcement_mechanisms.len(), 1);
        assert_eq!(ctx.enforcement_mechanisms[0].key, "eslint");
        assert_eq!(ctx.enforcement_mechanisms[0].strength, 7);
    }

    #[test]
    fn plugin_relationship_without_constraints_does_not_override_platform_constraints() {
        // A plugin extending a platform relationship that has constraints should
        // not clear those constraints (only sets constraints when existing has none).
        let plugin_rel = RelationshipSchema {
            key: "test-constrained".to_owned(),
            inverse: "test-constrained-by".to_owned(),
            description: "Test".to_owned(),
            from: vec!["task".to_owned()],
            to: vec!["epic".to_owned()],
            semantic: None,
            constraints: Some(RelationshipConstraints {
                required: Some(true),
                min_count: Some(1),
                max_count: None,
                require_inverse: None,
                status_rules: vec![],
            }),
        };
        // First pass: add it as the base
        let ctx1 = build_validation_context(
            &[],
            &DeliveryConfig::default(),
            &[],
            std::slice::from_ref(&plugin_rel),
        );
        let base = ctx1
            .relationships
            .iter()
            .find(|r| r.key == "test-constrained")
            .unwrap();
        assert!(base.constraints.is_some());

        // Second pass: another plugin adds from/to but no constraints
        let extension = RelationshipSchema {
            key: "test-constrained".to_owned(),
            inverse: "test-constrained-by".to_owned(),
            description: "Extension".to_owned(),
            from: vec!["epic".to_owned()],
            to: vec!["milestone".to_owned()],
            semantic: None,
            constraints: None,
        };
        let ctx2 = build_validation_context(
            &[],
            &DeliveryConfig::default(),
            &[],
            &[plugin_rel, extension],
        );
        let rel = ctx2
            .relationships
            .iter()
            .find(|r| r.key == "test-constrained")
            .unwrap();
        // Constraints should still be present from the first definition
        assert!(rel.constraints.is_some());
        // from/to should be merged
        assert!(rel.from.contains(&"task".to_owned()));
        assert!(rel.from.contains(&"epic".to_owned()));
    }

    #[test]
    fn duplicate_project_relationship_key_is_not_added_again() {
        // If a project relationship key is already in the inverse_map (e.g. from a plugin),
        // it should not be added as a duplicate schema entry.
        let plugin_rel = RelationshipSchema {
            key: "shared-rel".to_owned(),
            inverse: "shared-rel-by".to_owned(),
            description: "Plugin shared".to_owned(),
            from: vec![],
            to: vec![],
            semantic: None,
            constraints: None,
        };
        let project_rel = ProjectRelationshipConfig {
            key: "shared-rel".to_owned(),
            inverse: "shared-rel-by".to_owned(),
            label: "shared".to_owned(),
            inverse_label: "shared by".to_owned(),
        };
        let ctx = build_validation_context(
            &[],
            &DeliveryConfig::default(),
            &[project_rel],
            &[plugin_rel],
        );
        // Should only appear once in relationships list
        let count = ctx
            .relationships
            .iter()
            .filter(|r| r.key == "shared-rel")
            .count();
        assert_eq!(count, 1);
    }
}
