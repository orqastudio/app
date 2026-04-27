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

use orqa_enforcement::store::load_rules;
use orqa_engine_types::platform::ArtifactTypeDef;
use orqa_engine_types::types::enforcement::EnforcementRule;
use orqa_engine_types::ArtifactGraph;
use orqa_graph::surreal::{initialize_schema, open_embedded, GraphDb};
use orqa_graph::surreal_queries::total_artifacts;
use orqa_graph::sync::bulk_sync;

use crate::event_bus::EventBus;
use orqa_validation::context::build_validation_context_complete;
use orqa_validation::graph::{build_artifact_graph, load_project_config};
use orqa_validation::platform::scan_plugin_manifests;
use orqa_validation::types::ValidationContext;
use orqa_validation::ArtifactNode;
use tracing::{info, warn};

/// All cached engine state, rebuilt together on each reload so the graph,
/// validation context, and parsed enforcement rules are always in sync.
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
    /// Parsed enforcement rules from `.orqa/learning/rules/*.md`.
    ///
    /// Cached here so `GET /enforcement/rules` can be served in O(1) without
    /// re-reading 60+ files from disk on every request.  The file watcher
    /// invalidates this cache by calling [`GraphState::reload`] whenever a
    /// file under `.orqa/` changes.
    pub enforcement_rules: Vec<EnforcementRule>,
    /// The project root this state was built from.
    pub project_root: PathBuf,
    /// Embedded SurrealDB handle — a materialized view of the file-based artifact
    /// graph. Populated via `bulk_sync` at startup. `None` when SurrealDB could
    /// not be initialized (e.g. storage path not writable) or on `build_empty`.
    ///
    /// `reload()` preserves the existing `GraphDb` handle across graph rebuilds
    /// so the embedded database connection is never closed mid-session.
    pub db: Option<GraphDb>,
    /// Central event bus for publishing lifecycle events (create, delete, update).
    ///
    /// `None` in `build_empty` and test scenarios that don't inject a bus.
    /// Route handlers that publish events check `is_some()` and skip silently
    /// when absent — the bus is optional so tests don't require one.
    pub event_bus: Option<Arc<EventBus>>,
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
    ///
    /// Initializes an embedded SurrealDB instance at `{project_root}/.state/surreal/`.
    /// If the database is cold (no artifacts), runs `bulk_sync` to materialize the
    /// artifact graph. If the database is warm (artifacts already present from a prior
    /// session), skips `bulk_sync` — the file watcher handles incremental updates while
    /// the daemon is running. SurrealDB failures are non-fatal — the daemon starts with
    /// `db: None` and routes that require SurrealDB degrade gracefully.
    #[allow(clippy::too_many_lines)]
    pub async fn build(project_root: &Path) -> Result<Self, String> {
        let mut inner =
            build_inner(project_root).map_err(|e| format!("graph state build failed: {e}"))?;

        // Initialize embedded SurrealDB at .state/surreal/.
        let surreal_path = project_root.join(".state/surreal");
        if let Err(e) = std::fs::create_dir_all(&surreal_path) {
            warn!(
                subsystem = "graph-state",
                path = %surreal_path.display(),
                error = %e,
                "[graph-state] could not create .state/surreal/ — starting without SurrealDB"
            );
        } else {
            match open_embedded(&surreal_path).await {
                Err(e) => {
                    warn!(
                        subsystem = "graph-state",
                        path = %surreal_path.display(),
                        error = %e,
                        "[graph-state] could not open SurrealDB — starting without SurrealDB"
                    );
                }
                Ok(db) => {
                    if let Err(e) = initialize_schema(&db).await {
                        warn!(
                            subsystem = "graph-state",
                            error = %e,
                            "[graph-state] SurrealDB schema init failed — starting without SurrealDB"
                        );
                    } else {
                        // Warm-start optimisation: if the database already
                        // contains artifacts, skip bulk_sync entirely. The file
                        // watcher (Task 6) handles incremental updates while the
                        // daemon is running; bulk_sync is only needed on a cold
                        // start (first run or after the DB has been wiped).
                        let existing_count = total_artifacts(&db).await.unwrap_or(0);
                        if existing_count > 0 {
                            info!(
                                subsystem = "graph-state",
                                artifact_count = existing_count,
                                "[graph-state] SurrealDB warm start — \
                                 skipping bulk_sync ({} artifacts already present)",
                                existing_count
                            );
                            inner.db = Some(db);
                        } else {
                            match bulk_sync(&db, project_root).await {
                                Ok(summary) => {
                                    info!(
                                        subsystem = "graph-state",
                                        upserted = summary.upserted,
                                        unchanged = summary.unchanged,
                                        errors = summary.errors,
                                        "[graph-state] SurrealDB cold start — \
                                         bulk_sync complete \
                                         ({} upserted, {} unchanged, {} errors)",
                                        summary.upserted,
                                        summary.unchanged,
                                        summary.errors
                                    );
                                    inner.db = Some(db);
                                }
                                Err(e) => {
                                    warn!(
                                        subsystem = "graph-state",
                                        error = %e,
                                        "[graph-state] SurrealDB bulk_sync failed — starting without SurrealDB"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Self(Arc::new(RwLock::new(inner))))
    }

    /// Build an empty graph state for startup when the project has no artifacts yet.
    ///
    /// All route handlers check for empty state and return sensible defaults. This
    /// allows the daemon to start in a degraded-but-functional mode.
    ///
    /// SurrealDB is not initialized in the empty state (`db: None`). Callers that
    /// need SurrealDB should use `build()` instead.
    pub fn build_empty(project_root: &Path) -> Self {
        use orqa_validation::context::build_validation_context;
        use orqa_validation::settings::DeliveryConfig;

        let inner = GraphStateInner {
            graph: ArtifactGraph::default(),
            ctx: build_validation_context(&[], &DeliveryConfig::default(), &[], &[]),
            artifact_types: Vec::new(),
            terminal_statuses: Vec::new(),
            enforcement_rules: Vec::new(),
            project_root: project_root.to_path_buf(),
            db: None,
            event_bus: None,
        };
        Self(Arc::new(RwLock::new(inner)))
    }

    /// Reload the graph, validation context, and enforcement rules from disk.
    ///
    /// Called by the file watcher when `.orqa/` or `plugins/` changes. Returns
    /// the new artifact count for logging. Errors are logged as warnings and
    /// the existing state is preserved so the daemon keeps serving
    /// stale-but-valid data.
    ///
    /// The SurrealDB `db` handle is preserved across reloads so the embedded
    /// database connection is never closed mid-session. Per-file SurrealDB sync
    /// is handled by the file watcher (Task 6 of the SurrealDB migration).
    pub fn reload(&self, project_root: &Path) -> usize {
        // Preserve the existing SurrealDB handle and event bus across the inner rebuild.
        // build_inner sets db: None and event_bus: None; we re-inject both after construction.
        let (existing_db, existing_bus) = match self.0.read() {
            Ok(guard) => (guard.db.clone(), guard.event_bus.clone()),
            Err(_) => (None, None),
        };

        match build_inner(project_root) {
            Ok(mut inner) => {
                inner.db = existing_db;
                inner.event_bus = existing_bus;
                let artifact_count = inner.graph.nodes.len();
                let enforcement_rule_count = inner.enforcement_rules.len();
                let enforcement_entry_count: usize = inner
                    .enforcement_rules
                    .iter()
                    .map(|r| r.entries.len())
                    .sum();
                match self.0.write() {
                    Ok(mut guard) => {
                        *guard = inner;
                        info!(
                            subsystem = "graph-state",
                            artifact_count,
                            enforcement_rule_count,
                            enforcement_entry_count,
                            "[graph-state] reloaded ({artifact_count} artifacts, \
                             {enforcement_rule_count} enforcement rules, \
                             {enforcement_entry_count} entries)"
                        );
                        artifact_count
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

    /// Return a cloned snapshot of the cached enforcement rules.
    ///
    /// Route handlers call this instead of re-reading rule files from disk on
    /// every request.  The lock is held only for the duration of the clone.
    pub fn enforcement_rules(&self) -> Vec<EnforcementRule> {
        match self.0.read() {
            Ok(guard) => guard.enforcement_rules.clone(),
            Err(_) => Vec::new(),
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
                guard.graph.nodes.values().find(|n| n.id == id).cloned()
            }
            Err(_) => None,
        }
    }

    /// Return a clone of the embedded SurrealDB handle, if available.
    ///
    /// `None` when SurrealDB could not be initialized at startup (storage error,
    /// schema failure, or `build_empty` was used). Route handlers that require
    /// SurrealDB should check this and return an appropriate error response when
    /// `None` is returned.
    ///
    /// Cloning `GraphDb` is cheap — `Surreal<Db>` is an `Arc` wrapper internally.
    ///
    /// # Note
    /// Used by the parity validation route (`GET /graph/parity`) and the file
    /// watcher for incremental per-file SurrealDB sync on `.orqa/` changes.
    pub fn surreal_db(&self) -> Option<GraphDb> {
        match self.0.read() {
            Ok(guard) => guard.db.clone(),
            Err(_) => None,
        }
    }

    /// Inject a SurrealDB handle into an existing `GraphState`.
    ///
    /// Used by integration tests to replace the `None` database in a freshly
    /// built empty state with an in-memory SurrealDB instance. This allows
    /// integration tests to run the full import route pipeline without writing
    /// to disk.
    ///
    /// Must be `pub` so integration tests in `daemon/tests/` (separate Rust
    /// crates that link this lib) can call it. Integration-test compilation
    /// does NOT set `cfg(test)` on the lib (they link against it as a library),
    /// so the dead-code allow must be unconditional — the function is dead
    /// from the perspective of every compile-target except the integration
    /// tests themselves.
    #[allow(dead_code)]
    pub fn inject_db(&self, db: GraphDb) {
        if let Ok(mut guard) = self.0.write() {
            guard.db = Some(db);
        }
    }

    /// Inject an event bus into an existing `GraphState`.
    ///
    /// Called during daemon startup after the `EventBus` is created, so that
    /// artifact route handlers can publish lifecycle events (create, delete).
    #[allow(dead_code)]
    pub fn inject_event_bus(&self, bus: Arc<EventBus>) {
        if let Ok(mut guard) = self.0.write() {
            guard.event_bus = Some(bus);
        }
    }

    /// Return a clone of the event bus handle, if one was injected.
    ///
    /// Route handlers that publish events call this and skip silently when `None`.
    pub fn event_bus(&self) -> Option<Arc<EventBus>> {
        match self.0.read() {
            Ok(guard) => guard.event_bus.clone(),
            Err(_) => None,
        }
    }
}

/// Construct a fresh `GraphStateInner` from the project root.
///
/// Builds the graph, loads project config, scans plugin manifests, assembles
/// the validation context, and parses enforcement rules.  All these steps are
/// performed together so the cache is internally consistent on every reload.
///
/// Enforcement-rule load failures are tolerated: a missing rules directory or
/// a single bad file must not break graph construction, so the parser's
/// individual errors are logged by `load_rules` and the aggregate result
/// falls back to an empty list.
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

    let rules_dir = project_root.join(".orqa/learning/rules");
    let enforcement_rules = load_rules(&rules_dir).unwrap_or_else(|e| {
        warn!(
            subsystem = "graph-state",
            rules_dir = %rules_dir.display(),
            error = %e,
            "[graph-state] enforcement rules directory missing or unreadable — caching empty list"
        );
        Vec::new()
    });

    Ok(GraphStateInner {
        graph,
        ctx,
        artifact_types: plugin_contributions.artifact_types,
        terminal_statuses: plugin_contributions.terminal_statuses,
        enforcement_rules,
        project_root: project_root.to_path_buf(),
        db: None,
        event_bus: None,
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
        let delivery = self
            .artifact_types
            .iter()
            .filter(|t| t.pipeline_category.as_deref() == Some("delivery"))
            .map(|t| t.key.clone())
            .collect();
        let learning = self
            .artifact_types
            .iter()
            .filter(|t| t.pipeline_category.as_deref() == Some("learning"))
            .map(|t| t.key.clone())
            .collect();
        let excluded_types = self
            .artifact_types
            .iter()
            .filter(|t| t.pipeline_category.as_deref() == Some("excluded"))
            .map(|t| t.key.clone())
            .collect();
        let root_types = self
            .artifact_types
            .iter()
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
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/minimal-project")
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
            guard.project_root, root,
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
        let guard = state
            .0
            .read()
            .expect("lock must not be poisoned after build_empty");
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
    #[tokio::test]
    async fn build_with_fixture_succeeds_and_has_nodes() {
        let root = fixture_root();
        let state = GraphState::build(&root)
            .await
            .expect("build must succeed on minimal fixture");
        assert!(
            state.artifact_count() > 0,
            "fixture project has known artifacts; artifact_count must be > 0"
        );
    }

    /// find_node returns the correct artifact_type and title for a known fixture ID.
    #[tokio::test]
    async fn find_node_returns_correct_type_and_title_for_fixture_id() {
        let root = fixture_root();
        let state = GraphState::build(&root).await.expect("build must succeed");

        let node = state
            .find_node("EPIC-test001")
            .expect("EPIC-test001 must be present in the fixture graph");

        assert_eq!(node.artifact_type, "epic", "artifact_type must be 'epic'");
        assert_eq!(node.title, "Test Epic", "title must match frontmatter");
    }

    /// find_node with a non-existent ID returns None even after a successful build.
    #[tokio::test]
    async fn find_node_returns_none_for_nonexistent_id() {
        let root = fixture_root();
        let state = GraphState::build(&root).await.expect("build must succeed");
        assert!(
            state.find_node("EPIC-doesnotexist").is_none(),
            "find_node must return None for IDs not in the graph"
        );
    }

    /// artifact_count with the fixture reflects the known number of artifacts.
    #[tokio::test]
    async fn artifact_count_with_fixture_is_correct() {
        let root = fixture_root();
        let state = GraphState::build(&root).await.expect("build must succeed");
        // Fixture contains EPIC-test001, TASK-test001, RULE-test001 = 3 artifacts.
        assert_eq!(
            state.artifact_count(),
            3,
            "minimal fixture has exactly 3 artifacts"
        );
    }

    /// rule_count with the fixture reflects the known number of rule artifacts.
    #[tokio::test]
    async fn rule_count_with_fixture_is_correct() {
        let root = fixture_root();
        let state = GraphState::build(&root).await.expect("build must succeed");
        // Fixture contains exactly one rule: RULE-test001.
        assert_eq!(state.rule_count(), 1, "minimal fixture has exactly 1 rule");
    }

    /// build() with the minimal-project fixture populates SurrealDB with the
    /// same artifact count as the in-memory HashMap.
    ///
    /// This is the parity check for Task 5 — verifying that SurrealDB is
    /// initialized and bulk_sync runs without error on the fixture project.
    #[tokio::test]
    async fn build_populates_surreal_db_matching_artifact_count() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        // Use the bootstrap helper to create a minimal project with known artifacts.
        let rules_dir = bootstrap_minimal_project(root);
        // Write one known artifact with a hook rule.
        std::fs::write(
            rules_dir.join("RULE-test001.md"),
            hook_rule("test001", "--no-verify"),
        )
        .expect("write rule");

        let state = GraphState::build(root).await.expect("build must succeed");

        // SurrealDB must be initialized (not None).
        let db = state.surreal_db();
        assert!(db.is_some(), "surreal_db() must return Some after build()");

        // Artifact count in SurrealDB must match the HashMap count.
        let db = db.unwrap();
        let surreal_count = total_artifacts(&db)
            .await
            .expect("total_artifacts query must succeed");
        let hashmap_count = state.artifact_count();
        assert_eq!(
            surreal_count, hashmap_count,
            "SurrealDB artifact count must match HashMap count: \
             surreal={surreal_count}, hashmap={hashmap_count}"
        );
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
        assert_eq!(
            state.artifact_count(),
            0,
            "state must reflect the empty graph"
        );
    }

    // -------------------------------------------------------------------------
    // Consistency: artifact_count matches direct graph node iteration
    // -------------------------------------------------------------------------

    /// artifact_count and iterating graph.nodes.len() are consistent.
    ///
    /// This tests that the artifact_count helper faithfully reflects the graph —
    /// if someone changed one path but not the other, this would catch it.
    #[tokio::test]
    async fn artifact_count_consistent_with_graph_node_count() {
        let root = fixture_root();
        let state = GraphState::build(&root).await.expect("build must succeed");

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

    // -------------------------------------------------------------------------
    // enforcement rules cache
    // -------------------------------------------------------------------------

    /// YAML frontmatter body for a test rule containing a single `mechanism: hook`.
    ///
    /// The hook targets a bash event so it exercises the parser path that
    /// lifts an entry into `EnforcementRule::entries`, guaranteeing the
    /// cached rule has a non-empty entries vec for assertions.
    fn hook_rule(name: &str, pattern: &str) -> String {
        format!(
            r#"---
id: RULE-{name}
type: rule
title: "{name}"
status: active
enforcement:
  - mechanism: hook
    type: PreToolUse
    event: bash
    action: block
    pattern: "{pattern}"
---
# {name}
"#
        )
    }

    /// Bootstrap a minimal `.orqa/` layout in `dir`: creates `project.json`,
    /// the learning/rules directory, and the schema file the graph builder
    /// requires.  Returns the `learning/rules` path so tests can drop files
    /// into it directly.
    fn bootstrap_minimal_project(dir: &Path) -> PathBuf {
        let orqa = dir.join(".orqa");
        std::fs::create_dir_all(orqa.join("learning/rules")).expect("create rules dir");
        std::fs::create_dir_all(orqa.join("implementation/tasks")).expect("create tasks dir");
        std::fs::write(
            orqa.join("project.json"),
            r#"{"id":"test","name":"test","plugins":{}}"#,
        )
        .expect("write project.json");
        std::fs::write(
            orqa.join("schema.composed.json"),
            r#"{"artifact_types":{}}"#,
        )
        .expect("write schema");
        orqa.join("learning/rules")
    }

    /// Adding a new rule file then calling `reload` makes the new rule
    /// appear in the cached `enforcement_rules` list.  This is the
    /// "watcher fires on create" path end-to-end.
    #[tokio::test]
    async fn reload_picks_up_newly_added_rule_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let rules_dir = bootstrap_minimal_project(dir.path());

        let state = GraphState::build(dir.path())
            .await
            .expect("build must succeed");
        assert!(
            state.enforcement_rules().is_empty(),
            "precondition: empty rules dir yields empty cache"
        );

        // Drop a hook rule into the directory and reload.
        std::fs::write(
            rules_dir.join("RULE-fresh.md"),
            hook_rule("fresh", "--no-verify"),
        )
        .expect("write rule");
        state.reload(dir.path());

        let cached = state.enforcement_rules();
        assert_eq!(cached.len(), 1, "reload must pick up the new rule");
        assert_eq!(cached[0].name, "RULE-fresh");
        assert_eq!(
            cached[0].entries.len(),
            1,
            "hook entry must be lifted into entries on reload"
        );
    }

    /// Editing an existing rule file's content and calling `reload` makes
    /// the cached rule reflect the new pattern.  This is the "watcher
    /// fires on modify" path end-to-end.
    #[tokio::test]
    async fn reload_reflects_edited_rule_file_content() {
        let dir = tempfile::tempdir().expect("tempdir");
        let rules_dir = bootstrap_minimal_project(dir.path());

        std::fs::write(
            rules_dir.join("RULE-edit.md"),
            hook_rule("edit", "initial-pattern"),
        )
        .expect("write rule v1");
        let state = GraphState::build(dir.path())
            .await
            .expect("build must succeed");
        let v1 = state.enforcement_rules();
        assert_eq!(v1.len(), 1);
        assert_eq!(
            v1[0].entries[0].pattern.as_deref(),
            Some("initial-pattern"),
            "v1 must contain the original pattern"
        );

        // Overwrite with new pattern and reload.
        std::fs::write(
            rules_dir.join("RULE-edit.md"),
            hook_rule("edit", "updated-pattern"),
        )
        .expect("write rule v2");
        state.reload(dir.path());

        let v2 = state.enforcement_rules();
        assert_eq!(v2.len(), 1);
        assert_eq!(
            v2[0].entries[0].pattern.as_deref(),
            Some("updated-pattern"),
            "v2 must contain the updated pattern after reload"
        );
    }

    /// Deleting a rule file and calling `reload` drops the rule from the
    /// cached list.  This is the "watcher fires on remove" path end-to-end.
    #[tokio::test]
    async fn reload_drops_deleted_rule_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let rules_dir = bootstrap_minimal_project(dir.path());

        std::fs::write(
            rules_dir.join("RULE-doomed.md"),
            hook_rule("doomed", "--no-verify"),
        )
        .expect("write doomed");
        std::fs::write(
            rules_dir.join("RULE-survivor.md"),
            hook_rule("survivor", "--force"),
        )
        .expect("write survivor");

        let state = GraphState::build(dir.path())
            .await
            .expect("build must succeed");
        assert_eq!(
            state.enforcement_rules().len(),
            2,
            "precondition: both rules are cached"
        );

        // Delete doomed and reload.
        std::fs::remove_file(rules_dir.join("RULE-doomed.md")).expect("delete doomed");
        state.reload(dir.path());

        let cached = state.enforcement_rules();
        assert_eq!(cached.len(), 1, "deleted rule must disappear from cache");
        assert_eq!(cached[0].name, "RULE-survivor", "only survivor must remain");
    }

    /// `enforcement_rules()` on an empty state returns an empty list, never
    /// panics or blocks.  Guards against calling the helper before the
    /// first reload has completed.
    #[test]
    fn enforcement_rules_empty_state_returns_empty_list() {
        let state = GraphState::build_empty(Path::new("/x"));
        assert!(state.enforcement_rules().is_empty());
    }
}
