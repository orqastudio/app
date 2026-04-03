//! Structural integrity checks: broken refs, type constraints, required relationships.

use std::collections::HashMap;

use crate::graph::{ArtifactGraph, ArtifactNode, ArtifactRef};
use crate::platform::ArtifactTypeDef;
use crate::types::{
    IntegrityCategory, IntegrityCheck, IntegritySeverity, RelationshipSchema, ValidationContext,
};

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
            severity: IntegritySeverity::Error,
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

/// Check that explicit `type:` fields match the type implied by the ID prefix.
///
/// Uses plugin-provided artifact type schemas: each schema defines `key` (the type)
/// and `id_prefix` (e.g., "RULE", "TASK"). If an artifact has `id: RULE-abc` and
/// `type: task`, that's a mismatch — the prefix says "rule" but the type says "task".
///
/// Multiple types may share a prefix (e.g., `AD` maps to both `decision` and
/// `discovery-decision` when a stage-scoped type accepts legacy IDs via its
/// JSON Schema pattern). The check also tries multi-segment prefixes
/// (e.g., `DISC-AD`) with longest-match-wins semantics.
pub fn check_type_prefix_mismatch(
    graph: &ArtifactGraph,
    artifact_types: &[ArtifactTypeDef],
    checks: &mut Vec<IntegrityCheck>,
) {
    let prefix_to_types = build_prefix_map(artifact_types);

    for node in graph.nodes.values() {
        let explicit_type = match node.frontmatter.get("type").and_then(|v| v.as_str()) {
            Some(t) if !t.is_empty() => t,
            _ => continue,
        };
        check_node_prefix_mismatch(node, explicit_type, &prefix_to_types, checks);
    }
}

/// Build a mapping from uppercase ID prefix to the list of type keys that use that prefix.
///
/// Also extracts alternation prefixes from JSON Schema id patterns (e.g., `^(IDEA|DISC-IDEA)-`).
fn build_prefix_map(artifact_types: &[ArtifactTypeDef]) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for t in artifact_types {
        map.entry(t.id_prefix.to_uppercase())
            .or_default()
            .push(t.key.clone());
    }
    // Extract alternation prefixes from id JSON Schema patterns.
    for t in artifact_types {
        if let Some(pat) = t
            .frontmatter_schema
            .pointer("/properties/id/pattern")
            .and_then(|v| v.as_str())
        {
            add_schema_pattern_prefixes(pat, &t.key, &mut map);
        }
    }
    map
}

/// Parse a `^(ALT1|ALT2)-` style id pattern and register each alternative prefix for the type.
fn add_schema_pattern_prefixes(pat: &str, type_key: &str, map: &mut HashMap<String, Vec<String>>) {
    let inner = pat.trim_start_matches('^');
    if let Some(rest) = inner.strip_prefix('(') {
        if let Some(group_end) = rest.find(')') {
            for alt in rest[..group_end].split('|') {
                let prefix = alt.trim().to_uppercase();
                let types = map.entry(prefix).or_default();
                if !types.contains(&type_key.to_owned()) {
                    types.push(type_key.to_owned());
                }
            }
        }
    }
}

/// Check a single node for a type/prefix mismatch and push a violation if one is found.
///
/// Uses longest-match-wins: tries prefix candidates from longest to shortest segment count.
fn check_node_prefix_mismatch(
    node: &ArtifactNode,
    explicit_type: &str,
    prefix_to_types: &HashMap<String, Vec<String>>,
    checks: &mut Vec<IntegrityCheck>,
) {
    let id_upper = node.id.to_uppercase();
    let segments: Vec<&str> = id_upper.split('-').collect();

    for len in (1..segments.len()).rev() {
        let candidate = segments[..len].join("-");
        if let Some(valid_types) = prefix_to_types.get(&candidate) {
            let matched = valid_types.iter().any(|t| t == explicit_type);
            if !matched {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::TypePrefixMismatch,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "{} has type: '{}' but ID prefix '{}' implies type '{}' per plugin schema",
                        node.id,
                        explicit_type,
                        candidate,
                        valid_types.join(" or ")
                    ),
                    auto_fixable: true,
                    fix_description: Some(format!(
                        "Change type: {explicit_type} to type: {}",
                        valid_types[0]
                    )),
                });
            }
            break;
        }
    }
}

/// Check that every artifact has a `status:` field in its frontmatter.
///
/// Uses plugin-provided artifact type schemas: if a type's JSON Schema `required`
/// includes "status", then artifacts of that type must have it. Types without
/// "status" in their required fields are automatically excluded.
pub fn check_missing_status_field(
    graph: &ArtifactGraph,
    artifact_types: &[ArtifactTypeDef],
    checks: &mut Vec<IntegrityCheck>,
) {
    // Build set of types that require status from their schema
    let types_requiring_status: std::collections::HashSet<&str> = artifact_types
        .iter()
        .filter(|t| t.frontmatter_required().contains(&"status".to_owned()))
        .map(|t| t.key.as_str())
        .collect();

    for node in graph.nodes.values() {
        // If no schema loaded or type not in schema, skip (don't enforce without schema)
        if artifact_types.is_empty()
            || !types_requiring_status.contains(node.artifact_type.as_str())
        {
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
            severity: IntegritySeverity::Error,
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
        let mut seen: HashMap<(&str, &str), usize> = HashMap::new();

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
                    severity: IntegritySeverity::Error,
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

/// Check that artifact frontmatter contains all required fields for its type.
///
/// Required fields are declared in the plugin manifest's `provides.schemas[].frontmatter.required`
/// array. Only artifact types that declare requirements are checked; types with no schema
/// declaration (e.g. docs) are skipped silently.
pub fn check_frontmatter_requirements(
    graph: &ArtifactGraph,
    artifact_types: &[ArtifactTypeDef],
    checks: &mut Vec<IntegrityCheck>,
) {
    let type_map: HashMap<&str, &ArtifactTypeDef> =
        artifact_types.iter().map(|t| (t.key.as_str(), t)).collect();

    for node in graph.nodes.values() {
        let Some(type_def) = type_map.get(node.artifact_type.as_str()) else {
            continue;
        };

        let required = type_def.frontmatter_required();
        if required.is_empty() {
            continue;
        }

        // Derive the set of present frontmatter keys from the stored JSON value.
        let present_keys: std::collections::HashSet<&str> = node
            .frontmatter
            .as_object()
            .map(|obj| obj.keys().map(String::as_str).collect())
            .unwrap_or_default();

        for required_field in &required {
            if !present_keys.contains(required_field.as_str()) {
                checks.push(IntegrityCheck {
                    category: IntegrityCategory::SchemaViolation,
                    severity: IntegritySeverity::Error,
                    artifact_id: node.id.clone(),
                    message: format!(
                        "Missing required frontmatter field '{}' for type '{}'",
                        required_field, node.artifact_type
                    ),
                    auto_fixable: false,
                    fix_description: None,
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::ArtifactTypeDef;
    use crate::types::{IntegrityCategory, RelationshipConstraints, RelationshipSchema, ValidationContext};
    use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef};
    use std::collections::{HashMap, HashSet};

    fn make_node(id: &str, artifact_type: &str) -> ArtifactNode {
        ArtifactNode {
            id: id.to_owned(),
            project: None,
            path: format!(".orqa/test/{id}.md"),
            artifact_type: artifact_type.to_owned(),
            title: id.to_owned(),
            description: None,
            status: Some("active".to_owned()),
            priority: None,
            frontmatter: serde_json::json!({"type": artifact_type, "status": "active"}),
            body: None,
            references_out: vec![],
            references_in: vec![],
        }
    }

    fn make_ref(source: &str, target: &str, rel_type: &str) -> ArtifactRef {
        ArtifactRef {
            target_id: target.to_owned(),
            field: "relationships".to_owned(),
            source_id: source.to_owned(),
            relationship_type: Some(rel_type.to_owned()),
        }
    }

    fn make_ctx(rel_key: &str, from: &[&str], to: &[&str]) -> ValidationContext {
        let schema = RelationshipSchema {
            key: rel_key.to_owned(),
            inverse: format!("{rel_key}-inverse"),
            description: String::new(),
            from: from.iter().map(|s| s.to_string()).collect(),
            to: to.iter().map(|s| s.to_string()).collect(),
            semantic: None,
            constraints: None,
        };
        ValidationContext {
            relationships: vec![schema],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: Default::default(),
            dependency_keys: HashSet::new(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        }
    }

    // --- check_broken_refs ---

    #[test]
    fn broken_ref_to_missing_node_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.references_out.push(make_ref("TASK-A", "TASK-MISSING", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), node);

        let mut checks = vec![];
        check_broken_refs(&graph, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::BrokenLink);
        assert!(checks[0].message.contains("TASK-MISSING"));
    }

    #[test]
    fn no_broken_refs_when_target_exists() {
        let mut graph = ArtifactGraph::default();
        let mut node_a = make_node("TASK-A", "task");
        let node_b = make_node("EPIC-B", "epic");
        node_a.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("EPIC-B".to_owned(), node_b);

        let mut checks = vec![];
        check_broken_refs(&graph, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn empty_graph_has_no_broken_refs() {
        let graph = ArtifactGraph::default();
        let mut checks = vec![];
        check_broken_refs(&graph, &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_relationship_type_constraints ---

    #[test]
    fn from_type_violation_flagged() {
        // Node is type "task" but schema says only "epic" may use "delivers"
        let mut graph = ArtifactGraph::default();
        let mut node_a = make_node("TASK-A", "task");
        let node_b = make_node("EPIC-B", "epic");
        node_a.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("EPIC-B".to_owned(), node_b);

        let ctx = make_ctx("delivers", &["epic"], &[]);
        let mut checks = vec![];
        check_relationship_type_constraints(&graph, &ctx, &mut checks);
        assert!(!checks.is_empty());
        assert_eq!(checks[0].category, IntegrityCategory::TypeConstraintViolation);
    }

    #[test]
    fn to_type_violation_flagged() {
        // Schema says "delivers" only targets "epic", but target is "task"
        let mut graph = ArtifactGraph::default();
        let mut node_a = make_node("TASK-A", "task");
        let node_b = make_node("TASK-B", "task");
        node_a.references_out.push(make_ref("TASK-A", "TASK-B", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("TASK-B".to_owned(), node_b);

        let ctx = make_ctx("delivers", &[], &["epic"]);
        let mut checks = vec![];
        check_relationship_type_constraints(&graph, &ctx, &mut checks);
        assert!(!checks.is_empty());
        assert_eq!(checks[0].category, IntegrityCategory::TypeConstraintViolation);
    }

    #[test]
    fn valid_type_constraints_produce_no_checks() {
        let mut graph = ArtifactGraph::default();
        let mut node_a = make_node("TASK-A", "task");
        let node_b = make_node("EPIC-B", "epic");
        node_a.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("EPIC-B".to_owned(), node_b);

        let ctx = make_ctx("delivers", &["task"], &["epic"]);
        let mut checks = vec![];
        check_relationship_type_constraints(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_missing_type_field ---

    #[test]
    fn node_without_type_in_frontmatter_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.frontmatter = serde_json::json!({"status": "active"}); // no "type" key
        graph.nodes.insert("TASK-A".to_owned(), node);

        let mut checks = vec![];
        check_missing_type_field(&graph, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::MissingType);
    }

    #[test]
    fn doc_type_is_excluded_from_missing_type_check() {
        // Generic "doc" fallback should not be flagged
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("SOME-DOC", "doc");
        node.frontmatter = serde_json::json!({}); // no type key — but type is "doc"
        graph.nodes.insert("SOME-DOC".to_owned(), node);

        let mut checks = vec![];
        check_missing_type_field(&graph, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn node_with_type_field_is_not_flagged() {
        let mut graph = ArtifactGraph::default();
        graph.nodes.insert("TASK-A".to_owned(), make_node("TASK-A", "task"));
        let mut checks = vec![];
        check_missing_type_field(&graph, &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_duplicate_relationships ---

    #[test]
    fn duplicate_rel_to_same_target_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let mut node_a = make_node("TASK-A", "task");
        let node_b = make_node("EPIC-B", "epic");
        // Two identical relationship edges
        node_a.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        node_a.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("EPIC-B".to_owned(), node_b);

        let mut checks = vec![];
        check_duplicate_relationships(&graph, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::DuplicateRelationship);
    }

    #[test]
    fn different_rel_types_to_same_target_not_duplicate() {
        let mut graph = ArtifactGraph::default();
        let mut node_a = make_node("TASK-A", "task");
        let node_b = make_node("EPIC-B", "epic");
        node_a.references_out.push(make_ref("TASK-A", "EPIC-B", "delivers"));
        node_a.references_out.push(make_ref("TASK-A", "EPIC-B", "implements"));
        graph.nodes.insert("TASK-A".to_owned(), node_a);
        graph.nodes.insert("EPIC-B".to_owned(), node_b);

        let mut checks = vec![];
        check_duplicate_relationships(&graph, &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_filename_matches_id ---

    #[test]
    fn filename_mismatch_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-abc123", "task");
        node.path = ".orqa/implementation/tasks/TASK-001.md".to_owned(); // stem != id
        graph.nodes.insert("TASK-abc123".to_owned(), node);

        let mut checks = vec![];
        check_filename_matches_id(&graph, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::FilenameMismatch);
    }

    #[test]
    fn matching_filename_produces_no_check() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-abc123", "task");
        node.path = ".orqa/implementation/tasks/TASK-abc123.md".to_owned();
        graph.nodes.insert("TASK-abc123".to_owned(), node);

        let mut checks = vec![];
        check_filename_matches_id(&graph, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn qualified_project_prefix_ids_are_skipped_for_filename_check() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("app::RULE-xyz", "rule");
        node.path = ".orqa/rules/RULE-xyz.md".to_owned();
        graph.nodes.insert("app::RULE-xyz".to_owned(), node);

        let mut checks = vec![];
        check_filename_matches_id(&graph, &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_required_relationships ---

    #[test]
    fn required_rel_missing_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-A", "task"); // no outgoing "delivers" rel
        graph.nodes.insert("TASK-A".to_owned(), node);

        let schema = RelationshipSchema {
            key: "delivers".to_owned(),
            inverse: "delivered-by".to_owned(),
            description: String::new(),
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
        let ctx = ValidationContext {
            relationships: vec![schema],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: Default::default(),
            dependency_keys: HashSet::new(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        };
        let mut checks = vec![];
        check_required_relationships(&graph, &ctx, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::RequiredRelationshipMissing);
    }

    #[test]
    fn completed_artifact_skips_required_rel_check() {
        // A "completed" artifact should not be checked for required relationships.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.status = Some("completed".to_owned());
        node.frontmatter = serde_json::json!({"type": "task", "status": "completed"});
        graph.nodes.insert("TASK-A".to_owned(), node);

        let schema = RelationshipSchema {
            key: "delivers".to_owned(),
            inverse: "delivered-by".to_owned(),
            description: String::new(),
            from: vec!["task".to_owned()],
            to: vec![],
            semantic: None,
            constraints: Some(RelationshipConstraints {
                required: Some(true),
                min_count: Some(1),
                max_count: None,
                require_inverse: None,
                status_rules: vec![],
            }),
        };
        let ctx = ValidationContext {
            relationships: vec![schema],
            inverse_map: HashMap::new(),
            valid_statuses: vec![],
            delivery: Default::default(),
            dependency_keys: HashSet::new(),
            artifact_types: vec![],
            schema_extensions: vec![],
            enforcement_mechanisms: vec![],
        };
        let mut checks = vec![];
        check_required_relationships(&graph, &ctx, &mut checks);
        assert!(checks.is_empty());
    }

    // Helper: build a minimal ArtifactTypeDef for tests.
    fn make_type_def(key: &str, id_prefix: &str) -> ArtifactTypeDef {
        ArtifactTypeDef {
            key: key.to_owned(),
            label: key.to_owned(),
            icon: String::new(),
            id_prefix: id_prefix.to_owned(),
            frontmatter_schema: serde_json::json!({}),
            status_transitions: HashMap::new(),
            pipeline_category: None,
        }
    }

    // Helper: build an ArtifactTypeDef that requires specific frontmatter fields.
    fn make_type_def_requiring(key: &str, id_prefix: &str, required: &[&str]) -> ArtifactTypeDef {
        ArtifactTypeDef {
            key: key.to_owned(),
            label: key.to_owned(),
            icon: String::new(),
            id_prefix: id_prefix.to_owned(),
            frontmatter_schema: serde_json::json!({
                "required": required
            }),
            status_transitions: HashMap::new(),
            pipeline_category: None,
        }
    }

    // Helper: build an ArtifactTypeDef with declared status transitions.
    fn make_type_def_with_statuses(
        key: &str,
        id_prefix: &str,
        transitions: &[(&str, &[&str])],
    ) -> ArtifactTypeDef {
        let map: HashMap<String, Vec<String>> = transitions
            .iter()
            .map(|(k, vs)| (k.to_string(), vs.iter().map(|s| s.to_string()).collect()))
            .collect();
        ArtifactTypeDef {
            key: key.to_owned(),
            label: key.to_owned(),
            icon: String::new(),
            id_prefix: id_prefix.to_owned(),
            frontmatter_schema: serde_json::json!({}),
            status_transitions: map,
            pipeline_category: None,
        }
    }

    // --- check_status_transitions ---

    #[test]
    fn invalid_status_is_flagged() {
        // Node has status "unknown" which is not in the declared transitions.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.status = Some("unknown".to_owned());
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def =
            make_type_def_with_statuses("task", "TASK", &[("active", &["completed"]), ("completed", &[])]);
        let mut checks = vec![];
        check_status_transitions(&graph, &[type_def], &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::SchemaViolation);
        assert!(checks[0].message.contains("unknown"));
    }

    #[test]
    fn valid_status_produces_no_check() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.status = Some("active".to_owned());
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def =
            make_type_def_with_statuses("task", "TASK", &[("active", &["completed"]), ("completed", &[])]);
        let mut checks = vec![];
        check_status_transitions(&graph, &[type_def], &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn type_with_no_status_transitions_is_skipped() {
        // Types that declare no transitions should not be checked.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.status = Some("any-status-at-all".to_owned());
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def = make_type_def("task", "TASK"); // no status_transitions
        let mut checks = vec![];
        check_status_transitions(&graph, &[type_def], &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn node_with_no_status_is_skipped_for_status_transitions() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.status = None;
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def =
            make_type_def_with_statuses("task", "TASK", &[("active", &["completed"])]);
        let mut checks = vec![];
        check_status_transitions(&graph, &[type_def], &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_frontmatter_requirements ---

    #[test]
    fn missing_required_frontmatter_field_is_flagged() {
        // Node of type "task" is missing "priority" which the schema requires.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.frontmatter = serde_json::json!({"type": "task", "status": "active"}); // no "priority"
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def = make_type_def_requiring("task", "TASK", &["type", "status", "priority"]);
        let mut checks = vec![];
        check_frontmatter_requirements(&graph, &[type_def], &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::SchemaViolation);
        assert!(checks[0].message.contains("priority"));
    }

    #[test]
    fn all_required_fields_present_produces_no_check() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-A", "task"); // has type + status in frontmatter
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def = make_type_def_requiring("task", "TASK", &["type", "status"]);
        let mut checks = vec![];
        check_frontmatter_requirements(&graph, &[type_def], &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn type_with_no_required_fields_is_skipped() {
        // A type with no `required` schema array should produce no checks.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.frontmatter = serde_json::json!({}); // empty frontmatter
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def = make_type_def("task", "TASK"); // no required fields declared
        let mut checks = vec![];
        check_frontmatter_requirements(&graph, &[type_def], &mut checks);
        assert!(checks.is_empty());
    }

    // --- check_missing_status_field ---

    #[test]
    fn node_missing_status_when_type_requires_it_is_flagged() {
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.frontmatter = serde_json::json!({"type": "task"}); // no "status"
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def = make_type_def_requiring("task", "TASK", &["type", "status"]);
        let mut checks = vec![];
        check_missing_status_field(&graph, &[type_def], &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::MissingStatus);
        assert!(checks[0].message.contains("TASK-A"));
    }

    #[test]
    fn node_with_status_present_produces_no_check() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-A", "task"); // frontmatter has "status": "active"
        graph.nodes.insert("TASK-A".to_owned(), node);

        let type_def = make_type_def_requiring("task", "TASK", &["type", "status"]);
        let mut checks = vec![];
        check_missing_status_field(&graph, &[type_def], &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn empty_artifact_types_skips_status_check() {
        // With no artifact type definitions loaded, status checks are skipped entirely.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-A", "task");
        node.frontmatter = serde_json::json!({}); // no status
        graph.nodes.insert("TASK-A".to_owned(), node);

        let mut checks = vec![];
        check_missing_status_field(&graph, &[], &mut checks); // empty artifact_types
        assert!(checks.is_empty());
    }

    // --- check_type_prefix_mismatch ---

    #[test]
    fn type_mismatch_with_id_prefix_is_flagged() {
        // Node has id "RULE-abc123" (RULE prefix => rule type) but type: "task".
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("RULE-abc123", "rule");
        node.frontmatter = serde_json::json!({"type": "task"}); // type says task but id says rule
        graph.nodes.insert("RULE-abc123".to_owned(), node);

        let type_defs = vec![
            make_type_def("task", "TASK"),
            make_type_def("rule", "RULE"),
        ];
        let mut checks = vec![];
        check_type_prefix_mismatch(&graph, &type_defs, &mut checks);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].category, IntegrityCategory::TypePrefixMismatch);
        assert!(checks[0].message.contains("RULE-abc123"));
    }

    #[test]
    fn matching_type_and_prefix_produces_no_check() {
        let mut graph = ArtifactGraph::default();
        let node = make_node("TASK-abc123", "task"); // frontmatter has type: task, id prefix TASK
        graph.nodes.insert("TASK-abc123".to_owned(), node);

        let type_defs = vec![make_type_def("task", "TASK")];
        let mut checks = vec![];
        check_type_prefix_mismatch(&graph, &type_defs, &mut checks);
        assert!(checks.is_empty());
    }

    #[test]
    fn no_artifact_types_skips_prefix_check() {
        // Without type definitions, no prefix checks can be performed.
        let mut graph = ArtifactGraph::default();
        let mut node = make_node("TASK-abc123", "task");
        node.frontmatter = serde_json::json!({"type": "wrong-type"});
        graph.nodes.insert("TASK-abc123".to_owned(), node);

        let mut checks = vec![];
        check_type_prefix_mismatch(&graph, &[], &mut checks); // no type definitions
        assert!(checks.is_empty());
    }
}

/// Check that artifact status values are valid per the type's declared status transitions.
///
/// Status transitions are declared in the plugin manifest's
/// `provides.schemas[].statusTransitions` map. Only types that declare a transitions
/// map are validated; types with no declared transitions are skipped. An artifact whose
/// current status is not a key in the map is flagged as a warning.
pub fn check_status_transitions(
    graph: &ArtifactGraph,
    artifact_types: &[ArtifactTypeDef],
    checks: &mut Vec<IntegrityCheck>,
) {
    let type_map: HashMap<&str, &ArtifactTypeDef> =
        artifact_types.iter().map(|t| (t.key.as_str(), t)).collect();

    for node in graph.nodes.values() {
        let Some(type_def) = type_map.get(node.artifact_type.as_str()) else {
            continue;
        };

        if type_def.status_transitions.is_empty() {
            continue;
        }

        let Some(status) = &node.status else {
            continue;
        };

        if !type_def.status_transitions.contains_key(status.as_str()) {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::SchemaViolation,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "Status '{}' is not defined in schema transitions for type '{}'",
                    status, node.artifact_type
                ),
                auto_fixable: false,
                fix_description: None,
            });
        }
    }
}

/// Check that the filename (stem, without extension) matches the artifact's `id`.
///
/// The convention is `<ID>.md` — e.g., `EPIC-fb1822c2.md` for id `EPIC-fb1822c2`.
/// Legacy sequential filenames (e.g., `TASK-100.md` with id `TASK-4cfabe07`) are
/// flagged as warnings with an auto-fix suggestion to rename.
pub fn check_filename_matches_id(graph: &ArtifactGraph, checks: &mut Vec<IntegrityCheck>) {
    for node in graph.nodes.values() {
        // Extract filename stem from the path (last component, without .md)
        let path = &node.path;
        let stem = path
            .rsplit('/')
            .next()
            .unwrap_or(path)
            .strip_suffix(".md")
            .unwrap_or(path);

        // Skip qualified project-prefixed keys (e.g., "app::RULE-xyz")
        if node.id.contains("::") {
            continue;
        }

        if stem != node.id {
            checks.push(IntegrityCheck {
                category: IntegrityCategory::FilenameMismatch,
                severity: IntegritySeverity::Error,
                artifact_id: node.id.clone(),
                message: format!(
                    "Filename '{}' does not match id '{}' — expected '{}.md'",
                    stem, node.id, node.id
                ),
                auto_fixable: true,
                fix_description: Some(format!(
                    "Rename file from '{}.md' to '{}.md'",
                    stem, node.id
                )),
            });
        }
    }
}
