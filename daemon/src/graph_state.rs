// Shared artifact graph and validation context for the daemon.
//
// GraphState holds the cached ArtifactGraph and ValidationContext built from the
// project root. It is wrapped in Arc<RwLock<>> so all route handlers can read
// concurrently while the file watcher reloads in the background.
//
// Design: one reload() builds the graph and validation context together so they
// are always consistent. Callers never see a graph that was built with one
// set of plugins and validated with another.

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use orqa_engine_types::platform::ArtifactTypeDef;
use orqa_engine_types::ArtifactGraph;
use orqa_validation::context::build_validation_context_complete;
use orqa_validation::graph::{build_artifact_graph, load_project_config};
use orqa_validation::platform::scan_plugin_manifests;
use orqa_validation::types::ValidationContext;
use orqa_validation::ArtifactNode;
use tracing::{info, warn};

/// All cached engine state, rebuilt together on each reload so the graph and
/// validation context are always in sync.
pub struct GraphStateInner {
    /// The artifact graph for the project, built from `.orqa/`.
    pub graph: ArtifactGraph,
    /// The validation context derived from plugin manifests and project config.
    pub ctx: ValidationContext,
    /// Artifact type definitions resolved from plugin manifests. Used by the
    /// /plugins endpoint (A4) to expose available types to the frontend.
    #[allow(dead_code)]
    pub artifact_types: Vec<ArtifactTypeDef>,
    /// The project root this state was built from.
    pub project_root: PathBuf,
}

/// Shared reference to the cached graph state, safe for concurrent read access.
///
/// Handlers receive a clone of this `Arc<RwLock<>>` via axum's `State` extractor.
#[derive(Clone)]
pub struct GraphState(pub Arc<RwLock<GraphStateInner>>);

impl GraphState {
    /// Build initial state from the project root.
    ///
    /// Returns an error if the artifact graph cannot be constructed. Validation
    /// context failures are non-fatal — we fall back to a minimal context rather
    /// than refusing to start.
    pub fn build(project_root: &Path) -> Result<Self, String> {
        let inner = build_inner(project_root)
            .map_err(|e| format!("graph state build failed: {e}"))?;
        Ok(Self(Arc::new(RwLock::new(inner))))
    }

    /// Build an empty graph state for startup when the project has no artifacts yet.
    ///
    /// All route handlers check for empty state and return sensible defaults. This
    /// allows the daemon to start in a degraded-but-functional mode.
    pub fn build_empty(project_root: &Path) -> Self {
        use orqa_validation::context::build_validation_context;
        use orqa_validation::settings::DeliveryConfig;

        let inner = GraphStateInner {
            graph: ArtifactGraph::default(),
            ctx: build_validation_context(&[], &DeliveryConfig::default(), &[], &[]),
            artifact_types: Vec::new(),
            project_root: project_root.to_path_buf(),
        };
        Self(Arc::new(RwLock::new(inner)))
    }

    /// Reload the graph and validation context from disk.
    ///
    /// Called by the file watcher when `.orqa/` or `plugins/` changes. Returns the
    /// new artifact count for logging. Errors are logged as warnings and the
    /// existing state is preserved so the daemon keeps serving stale-but-valid data.
    pub fn reload(&self, project_root: &Path) -> usize {
        match build_inner(project_root) {
            Ok(inner) => {
                let count = inner.graph.nodes.len();
                match self.0.write() {
                    Ok(mut guard) => {
                        *guard = inner;
                        info!(
                            subsystem = "graph-state",
                            artifact_count = count,
                            "[graph-state] graph reloaded ({count} artifacts)"
                        );
                        count
                    }
                    Err(e) => {
                        warn!(
                            subsystem = "graph-state",
                            error = %e,
                            "[graph-state] could not acquire write lock during reload — keeping stale state"
                        );
                        0
                    }
                }
            }
            Err(e) => {
                warn!(
                    subsystem = "graph-state",
                    error = %e,
                    "[graph-state] graph reload failed — keeping stale state"
                );
                0
            }
        }
    }

    /// Count artifacts of type "rule" in the cached graph.
    ///
    /// Used by the reload response and health endpoint.
    pub fn rule_count(&self) -> usize {
        match self.0.read() {
            Ok(guard) => guard
                .graph
                .nodes
                .values()
                .filter(|n| n.artifact_type == "rule")
                .count(),
            Err(_) => 0,
        }
    }

    /// Total artifact count from the cached graph.
    pub fn artifact_count(&self) -> usize {
        match self.0.read() {
            Ok(guard) => guard.graph.nodes.len(),
            Err(_) => 0,
        }
    }

    /// Find a single node by exact ID in the cached graph.
    ///
    /// Returns a clone of the node if found, `None` otherwise.
    pub fn find_node(&self, id: &str) -> Option<ArtifactNode> {
        match self.0.read() {
            Ok(guard) => {
                // Try bare ID first, then qualified keys (org mode: "project::ID").
                if let Some(node) = guard.graph.nodes.get(id) {
                    return Some(node.clone());
                }
                guard
                    .graph
                    .nodes
                    .values()
                    .find(|n| n.id == id)
                    .cloned()
            }
            Err(_) => None,
        }
    }
}

/// Construct a fresh `GraphStateInner` from the project root.
///
/// Builds the graph, loads project config, scans plugin manifests, and assembles
/// the validation context. All steps must succeed for a consistent state.
fn build_inner(project_root: &Path) -> Result<GraphStateInner, orqa_validation::ValidationError> {
    let graph = build_artifact_graph(project_root)?;
    let (valid_statuses, delivery, project_relationships) = load_project_config(project_root);
    let plugin_contributions = scan_plugin_manifests(project_root);

    let ctx = build_validation_context_complete(
        &valid_statuses,
        &delivery,
        &project_relationships,
        &plugin_contributions.relationships,
        &plugin_contributions.artifact_types,
        &plugin_contributions.schema_extensions,
        &plugin_contributions.enforcement_mechanisms,
    );

    Ok(GraphStateInner {
        graph,
        ctx,
        artifact_types: plugin_contributions.artifact_types,
        project_root: project_root.to_path_buf(),
    })
}
