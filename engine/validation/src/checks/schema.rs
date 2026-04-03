//! JSON Schema (draft 2020-12) validation for artifact frontmatter.
//!
//! Plugin manifests declare each artifact type's frontmatter constraints as a
//! JSON Schema object. This module:
//!
//! 1. Enriches the raw manifest schema with auto-derived `id` and `status` properties
//! 2. Compiles the schema with the `jsonschema` crate
//! 3. Validates artifact frontmatter against the compiled schema
//! 4. Converts validation errors to [`IntegrityCheck`] findings

use std::collections::HashMap;

use crate::graph::ArtifactGraph;
use crate::platform::{ArtifactTypeDef, SchemaExtension};
use crate::types::{IntegrityCategory, IntegrityCheck, IntegritySeverity};

/// A single schema validation error with field path and message.
#[derive(Debug, Clone)]
pub struct SchemaError {
    /// JSON Pointer path to the invalid field (e.g. "/status", "/relationships/0/target").
    pub path: String,
    /// Human-readable error message.
    pub message: String,
}

/// Build a complete JSON Schema for frontmatter validation from an artifact type definition.
///
/// Takes the raw `frontmatter_schema` from the plugin manifest and enriches it with:
/// - Auto-derived `id` property: `{ "type": "string", "pattern": "^{idPrefix}-[a-f0-9]{8}$" }`
/// - Auto-derived `status` property: `{ "type": "string", "enum": [keys of statusTransitions] }`
///
/// Explicit properties in the manifest take precedence over auto-derived ones.
pub fn build_frontmatter_schema(type_def: &ArtifactTypeDef) -> serde_json::Value {
    let mut schema = type_def.frontmatter_schema.clone();

    // Ensure it's an object schema
    if !schema.is_object() {
        schema = serde_json::json!({ "type": "object", "additionalProperties": true });
    }

    let obj = schema.as_object_mut().expect("schema is object");

    // Ensure type: "object" is set
    obj.entry("type")
        .or_insert_with(|| serde_json::json!("object"));

    // Ensure additionalProperties defaults to true
    obj.entry("additionalProperties")
        .or_insert_with(|| serde_json::json!(true));

    // Build or extend the properties object
    let properties = obj
        .entry("properties")
        .or_insert_with(|| serde_json::json!({}));

    if let Some(props) = properties.as_object_mut() {
        // Auto-derive `id` property if not explicitly declared
        if !props.contains_key("id") {
            let pattern = format!("^{}-[a-f0-9]{{8}}$", regex::escape(&type_def.id_prefix));
            props.insert(
                "id".to_owned(),
                serde_json::json!({ "type": "string", "pattern": pattern }),
            );
        }

        // Auto-derive `status` enum from statusTransitions keys if not explicitly declared
        if !props.contains_key("status") && !type_def.status_transitions.is_empty() {
            let mut statuses: Vec<&String> = type_def.status_transitions.keys().collect();
            statuses.sort();
            props.insert(
                "status".to_owned(),
                serde_json::json!({ "type": "string", "enum": statuses }),
            );
        }
    }

    schema
}

/// Validate a frontmatter JSON value against a compiled JSON Schema.
///
/// Returns a list of structured errors. An empty list means the frontmatter is valid.
pub fn validate_frontmatter(
    frontmatter: &serde_json::Value,
    schema: &serde_json::Value,
) -> Vec<SchemaError> {
    let validator = match jsonschema::validator_for(schema) {
        Ok(v) => v,
        Err(e) => {
            return vec![SchemaError {
                path: String::new(),
                message: format!("Failed to compile JSON Schema: {e}"),
            }];
        }
    };

    validator
        .iter_errors(frontmatter)
        .map(|error| {
            let path = error.instance_path.to_string();
            SchemaError {
                path,
                message: error.to_string(),
            }
        })
        .collect()
}

/// Build a composed JSON Schema by merging the base schema with extensions via `allOf`.
///
/// If there are no extensions for this type, returns the base schema unchanged.
/// With extensions, wraps everything in `{ "allOf": [base, ext1, ext2, ...] }`.
pub fn build_composed_schema(
    type_def: &ArtifactTypeDef,
    extensions: &[&SchemaExtension],
) -> serde_json::Value {
    let base = build_frontmatter_schema(type_def);

    if extensions.is_empty() {
        return base;
    }

    let mut all_of = vec![base];
    for ext in extensions {
        all_of.push(ext.frontmatter_schema.clone());
    }

    serde_json::json!({ "allOf": all_of })
}

/// Run JSON Schema validation for all artifacts in the graph against their type schemas.
///
/// Replaces the old `check_frontmatter_requirements` (which only checked field presence).
/// This validates types, patterns, enums, and all other JSON Schema constraints.
pub fn check_frontmatter_schemas(
    graph: &ArtifactGraph,
    artifact_types: &[ArtifactTypeDef],
    checks: &mut Vec<IntegrityCheck>,
) {
    check_frontmatter_schemas_with_extensions(graph, artifact_types, &[], checks);
}

/// Run JSON Schema validation with schema extensions (allOf composition).
pub fn check_frontmatter_schemas_with_extensions(
    graph: &ArtifactGraph,
    artifact_types: &[ArtifactTypeDef],
    extensions: &[SchemaExtension],
    checks: &mut Vec<IntegrityCheck>,
) {
    // Pre-build schemas for each artifact type, keyed by type key.
    let schemas: HashMap<&str, serde_json::Value> = artifact_types
        .iter()
        .map(|t| {
            let type_extensions: Vec<&SchemaExtension> = extensions
                .iter()
                .filter(|e| e.target_key == t.key)
                .collect();
            (t.key.as_str(), build_composed_schema(t, &type_extensions))
        })
        .collect();

    for node in graph.nodes.values() {
        let Some(schema) = schemas.get(node.artifact_type.as_str()) else {
            continue;
        };

        let errors = validate_frontmatter(&node.frontmatter, schema);

        for error in errors {
            let path_info = if error.path.is_empty() {
                String::new()
            } else {
                format!(" at {}", error.path)
            };

            checks.push(IntegrityCheck {
                category: IntegrityCategory::SchemaViolation,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!("Frontmatter schema violation{path_info}: {}", error.message,),
                auto_fixable: false,
                fix_description: None,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_type_def(
        key: &str,
        id_prefix: &str,
        frontmatter_schema: serde_json::Value,
        status_transitions: HashMap<String, Vec<String>>,
    ) -> ArtifactTypeDef {
        ArtifactTypeDef {
            key: key.to_owned(),
            label: key.to_owned(),
            icon: "file".to_owned(),
            id_prefix: id_prefix.to_owned(),
            frontmatter_schema,
            status_transitions,
            pipeline_category: None,
        }
    }

    #[test]
    fn auto_derives_id_pattern() {
        let td = make_type_def("rule", "RULE", serde_json::json!({}), HashMap::new());
        let schema = build_frontmatter_schema(&td);
        let id_prop = &schema["properties"]["id"];
        assert_eq!(id_prop["type"], "string");
        assert_eq!(id_prop["pattern"], "^RULE-[a-f0-9]{8}$");
    }

    #[test]
    fn auto_derives_status_enum() {
        let mut transitions = HashMap::new();
        transitions.insert("active".to_owned(), vec!["archived".to_owned()]);
        transitions.insert("archived".to_owned(), vec![]);
        let td = make_type_def("rule", "RULE", serde_json::json!({}), transitions);
        let schema = build_frontmatter_schema(&td);
        let status_prop = &schema["properties"]["status"];
        assert_eq!(status_prop["type"], "string");
        let enums = status_prop["enum"].as_array().expect("enum array");
        assert!(enums.contains(&serde_json::json!("active")));
        assert!(enums.contains(&serde_json::json!("archived")));
    }

    #[test]
    fn explicit_properties_override_auto_derived() {
        let schema_input = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "string", "pattern": "^CUSTOM-[0-9]+$" }
            }
        });
        let td = make_type_def("custom", "CUSTOM", schema_input, HashMap::new());
        let schema = build_frontmatter_schema(&td);
        // Should keep the explicit pattern, not auto-derive
        assert_eq!(schema["properties"]["id"]["pattern"], "^CUSTOM-[0-9]+$");
    }

    #[test]
    fn validates_valid_frontmatter() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["id", "status"],
            "properties": {
                "id": { "type": "string", "pattern": "^RULE-[a-f0-9]{8}$" },
                "status": { "type": "string", "enum": ["active", "archived"] }
            },
            "additionalProperties": true
        });
        let frontmatter = serde_json::json!({
            "id": "RULE-a1b2c3d4",
            "status": "active",
            "title": "My Rule"
        });
        let errors = validate_frontmatter(&frontmatter, &schema);
        assert!(errors.is_empty(), "Expected no errors, got: {errors:?}");
    }

    #[test]
    fn catches_missing_required_field() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["id", "status"],
            "properties": {
                "id": { "type": "string" },
                "status": { "type": "string" }
            }
        });
        let frontmatter = serde_json::json!({ "id": "RULE-a1b2c3d4" });
        let errors = validate_frontmatter(&frontmatter, &schema);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("required"));
    }

    #[test]
    fn catches_invalid_status_enum() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["id", "status"],
            "properties": {
                "id": { "type": "string" },
                "status": { "type": "string", "enum": ["active", "archived"] }
            }
        });
        let frontmatter = serde_json::json!({ "id": "RULE-a1b2c3d4", "status": "invalid" });
        let errors = validate_frontmatter(&frontmatter, &schema);
        assert_eq!(errors.len(), 1);
        assert!(
            errors[0].message.contains("is not one of"),
            "Error should mention enum constraint: {:?}",
            errors[0].message
        );
    }

    #[test]
    fn catches_wrong_type() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "recurrence": { "type": "integer", "minimum": 0 }
            }
        });
        let frontmatter = serde_json::json!({ "recurrence": "not a number" });
        let errors = validate_frontmatter(&frontmatter, &schema);
        assert!(!errors.is_empty(), "Expected type error");
    }

    #[test]
    fn catches_pattern_mismatch() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["id"],
            "properties": {
                "id": { "type": "string", "pattern": "^RULE-[a-f0-9]{8}$" }
            }
        });
        let frontmatter = serde_json::json!({ "id": "RULE-xyz" });
        let errors = validate_frontmatter(&frontmatter, &schema);
        assert!(!errors.is_empty(), "Expected pattern error");
    }

    #[test]
    fn schema_composition_via_allof() {
        let td = make_type_def(
            "rule",
            "RULE",
            serde_json::json!({
                "type": "object",
                "required": ["id"],
                "properties": {
                    "id": { "type": "string" }
                },
                "additionalProperties": true
            }),
            HashMap::new(),
        );

        let extension = SchemaExtension {
            target_key: "rule".to_owned(),
            frontmatter_schema: serde_json::json!({
                "type": "object",
                "required": ["linter"],
                "properties": {
                    "linter": { "type": "string", "enum": ["eslint", "clippy"] }
                }
            }),
        };

        let schema = build_composed_schema(&td, &[&extension]);

        // Should have allOf
        assert!(schema.get("allOf").is_some(), "Expected allOf composition");

        // Valid: has both id and linter
        let valid = serde_json::json!({ "id": "RULE-a1b2c3d4", "linter": "eslint" });
        let errors = validate_frontmatter(&valid, &schema);
        assert!(errors.is_empty(), "Expected valid: {errors:?}");

        // Invalid: missing linter (required by extension)
        let missing_linter = serde_json::json!({ "id": "RULE-a1b2c3d4" });
        let errors = validate_frontmatter(&missing_linter, &schema);
        assert!(!errors.is_empty(), "Expected error for missing linter");

        // Invalid: wrong linter enum value
        let bad_linter = serde_json::json!({ "id": "RULE-a1b2c3d4", "linter": "invalid" });
        let errors = validate_frontmatter(&bad_linter, &schema);
        assert!(!errors.is_empty(), "Expected error for invalid linter");
    }

    #[test]
    fn allows_additional_properties_by_default() {
        let td = make_type_def(
            "rule",
            "RULE",
            serde_json::json!({ "type": "object", "required": ["id"] }),
            HashMap::new(),
        );
        let schema = build_frontmatter_schema(&td);
        let frontmatter = serde_json::json!({
            "id": "RULE-a1b2c3d4",
            "custom_field": "hello"
        });
        let errors = validate_frontmatter(&frontmatter, &schema);
        assert!(
            errors.is_empty(),
            "Additional properties should be allowed: {errors:?}"
        );
    }
}
