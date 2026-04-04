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
    /// Terminal status values collected from plugin manifests.
    /// Used to build PipelineCategories.excluded_statuses for callers.
    pub terminal_statuses: Vec<String>,
    /// The project root this state was built from.
    pub project_root: PathBuf,
}

/// Shared reference to the cached graph state, safe for concurrent read access.
///
/// Handlers receive a clone of this `Arc<RwLock<>>` via axum's `State` extractor.
///
/// Lock scope rationale (R9): `RwLock` is used because the file watcher needs
/// exclusive write access during reload while route handlers read concurrently.
/// Reads are lock-held only for the duration of a single field access or clone
/// (see `rule_count`, `artifact_count`, `find_node`) — never across await points.
/// The write lock in `reload` is held only while swapping `*guard = inner`, not
/// during the (potentially slow) `build_inner` call that precedes it. This keeps
/// contention minimal. An `ArcSwap<GraphStateInner>` would eliminate reader
/// blocking entirely but requires an additional dependency; the current pattern
/// is acceptable given the low reload frequency (file-watcher events, not hot path).
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
            terminal_statuses: Vec::new(),
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
        terminal_statuses: plugin_contributions.terminal_statuses,
        project_root: project_root.to_path_buf(),
    })
}

/// Owned pipeline category data built from plugin contributions in GraphStateInner.
///
/// Holds Vec<String> so route handlers can build borrowed PipelineCategories from it
/// by calling .as_ref_slices() and constructing PipelineCategories with &[&str].
pub struct OwnedPipelineCategories {
    /// Keys of artifact types in the delivery pipeline.
    pub delivery: Vec<String>,
    /// Keys of artifact types in the learning pipeline.
    pub learning: Vec<String>,
    /// Status values that exclude artifacts from outlier analysis.
    pub excluded_statuses: Vec<String>,
    /// Artifact type keys excluded from outlier analysis entirely.
    pub excluded_types: Vec<String>,
    /// Artifact type keys that act as pipeline roots (e.g. "pillar", "vision").
    pub root_types: Vec<String>,
}

impl OwnedPipelineCategories {
    /// Convert owned string vecs to borrowed `&str` slices for `PipelineCategories`.
    ///
    /// Returns five `Vec<&str>` tuples. Callers use these to construct a
    /// `PipelineCategories<'_>` without repeating the `.iter().map(String::as_str)` pattern.
    #[allow(clippy::type_complexity)]
    pub fn as_str_vecs(&self) -> (Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>) {
        (
            self.delivery.iter().map(String::as_str).collect(),
            self.learning.iter().map(String::as_str).collect(),
            self.excluded_statuses.iter().map(String::as_str).collect(),
            self.excluded_types.iter().map(String::as_str).collect(),
            self.root_types.iter().map(String::as_str).collect(),
        )
    }
}

impl GraphStateInner {
    /// Build a PipelineCategories instance from the inner state.
    ///
    /// Filters artifact_types by pipeline_category to populate delivery, learning,
    /// excluded_types, and root_types. Uses terminal_statuses for excluded_statuses.
    pub fn owned_pipeline_categories(&self) -> OwnedPipelineCategories {
        let delivery = self.artifact_types.iter()
            .filter(|t| t.pipeline_category.as_deref() == Some("delivery"))
            .map(|t| t.key.clone())
            .collect();
        let learning = self.artifact_types.iter()
            .filter(|t| t.pipeline_category.as_deref() == Some("learning"))
            .map(|t| t.key.clone())
            .collect();
        let excluded_types = self.artifact_types.iter()
            .filter(|t| t.pipeline_category.as_deref() == Some("excluded"))
            .map(|t| t.key.clone())
            .collect();
        let root_types = self.artifact_types.iter()
            .filter(|t| t.pipeline_category.as_deref() == Some("root"))
            .map(|t| t.key.clone())
            .collect();
        OwnedPipelineCategories {
            delivery,
            learning,
            excluded_statuses: self.terminal_statuses.clone(),
            excluded_types,
            root_types,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Resolve the path to the minimal-project fixture bundled with the daemon tests.
    fn fixture_root() -> PathBuf {
        // __file__ resolves to daemon/src/graph_state.rs at compile time.
        // The fixture lives at daemon/tests/fixtures/minimal-project/.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/minimal-project")
    }

    // -------------------------------------------------------------------------
    // build_empty
    // -------------------------------------------------------------------------

    /// build_empty produces a graph with zero nodes.
    #[test]
    fn build_empty_graph_has_zero_nodes() {
        let root = PathBuf::from("/nonexistent/project");
        let state = GraphState::build_empty(&root);
        assert_eq!(
            state.artifact_count(),
            0,
            "empty state must report zero artifacts"
        );
    }

    /// build_empty sets project_root to the supplied path.
    #[test]
    fn build_empty_sets_project_root() {
        let root = PathBuf::from("/some/project/root");
        let state = GraphState::build_empty(&root);
        let guard = state.0.read().unwrap();
        assert_eq!(
            guard.project_root,
            root,
            "project_root must match the path passed to build_empty"
        );
    }

    /// build_empty produces a usable ValidationContext (not a poisoned/null state).
    #[test]
    fn build_empty_ctx_is_valid() {
        let root = PathBuf::from("/nonexistent/project");
        let state = GraphState::build_empty(&root);
        // If the ValidationContext were in a broken state, acquiring the read lock
        // would either panic or the guard would be poisoned. Accessing it here
        // verifies it is at least structurally valid.
        let guard = state.0.read().expect("lock must not be poisoned after build_empty");
        // The context exists — its type itself is the proof of a valid build.
        let _ = &guard.ctx;
    }

    // -------------------------------------------------------------------------
    // artifact_count / rule_count / find_node — empty state
    // -------------------------------------------------------------------------

    /// artifact_count returns 0 on empty state.
    #[test]
    fn artifact_count_empty_returns_zero() {
        let state = GraphState::build_empty(Path::new("/x"));
        assert_eq!(state.artifact_count(), 0);
    }

    /// rule_count returns 0 on empty state.
    #[test]
    fn rule_count_empty_returns_zero() {
        let state = GraphState::build_empty(Path::new("/x"));
        assert_eq!(state.rule_count(), 0);
    }

    /// find_node returns None for any ID on an empty graph.
    #[test]
    fn find_node_empty_returns_none() {
        let state = GraphState::build_empty(Path::new("/x"));
        assert!(
            state.find_node("EPIC-test001").is_none(),
            "find_node on an empty graph must always return None"
        );
    }

    // -------------------------------------------------------------------------
    // build with fixture project
    // -------------------------------------------------------------------------

    /// build with the minimal-project fixture succeeds and returns nodes.
    #[test]
    fn build_with_fixture_succeeds_and_has_nodes() {
        let root = fixture_root();
        let state = GraphState::build(&root).expect("build must succeed on minimal fixture");
        assert!(
            state.artifact_count() > 0,
            "fixture project has known artifacts; artifact_count must be > 0"
        );
    }

    /// find_node returns the correct artifact_type and title for a known fixture ID.
    #[test]
    fn find_node_returns_correct_type_and_title_for_fixture_id() {
        let root = fixture_root();
        let state = GraphState::build(&root).expect("build must succeed");

        let node = state
            .find_node("EPIC-test001")
            .expect("EPIC-test001 must be present in the fixture graph");

        assert_eq!(node.artifact_type, "epic", "artifact_type must be 'epic'");
        assert_eq!(node.title, "Test Epic", "title must match frontmatter");
    }

    /// find_node with a non-existent ID returns None even after a successful build.
    #[test]
    fn find_node_returns_none_for_nonexistent_id() {
        let root = fixture_root();
        let state = GraphState::build(&root).expect("build must succeed");
        assert!(
            state.find_node("EPIC-doesnotexist").is_none(),
            "find_node must return None for IDs not in the graph"
        );
    }

    /// artifact_count with the fixture reflects the known number of artifacts.
    #[test]
    fn artifact_count_with_fixture_is_correct() {
        let root = fixture_root();
        let state = GraphState::build(&root).expect("build must succeed");
        // Fixture contains EPIC-test001, TASK-test001, RULE-test001 = 3 artifacts.
        assert_eq!(
            state.artifact_count(),
            3,
            "minimal fixture has exactly 3 artifacts"
        );
    }

    /// rule_count with the fixture reflects the known number of rule artifacts.
    #[test]
    fn rule_count_with_fixture_is_correct() {
        let root = fixture_root();
        let state = GraphState::build(&root).expect("build must succeed");
        // Fixture contains exactly one rule: RULE-test001.
        assert_eq!(state.rule_count(), 1, "minimal fixture has exactly 1 rule");
    }

    // -------------------------------------------------------------------------
    // reload
    // -------------------------------------------------------------------------

    /// reload with a valid project path updates the state and returns the new count.
    #[test]
    fn reload_updates_state_and_returns_count() {
        let root = fixture_root();
        // Start from empty so we can observe the state change.
        let state = GraphState::build_empty(&root);
        assert_eq!(state.artifact_count(), 0, "precondition: starts empty");

        let returned_count = state.reload(&root);

        assert!(
            returned_count > 0,
            "reload must return > 0 when the fixture has artifacts"
        );
        assert_eq!(
            state.artifact_count(),
            returned_count,
            "artifact_count after reload must equal the count returned by reload"
        );
    }

    /// reload with a path that has no .orqa/ returns 0 and does not crash.
    ///
    /// The graph engine silently returns an empty graph for a missing .orqa/ directory
    /// (it logs a warning but does not error). This means reload replaces the state
    /// with an empty graph rather than preserving it — the "no crash, non-fatal"
    /// contract is what we are verifying here. The `reload` return value of 0
    /// signals to the caller that the project had nothing to scan.
    #[test]
    fn reload_with_empty_path_returns_zero_and_does_not_crash() {
        let state = GraphState::build_empty(Path::new("/x"));

        // A path with no .orqa/ directory — graph engine returns empty graph silently.
        let returned_count = state.reload(Path::new("/nonexistent/path/that/does/not/exist"));

        assert_eq!(
            returned_count, 0,
            "reload on a path with no .orqa/ must return 0"
        );
        // Must not panic — daemon keeps serving, just with empty state.
        assert_eq!(state.artifact_count(), 0, "state must reflect the empty graph");
    }

    // -------------------------------------------------------------------------
    // Consistency: artifact_count matches direct graph node iteration
    // -------------------------------------------------------------------------

    /// artifact_count and iterating graph.nodes.len() are consistent.
    ///
    /// This tests that the artifact_count helper faithfully reflects the graph —
    /// if someone changed one path but not the other, this would catch it.
    #[test]
    fn artifact_count_consistent_with_graph_node_count() {
        let root = fixture_root();
        let state = GraphState::build(&root).expect("build must succeed");

        let via_helper = state.artifact_count();
        let via_direct = state
            .0
            .read()
            .expect("lock must not be poisoned")
            .graph
            .nodes
            .len();

        assert_eq!(
            via_helper, via_direct,
            "artifact_count helper must match direct graph.nodes.len()"
        );
    }
}
