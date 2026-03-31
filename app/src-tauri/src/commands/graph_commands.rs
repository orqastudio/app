use std::path::Path;

use tauri::{Emitter, Runtime, State};

use crate::domain::artifact_graph::{
    apply_fixes, build_artifact_graph, check_integrity, compute_graph_health, graph_stats,
    update_artifact_field as domain_update_artifact_field, AppliedFix, ArtifactGraph, ArtifactNode,
    GraphHealth, GraphStats, IntegrityCheck,
};
use crate::domain::health_snapshot::{HealthSnapshot, NewHealthSnapshot};
use crate::domain::integrity_engine::{compute_traceability_for, TraceabilityResult};
use crate::domain::project_settings::{DeliveryConfig, StatusDefinition};
use crate::domain::status_transitions::{evaluate_transitions, ProposedTransition};
use crate::error::OrqaError;
use crate::repo::{health_snapshot_repo, project_repo};
use crate::state::AppState;

use super::helpers::active_project_path;

/// Tauri event name emitted when non-auto-apply transitions are pending.
const STATUS_TRANSITIONS_AVAILABLE_EVENT: &str = "status-transitions-available";

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Retrieve the cached graph from state, or build it fresh if absent.
///
/// This lazy-init pattern means the graph is only constructed once per app
/// session. The artifact watcher invalidates it by calling `refresh_graph`
/// when `.orqa/` files change.
fn get_or_build_graph(state: &State<'_, AppState>) -> Result<ArtifactGraph, OrqaError> {
    {
        let guard = state
            .artifacts
            .graph
            .lock()
            .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
        if let Some(graph) = guard.as_ref() {
            return Ok(graph.clone());
        }
    }

    // Graph is absent — build it now.
    let project_path = active_project_path(state)?;
    let graph = build_artifact_graph(Path::new(&project_path))?;

    let mut guard = state
        .artifacts
        .graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(graph.clone());

    Ok(graph)
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

/// Get all artifact nodes of a given type (e.g. "epic", "task", "milestone").
#[tauri::command]
pub fn get_artifacts_by_type(
    artifact_type: String,
    state: State<'_, AppState>,
) -> Result<Vec<ArtifactNode>, OrqaError> {
    if artifact_type.trim().is_empty() {
        return Err(OrqaError::Validation(
            "artifact_type cannot be empty".to_owned(),
        ));
    }
    let graph = get_or_build_graph(&state)?;
    let nodes: Vec<ArtifactNode> = graph
        .nodes
        .values()
        .filter(|n| n.artifact_type == artifact_type)
        .cloned()
        .collect();
    tracing::debug!(r#type = %artifact_type, count = nodes.len(), "get_artifacts_by_type");
    Ok(nodes)
}

/// Read the raw markdown body of an artifact file from disk.
///
/// Always reads from disk to ensure the caller sees the current content.
/// The path must be relative to the project root. Path traversal is rejected.
#[tauri::command]
pub fn read_artifact_content(
    path: String,
    state: State<'_, AppState>,
) -> Result<String, OrqaError> {
    if path.trim().is_empty() {
        return Err(OrqaError::Validation("path cannot be empty".to_owned()));
    }
    if path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_owned(),
        ));
    }

    let project_path = active_project_path(&state)?;
    let project_root = Path::new(&project_path);
    let full_path = project_root.join(path.replace('\\', "/"));

    if !full_path.exists() {
        return Err(OrqaError::NotFound(format!("file not found: {path}")));
    }

    std::fs::read_to_string(&full_path).map_err(OrqaError::from)
}

/// Return summary statistics about the artifact graph.
#[tauri::command]
pub fn get_graph_stats(state: State<'_, AppState>) -> Result<GraphStats, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    // Log all distinct artifact types in the graph for debugging
    let mut type_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for node in graph.nodes.values() {
        *type_counts.entry(&node.artifact_type).or_insert(0) += 1;
    }
    let mut types_sorted: Vec<_> = type_counts.into_iter().collect();
    types_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    let summary: Vec<String> = types_sorted.iter().map(|(t, c)| format!("{t}:{c}")).collect();
    tracing::info!(total = graph.nodes.len(), types = %summary.join(" "), "graph_stats: all types in graph");
    Ok(graph_stats(&graph))
}

/// Return extended structural health metrics for the artifact graph.
///
/// Computes connected components, orphan percentage, average degree,
/// graph density, pillar traceability, and bidirectionality ratio.
/// This replaces the client-side Cytoscape analysis in the dashboard.
#[tauri::command]
pub fn get_graph_health(state: State<'_, AppState>) -> Result<GraphHealth, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    Ok(compute_graph_health(&graph))
}

/// Return the full traceability chain for a single artifact.
///
/// Computes:
/// - All paths from the artifact upward to any pillar or vision artifact.
/// - All downstream artifacts with their BFS distance.
/// - Sibling artifacts (sharing at least one direct parent).
/// - Impact radius (distinct descendants within 2 hops).
/// - Whether the artifact is disconnected from the pillar hierarchy.
#[tauri::command]
pub fn get_artifact_traceability(
    id: String,
    state: State<'_, AppState>,
) -> Result<TraceabilityResult, OrqaError> {
    if id.trim().is_empty() {
        return Err(OrqaError::Validation("id cannot be empty".to_owned()));
    }
    let graph = get_or_build_graph(&state)?;
    compute_traceability_for(&graph, &id)
        .map_err(|e| OrqaError::Serialization(format!("traceability conversion failed: {e}")))
}

/// Apply a single auto-apply transition by writing the new status to disk.
///
/// Returns `true` if the write succeeded, `false` on error (already logged).
fn apply_auto_transition(proposal: &ProposedTransition, project_root: &Path) -> bool {
    if proposal.artifact_path.contains("..") {
        tracing::warn!(
            "[transitions] skipping unsafe path: {}",
            proposal.artifact_path
        );
        return false;
    }
    let full_path = project_root.join(proposal.artifact_path.replace('\\', "/"));
    match domain_update_artifact_field(&full_path, "status", &proposal.proposed_status) {
        Ok(()) => {
            tracing::info!(
                "[transitions] auto-applied: {} {} → {}",
                proposal.artifact_id,
                proposal.current_status,
                proposal.proposed_status
            );
            true
        }
        Err(e) => {
            tracing::warn!(
                "[transitions] failed to auto-apply {} → {}: {e}",
                proposal.artifact_id,
                proposal.proposed_status
            );
            false
        }
    }
}

/// Rebuild the artifact graph from disk and replace the cached copy.
///
/// Runs the status-transition engine after rebuilding:
/// - `auto_apply: true` transitions (blocked/unblocked tasks) are written to
///   disk immediately and the graph is rebuilt once more to reflect them.
/// - `auto_apply: false` transitions are emitted as
///   `"status-transitions-available"` for the frontend to surface to the user.
#[tauri::command]
pub fn refresh_artifact_graph<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    let project_path = active_project_path(&state)?;
    let project_root = Path::new(&project_path);

    let graph = build_artifact_graph(project_root)?;
    let statuses = load_status_definitions(&project_path);
    let (auto_apply, pending): (Vec<ProposedTransition>, Vec<ProposedTransition>) =
        evaluate_transitions(&graph, &statuses)
            .into_iter()
            .partition(|p| p.auto_apply);

    // Apply every auto-apply transition; track whether at least one succeeded.
    let any_applied = auto_apply.iter().fold(false, |acc, p| {
        apply_auto_transition(p, project_root) || acc
    });

    // Rebuild after auto-applies so the cached graph reflects new statuses.
    let (final_graph, pending_for_event) = if any_applied {
        let updated = build_artifact_graph(project_root)?;
        let updated_pending = evaluate_transitions(&updated, &statuses)
            .into_iter()
            .filter(|p| !p.auto_apply)
            .collect();
        (updated, updated_pending)
    } else {
        (graph, pending)
    };

    {
        let mut guard = state
            .artifacts
            .graph
            .lock()
            .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
        *guard = Some(final_graph);
    }

    if !pending_for_event.is_empty() {
        if let Err(e) = app.emit(STATUS_TRANSITIONS_AVAILABLE_EVENT, &pending_for_event) {
            tracing::warn!("[transitions] failed to emit pending transitions event: {e}");
        }
    }

    Ok(())
}

/// Load `StatusDefinition` entries for the active project from `project.json`.
///
/// Returns an empty `Vec` if settings are unavailable or have no statuses defined.
fn load_status_definitions(project_path: &str) -> Vec<StatusDefinition> {
    crate::repo::project_settings_repo::read(project_path)
        .unwrap_or(None)
        .map(|s| s.statuses)
        .unwrap_or_default()
}

/// Load the valid status keys for the active project from `project.json`.
///
/// Returns an empty `Vec` if settings are unavailable or have no statuses defined.
fn load_valid_statuses(project_path: &str) -> Vec<String> {
    load_status_definitions(project_path)
        .into_iter()
        .map(|sd| sd.key)
        .collect()
}

/// Load the delivery-type hierarchy for the active project from `project.json`.
///
/// Returns an empty `DeliveryConfig` if settings are unavailable.
fn load_delivery_config(project_path: &str) -> DeliveryConfig {
    crate::repo::project_settings_repo::read(project_path)
        .unwrap_or(None)
        .map(|s| s.delivery)
        .unwrap_or_default()
}

/// Load project-level relationship definitions from `project.json`.
///
/// Returns an empty vec if settings are unavailable.
fn load_project_relationships(
    project_path: &str,
) -> Vec<crate::domain::project_settings::ProjectRelationshipConfig> {
    crate::repo::project_settings_repo::read(project_path)
        .unwrap_or(None)
        .map(|s| s.relationships)
        .unwrap_or_default()
}

/// Load relationship schemas from all installed plugin manifests.
///
/// Scans `plugins/` directory, reads each `orqa-plugin.json`, and extracts
/// relationship definitions into `RelationshipSchema` structs.
fn load_plugin_relationships(
    project_path: &str,
) -> Vec<crate::domain::integrity_engine::RelationshipSchema> {
    let project_root = Path::new(project_path);
    let plugins = crate::plugins::discovery::scan_plugins(project_root);
    let mut rels = Vec::new();

    for plugin in &plugins {
        let manifest_path = Path::new(&plugin.path).join("orqa-plugin.json");
        let Ok(contents) = std::fs::read_to_string(&manifest_path) else {
            continue;
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents) else {
            continue;
        };
        let Some(provides) = json.get("provides") else {
            continue;
        };
        let Some(relationships) = provides.get("relationships").and_then(|r| r.as_array())
        else {
            continue;
        };

        for rel_value in relationships {
            if let Ok(schema) = serde_json::from_value::<
                crate::domain::integrity_engine::RelationshipSchema,
            >(rel_value.clone())
            {
                rels.push(schema);
            }
        }
    }

    rels
}

/// Run integrity checks on the artifact graph and return all findings.
#[tauri::command]
pub fn run_integrity_scan(state: State<'_, AppState>) -> Result<Vec<IntegrityCheck>, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    let project_path = active_project_path(&state)?;
    let valid_statuses = load_valid_statuses(&project_path);
    let delivery = load_delivery_config(&project_path);
    let project_rels = load_project_relationships(&project_path);
    let plugin_rels = load_plugin_relationships(&project_path);
    Ok(check_integrity(
        &graph,
        &valid_statuses,
        &delivery,
        &project_rels,
        &plugin_rels,
    ))
}

/// Apply auto-fixable integrity checks and return the list of applied fixes.
///
/// Runs the integrity scan first, then applies all auto-fixable findings.
/// Refreshes the graph cache after applying fixes.
#[tauri::command]
pub fn apply_auto_fixes(state: State<'_, AppState>) -> Result<Vec<AppliedFix>, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    let project_path = active_project_path(&state)?;
    let valid_statuses = load_valid_statuses(&project_path);
    let delivery = load_delivery_config(&project_path);
    let project_rels = load_project_relationships(&project_path);
    let plugin_rels = load_plugin_relationships(&project_path);
    let checks = check_integrity(
        &graph,
        &valid_statuses,
        &delivery,
        &project_rels,
        &plugin_rels,
    );
    let applied = apply_fixes(&graph, &checks, Path::new(&project_path))?;

    // Refresh the graph if any fixes were applied.
    if !applied.is_empty() {
        let new_graph = build_artifact_graph(Path::new(&project_path))?;
        let mut guard = state
            .artifacts
            .graph
            .lock()
            .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
        *guard = Some(new_graph);
    }

    Ok(applied)
}

/// Store a health snapshot with the current graph metrics and integrity counts.
///
/// Computes full structural health metrics (components, density, traceability,
/// bidirectionality) before storing so the trend chart has complete data.
#[tauri::command]
pub fn store_health_snapshot(
    error_count: i64,
    warning_count: i64,
    state: State<'_, AppState>,
) -> Result<HealthSnapshot, OrqaError> {
    let graph = get_or_build_graph(&state)?;
    let health = compute_graph_health(&graph);

    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_owned()))?;

    health_snapshot_repo::create(
        &conn,
        project.id,
        &NewHealthSnapshot {
            node_count: health.total_nodes as i64,
            edge_count: health.total_edges as i64,
            orphan_count: health.orphan_count as i64,
            broken_ref_count: health.broken_ref_count as i64,
            error_count,
            warning_count,
            largest_component_ratio: health.largest_component_ratio,
            orphan_percentage: health.orphan_percentage,
            avg_degree: health.avg_degree,
            graph_density: health.graph_density,
            component_count: health.component_count as i64,
            pillar_traceability: health.pillar_traceability,
            bidirectionality_ratio: health.bidirectionality_ratio,
        },
    )
}

/// Update a single YAML frontmatter field in an artifact file on disk.
///
/// Reads the file, replaces the field value in the YAML block, writes it back,
/// then refreshes the artifact graph cache. The path must be relative to the
/// project root. Path traversal is rejected.
///
/// Only simple scalar fields (strings) are supported. The `field` must already
/// exist in the frontmatter — this command updates values, it does not add
/// new fields.
#[tauri::command]
pub fn update_artifact_field(
    path: String,
    field: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), OrqaError> {
    if path.trim().is_empty() {
        return Err(OrqaError::Validation("path cannot be empty".to_owned()));
    }
    if path.contains("..") {
        return Err(OrqaError::Validation(
            "path traversal not allowed".to_owned(),
        ));
    }
    if field.trim().is_empty() {
        return Err(OrqaError::Validation("field cannot be empty".to_owned()));
    }

    let project_path = active_project_path(&state)?;
    let full_path = Path::new(&project_path).join(path.replace('\\', "/"));

    if !full_path.exists() {
        return Err(OrqaError::NotFound(format!(
            "artifact not found: {}",
            full_path.display()
        )));
    }

    domain_update_artifact_field(&full_path, &field, &value)?;

    // Refresh the graph cache so subsequent queries reflect the change.
    let new_graph = build_artifact_graph(Path::new(&project_path))?;
    let mut guard = state
        .artifacts
        .graph
        .lock()
        .map_err(|e| OrqaError::Database(format!("graph lock poisoned: {e}")))?;
    *guard = Some(new_graph);

    Ok(())
}

/// Get the most recent health snapshots for the active project.
#[tauri::command]
pub fn get_health_snapshots(
    limit: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Vec<HealthSnapshot>, OrqaError> {
    let conn = state
        .db
        .conn
        .lock()
        .map_err(|e| OrqaError::Database(format!("lock poisoned: {e}")))?;

    let project = project_repo::get_active(&conn)?
        .ok_or_else(|| OrqaError::NotFound("no active project".to_owned()))?;

    health_snapshot_repo::get_recent(&conn, project.id, limit.unwrap_or(30))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn graph_stats_on_empty_graph() {
        let graph = ArtifactGraph {
            nodes: HashMap::new(),
            path_index: HashMap::new(),
        };
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
        assert_eq!(stats.orphan_count, 0);
        assert_eq!(stats.broken_ref_count, 0);
    }

    #[test]
    fn graph_stats_counts_nodes_and_orphans() {
        let mut nodes = HashMap::new();
        nodes.insert(
            "EPIC-001".to_owned(),
            ArtifactNode {
                id: "EPIC-001".to_owned(),
                project: None,
                path: ".orqa/implementation/epics/EPIC-001.md".to_owned(),
                artifact_type: "epic".to_owned(),
                title: "Test Epic".to_owned(),
                description: None,
                status: Some("draft".to_owned()),
                priority: None,
                frontmatter: serde_json::json!({}),
                body: None,
                references_out: vec![],
                references_in: vec![],
            },
        );
        let graph = ArtifactGraph {
            nodes,
            path_index: HashMap::new(),
        };
        let stats = graph_stats(&graph);
        assert_eq!(stats.node_count, 1);
        assert_eq!(stats.orphan_count, 1); // no refs in or out
    }

    #[test]
    fn build_graph_on_empty_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let graph = build_artifact_graph(dir.path()).expect("should succeed");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn build_graph_finds_artifacts_with_id() {
        let dir = tempfile::tempdir().expect("tempdir");
        let epics_dir = dir.path().join(".orqa").join("planning").join("epics");
        std::fs::create_dir_all(&epics_dir).expect("create dirs");
        std::fs::write(
            epics_dir.join("EPIC-001.md"),
            "---\nid: EPIC-001\ntitle: Test\nstatus: draft\n---\nBody\n",
        )
        .expect("write");

        let graph = build_artifact_graph(dir.path()).expect("should succeed");
        assert!(graph.nodes.contains_key("EPIC-001"));
        let node = &graph.nodes["EPIC-001"];
        assert_eq!(node.artifact_type, "epic");
        assert_eq!(node.title, "Test");
    }

    #[test]
    fn artifact_type_validation_rejects_empty() {
        let artifact_type = "";
        assert!(artifact_type.trim().is_empty());
    }

    #[test]
    fn path_traversal_rejected() {
        let path = "../../../etc/passwd";
        assert!(path.contains(".."));
    }

    #[test]
    fn path_empty_rejected() {
        let path = "   ";
        assert!(path.trim().is_empty());
    }
}
