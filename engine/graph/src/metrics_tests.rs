//! Unit tests for `engine/graph/src/metrics.rs`.
//!
//! Tests verify the behavioral semantics of each public and private metric function.
//! Every test is designed to catch regressions in the underlying logic, not just
//! confirm that the function returns a struct.
//!
//! Test helper: `GraphBuilder` constructs `ArtifactGraph` values programmatically
//! from typed node descriptors plus an edge list.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use orqa_engine_types::{ArtifactGraph, ArtifactNode, ArtifactRef};

    use crate::metrics::{
        compute_health, compute_traceability, find_siblings, trace_descendants, trace_to_pillars,
    };

    // -------------------------------------------------------------------------
    // Graph builder helpers
    // -------------------------------------------------------------------------

    /// Minimal node descriptor used only by `GraphBuilder`.
    struct NodeSpec<'a> {
        id: &'a str,
        artifact_type: &'a str,
        status: Option<&'a str>,
        /// ISO date string "YYYY-MM-DD", or None.
        created: Option<&'a str>,
    }

    /// Fluent builder that produces an `ArtifactGraph` for testing.
    struct GraphBuilder {
        nodes: HashMap<String, ArtifactNode>,
    }

    impl GraphBuilder {
        fn new() -> Self {
            Self {
                nodes: HashMap::new(),
            }
        }

        /// Add a node with the given id and type. Status defaults to "active".
        fn node(mut self, id: &str, artifact_type: &str) -> Self {
            self.add_node(NodeSpec {
                id,
                artifact_type,
                status: Some("active"),
                created: None,
            });
            self
        }

        /// Add a node with explicit status.
        fn node_with_status(mut self, id: &str, artifact_type: &str, status: &str) -> Self {
            self.add_node(NodeSpec {
                id,
                artifact_type,
                status: Some(status),
                created: None,
            });
            self
        }

        /// Add a node with an explicit created date (ISO "YYYY-MM-DD").
        fn node_with_created(
            mut self,
            id: &str,
            artifact_type: &str,
            created: &str,
        ) -> Self {
            self.add_node(NodeSpec {
                id,
                artifact_type,
                status: Some("active"),
                created: Some(created),
            });
            self
        }

        /// Add a directed edge from `from_id` → `to_id`.
        ///
        /// Also inserts the corresponding `references_in` back-link on the target.
        fn edge(mut self, from_id: &str, to_id: &str) -> Self {
            // Outgoing ref on the source node.
            let out_ref = ArtifactRef {
                target_id: to_id.to_owned(),
                field: "relationships".to_owned(),
                source_id: from_id.to_owned(),
                relationship_type: Some("delivers".to_owned()),
            };
            if let Some(node) = self.nodes.get_mut(from_id) {
                node.references_out.push(out_ref);
            }

            // Back-link on the target node.
            let in_ref = ArtifactRef {
                target_id: to_id.to_owned(),
                field: "relationships".to_owned(),
                source_id: from_id.to_owned(),
                relationship_type: Some("delivers".to_owned()),
            };
            if let Some(node) = self.nodes.get_mut(to_id) {
                node.references_in.push(in_ref);
            }

            self
        }

        /// Add a broken edge from `from_id` to a target that is NOT in the graph.
        fn broken_edge(mut self, from_id: &str, missing_target_id: &str) -> Self {
            let out_ref = ArtifactRef {
                target_id: missing_target_id.to_owned(),
                field: "relationships".to_owned(),
                source_id: from_id.to_owned(),
                relationship_type: Some("delivers".to_owned()),
            };
            if let Some(node) = self.nodes.get_mut(from_id) {
                node.references_out.push(out_ref);
            }
            self
        }

        fn build(self) -> ArtifactGraph {
            ArtifactGraph {
                nodes: self.nodes,
                path_index: HashMap::new(),
            }
        }

        // Private: materialise a NodeSpec into the nodes map.
        fn add_node(&mut self, spec: NodeSpec<'_>) {
            let frontmatter = match spec.created {
                Some(date) => serde_json::json!({ "created": date }),
                None => serde_json::Value::Object(serde_json::Map::new()),
            };

            self.nodes.insert(
                spec.id.to_owned(),
                ArtifactNode {
                    id: spec.id.to_owned(),
                    project: None,
                    path: format!(".orqa/{}.md", spec.id),
                    artifact_type: spec.artifact_type.to_owned(),
                    title: format!("Title of {}", spec.id),
                    description: None,
                    status: spec.status.map(str::to_owned),
                    priority: None,
                    frontmatter,
                    body: None,
                    references_out: Vec::new(),
                    references_in: Vec::new(),
                },
            );
        }
    }

    // -------------------------------------------------------------------------
    // compute_health — empty graph
    // -------------------------------------------------------------------------

    #[test]
    fn health_empty_graph_returns_all_zeros() {
        // An empty graph should yield a fully-zeroed GraphHealth (the Default).
        let graph = GraphBuilder::new().build();
        let health = compute_health(&graph);
        assert_eq!(health.total_nodes, 0);
        assert_eq!(health.total_edges, 0);
        assert_eq!(health.outlier_count, 0);
        assert_eq!(health.broken_ref_count, 0);
        assert_eq!(health.delivery_connectivity, 0.0);
        assert_eq!(health.learning_connectivity, 0.0);
        assert_eq!(health.avg_degree, 0.0);
    }

    // -------------------------------------------------------------------------
    // compute_health — single isolated node
    // -------------------------------------------------------------------------

    #[test]
    fn health_single_isolated_node_has_no_edges() {
        // A single node with no relationships has 0 edges and avg_degree 0.
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let health = compute_health(&graph);
        assert_eq!(health.total_nodes, 1);
        assert_eq!(health.total_edges, 0);
        assert_eq!(health.avg_degree, 0.0);
        assert_eq!(health.largest_component_ratio, 1.0, "one node = 100% in its own component");
    }

    // -------------------------------------------------------------------------
    // compute_health — delivery pipeline connectivity
    // -------------------------------------------------------------------------

    #[test]
    fn health_delivery_pipeline_connected() {
        // pillar→epic→task: all three are in one delivery component.
        // Delivery connectivity should be 1.0 (2 out of 2 delivery nodes reachable — pillar is not
        // in DELIVERY_TYPES, epic+task are).
        let graph = GraphBuilder::new()
            .node("PIL-1", "pillar")
            .node("EP-1", "epic")
            .node("T-1", "task")
            .edge("T-1", "EP-1")
            .edge("EP-1", "PIL-1")
            .build();
        let health = compute_health(&graph);
        // All delivery-type nodes (epic + task) are connected → 1.0
        assert_eq!(health.delivery_connectivity, 1.0,
            "fully connected delivery nodes should have connectivity 1.0");
    }

    #[test]
    fn health_delivery_pipeline_partial_connectivity() {
        // Two tasks: T-1 connected to an epic, T-2 isolated. Half-connected.
        let graph = GraphBuilder::new()
            .node("EP-1", "epic")
            .node("T-1", "task")
            .node("T-2", "task")
            .edge("T-1", "EP-1")
            .build();
        let health = compute_health(&graph);
        // 3 delivery nodes (EP-1, T-1, T-2). EP-1 and T-1 form one component of
        // size 2, T-2 is isolated. Largest component = 2, total = 3 → ~0.667.
        assert!(
            health.delivery_connectivity > 0.5 && health.delivery_connectivity < 1.0,
            "partial delivery connectivity expected ~0.667, got {}",
            health.delivery_connectivity
        );
    }

    #[test]
    fn health_no_delivery_nodes_yields_zero_delivery_connectivity() {
        // A graph with only learning-type nodes has no delivery connectivity.
        let graph = GraphBuilder::new()
            .node("LES-1", "lesson")
            .node("RULE-1", "rule")
            .edge("LES-1", "RULE-1")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.delivery_connectivity, 0.0);
    }

    // -------------------------------------------------------------------------
    // compute_health — learning pipeline connectivity
    // -------------------------------------------------------------------------

    #[test]
    fn health_learning_pipeline_connected() {
        // lesson connected to decision: lesson is "connected" and rule is connected to lesson.
        let graph = GraphBuilder::new()
            .node("DEC-1", "decision")
            .node("LES-1", "lesson")
            .node("RULE-1", "rule")
            .edge("LES-1", "DEC-1")
            .edge("RULE-1", "LES-1")
            .build();
        let health = compute_health(&graph);
        // Both lesson and rule have connections to learning/decision → 1.0
        assert_eq!(health.learning_connectivity, 1.0,
            "all learning nodes should be connected");
    }

    #[test]
    fn health_no_learning_nodes_yields_zero() {
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let health = compute_health(&graph);
        assert_eq!(health.learning_connectivity, 0.0);
    }

    // -------------------------------------------------------------------------
    // compute_health — disconnected components
    // -------------------------------------------------------------------------

    #[test]
    fn health_three_disconnected_components_reflected_in_ratio() {
        // Three isolated nodes = three components. Largest is size 1 → ratio = 1/3.
        let graph = GraphBuilder::new()
            .node("A", "task")
            .node("B", "task")
            .node("C", "task")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.total_nodes, 3);
        let expected = 1.0 / 3.0;
        assert!(
            (health.largest_component_ratio - expected).abs() < 1e-9,
            "expected largest_component_ratio ≈ 0.333, got {}",
            health.largest_component_ratio
        );
    }

    #[test]
    fn health_two_components_largest_ratio_is_correct() {
        // 4 nodes: A-B connected, C-D connected.  Largest = 2, total = 4 → 0.5.
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .node("C", "epic")
            .node("D", "task")
            .edge("A", "B")
            .edge("C", "D")
            .build();
        let health = compute_health(&graph);
        assert!(
            (health.largest_component_ratio - 0.5).abs() < 1e-9,
            "two equal components → ratio 0.5, got {}",
            health.largest_component_ratio
        );
    }

    // -------------------------------------------------------------------------
    // compute_health — outlier detection
    // -------------------------------------------------------------------------

    /// Returns today's date as "YYYY-MM-DD". Used to create nodes that appear fresh.
    fn today_date_str() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let days = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            / 86400;
        // Days-since-epoch → Gregorian via the proleptic civil-calendar inverse.
        let z = days + 719468;
        let era = z.div_euclid(146097);
        let doe = z.rem_euclid(146097);
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        format!("{:04}-{:02}-{:02}", y, m, d)
    }

    #[test]
    fn health_outlier_type_outside_both_pipelines_old_node_is_counted() {
        // "pillar" is neither a delivery nor a learning type, and has no `created` date
        // (treated as stale → past grace). Should be counted as an outlier.
        let graph = GraphBuilder::new().node("PIL-1", "pillar").build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 1,
            "pillar with no created date must be counted as an outlier");
    }

    #[test]
    fn health_fresh_node_within_grace_period_not_counted_as_outlier() {
        // A "pillar" node created today should NOT exceed the 30-day grace period.
        let today = today_date_str();
        let graph = GraphBuilder::new()
            .node_with_created("PIL-1", "pillar", &today)
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 0,
            "a node created today is within the grace period and should not be an outlier");
    }

    #[test]
    fn health_excluded_type_knowledge_not_counted_as_outlier() {
        // "knowledge" type is explicitly excluded from outlier analysis.
        let graph = GraphBuilder::new().node("KNOW-1", "knowledge").build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 0,
            "knowledge type must be excluded from outlier analysis");
    }

    #[test]
    fn health_excluded_type_doc_not_counted_as_outlier() {
        // "doc" type is explicitly excluded from outlier analysis.
        let graph = GraphBuilder::new().node("DOC-1", "doc").build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 0,
            "doc type must be excluded from outlier analysis");
    }

    #[test]
    fn health_archived_status_not_counted_as_outlier() {
        // Archived artifacts are excluded regardless of type or age.
        let graph = GraphBuilder::new()
            .node_with_status("PIL-1", "pillar", "archived")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 0,
            "archived artifact must not be an outlier");
    }

    #[test]
    fn health_surpassed_status_not_counted_as_outlier() {
        let graph = GraphBuilder::new()
            .node_with_status("PIL-1", "pillar", "surpassed")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 0,
            "surpassed artifact must not be an outlier");
    }

    #[test]
    fn health_delivery_type_task_not_an_outlier() {
        // "task" belongs to the delivery pipeline, so it must never be an outlier.
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 0,
            "task is in the delivery pipeline and must never be an outlier");
    }

    #[test]
    fn health_learning_type_lesson_not_an_outlier() {
        // "lesson" is in the learning pipeline — never an outlier.
        let graph = GraphBuilder::new().node("LES-1", "lesson").build();
        let health = compute_health(&graph);
        assert_eq!(health.outlier_count, 0,
            "lesson is in the learning pipeline and must never be an outlier");
    }

    // -------------------------------------------------------------------------
    // compute_health — broken references
    // -------------------------------------------------------------------------

    #[test]
    fn health_broken_ref_count_increments_for_missing_target() {
        // T-1 has an outgoing edge to MISSING-999 which is not in the graph.
        let graph = GraphBuilder::new()
            .node("T-1", "task")
            .broken_edge("T-1", "MISSING-999")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.broken_ref_count, 1,
            "one edge pointing to a non-existent target must be counted as a broken ref");
    }

    #[test]
    fn health_no_broken_refs_when_all_targets_exist() {
        let graph = GraphBuilder::new()
            .node("T-1", "task")
            .node("EP-1", "epic")
            .edge("T-1", "EP-1")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.broken_ref_count, 0);
    }

    #[test]
    fn health_two_broken_refs_out_of_three_edges() {
        let graph = GraphBuilder::new()
            .node("T-1", "task")
            .node("EP-1", "epic")
            .edge("T-1", "EP-1")
            .broken_edge("T-1", "GONE-1")
            .broken_edge("T-1", "GONE-2")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.broken_ref_count, 2);
    }

    // -------------------------------------------------------------------------
    // compute_health — edge / degree counts
    // -------------------------------------------------------------------------

    #[test]
    fn health_total_edges_counts_directed_outgoing() {
        // A→B, A→C: 2 outgoing edges from A.
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .node("C", "task")
            .edge("A", "B")
            .edge("A", "C")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.total_edges, 2);
    }

    #[test]
    fn health_avg_degree_is_total_degree_over_node_count() {
        // A→B: A has out=1, in=0; B has out=0, in=1. Total degree = 2. Nodes = 2. Avg = 1.0.
        let graph = GraphBuilder::new()
            .node("A", "task")
            .node("B", "epic")
            .edge("A", "B")
            .build();
        let health = compute_health(&graph);
        assert!(
            (health.avg_degree - 1.0).abs() < 1e-9,
            "avg_degree should be 1.0 for a single edge between 2 nodes, got {}",
            health.avg_degree
        );
    }

    // -------------------------------------------------------------------------
    // compute_health — pillar traceability
    // -------------------------------------------------------------------------

    #[test]
    fn health_pillar_traceability_full_pipeline_reaches_100() {
        // task→epic→pillar: task and epic (non-doc) can both trace back to pillar → 100%.
        let graph = GraphBuilder::new()
            .node("PIL-1", "pillar")
            .node("EP-1", "epic")
            .node("T-1", "task")
            .edge("T-1", "EP-1")
            .edge("EP-1", "PIL-1")
            .build();
        let health = compute_health(&graph);
        assert!(
            (health.pillar_traceability - 100.0).abs() < 1e-9,
            "all non-doc nodes can trace to pillar, expected 100.0%, got {}",
            health.pillar_traceability
        );
    }

    #[test]
    fn health_pillar_traceability_zero_with_no_pillars() {
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let health = compute_health(&graph);
        assert_eq!(health.pillar_traceability, 0.0,
            "no pillars in graph → traceability must be 0");
    }

    // -------------------------------------------------------------------------
    // compute_traceability — artifact not in graph
    // -------------------------------------------------------------------------

    #[test]
    fn traceability_unknown_artifact_returns_empty_result() {
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let result = compute_traceability(&graph, "NONEXISTENT-999");
        assert!(result.ancestry_chains.is_empty(),
            "unknown artifact must have no ancestry chains");
        assert!(result.descendants.is_empty(),
            "unknown artifact must have no descendants");
        assert!(result.siblings.is_empty(),
            "unknown artifact must have no siblings");
        assert_eq!(result.impact_radius, 0);
    }

    // -------------------------------------------------------------------------
    // compute_traceability — isolated artifact
    // -------------------------------------------------------------------------

    #[test]
    fn traceability_isolated_artifact_is_disconnected() {
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let result = compute_traceability(&graph, "T-1");
        assert!(result.disconnected,
            "task with no relationships should be marked disconnected");
        assert!(result.descendants.is_empty());
        assert!(result.siblings.is_empty());
    }

    // -------------------------------------------------------------------------
    // trace_to_pillars
    // -------------------------------------------------------------------------

    #[test]
    fn trace_to_pillars_linear_chain_from_task_finds_pillar() {
        // task → epic → pillar: tracing from task should return a chain ending at pillar.
        let graph = GraphBuilder::new()
            .node("PIL-1", "pillar")
            .node("EP-1", "epic")
            .node("T-1", "task")
            .edge("T-1", "EP-1")
            .edge("EP-1", "PIL-1")
            .build();
        let chains = trace_to_pillars(&graph, "T-1");
        assert_eq!(chains.len(), 1, "exactly one ancestry chain expected");
        let path = &chains[0].path;
        // Path: T-1 (index 0) → EP-1 → PIL-1 (last)
        assert_eq!(path[0].id, "T-1");
        assert_eq!(path.last().unwrap().id, "PIL-1");
        assert_eq!(path.last().unwrap().artifact_type, "pillar");
    }

    #[test]
    fn trace_to_pillars_isolated_task_returns_empty() {
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let chains = trace_to_pillars(&graph, "T-1");
        assert!(chains.is_empty(),
            "isolated task has no path to any pillar");
    }

    #[test]
    fn trace_to_pillars_ancestry_includes_intermediate_nodes() {
        // task → epic → pillar: the epic must appear in the ancestry path.
        let graph = GraphBuilder::new()
            .node("PIL-1", "pillar")
            .node("EP-1", "epic")
            .node("T-1", "task")
            .edge("T-1", "EP-1")
            .edge("EP-1", "PIL-1")
            .build();
        let chains = trace_to_pillars(&graph, "T-1");
        let path = &chains[0].path;
        let ids: Vec<&str> = path.iter().map(|n| n.id.as_str()).collect();
        assert!(ids.contains(&"EP-1"),
            "epic must appear in the ancestry path from task to pillar");
    }

    #[test]
    fn trace_to_pillars_direct_pillar_query() {
        // Querying a pillar directly: should return a trivial single-node chain.
        let graph = GraphBuilder::new().node("PIL-1", "pillar").build();
        let chains = trace_to_pillars(&graph, "PIL-1");
        // Single-node chain: the pillar itself.
        assert!(!chains.is_empty(),
            "querying a pillar should return a (trivial) chain");
    }

    #[test]
    fn trace_to_pillars_cycle_does_not_infinite_loop() {
        // A→B→A circular reference: the DFS cycle guard must prevent infinite recursion.
        let mut builder = GraphBuilder::new().node("A", "task").node("B", "task");
        builder = builder.edge("A", "B").edge("B", "A");
        let graph = builder.build();
        // Must terminate without hanging.
        let chains = trace_to_pillars(&graph, "A");
        // There is no pillar, so chains should be empty (or at most trivial).
        // The critical property is that this terminates.
        let _ = chains;
    }

    // -------------------------------------------------------------------------
    // trace_descendants
    // -------------------------------------------------------------------------

    #[test]
    fn trace_descendants_linear_chain_returns_all_downstream() {
        // pillar → epic → task: descendants of pillar = [epic, task].
        let graph = GraphBuilder::new()
            .node("PIL-1", "pillar")
            .node("EP-1", "epic")
            .node("T-1", "task")
            .edge("PIL-1", "EP-1")
            .edge("EP-1", "T-1")
            .build();
        let descendants = trace_descendants(&graph, "PIL-1", 10);
        let ids: Vec<&str> = descendants.iter().map(|d| d.id.as_str()).collect();
        assert!(ids.contains(&"EP-1"), "epic must be a descendant of pillar");
        assert!(ids.contains(&"T-1"), "task must be a descendant of pillar");
    }

    #[test]
    fn trace_descendants_max_depth_limits_traversal() {
        // A→B→C→D: with max_depth=1 only B should be returned (not C or D).
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .node("C", "task")
            .node("D", "task")
            .edge("A", "B")
            .edge("B", "C")
            .edge("C", "D")
            .build();
        let descendants = trace_descendants(&graph, "A", 1);
        let ids: Vec<&str> = descendants.iter().map(|d| d.id.as_str()).collect();
        assert!(ids.contains(&"B"), "B is at depth 1 and must be included");
        assert!(!ids.contains(&"C"), "C is at depth 2 and must be excluded with max_depth=1");
        assert!(!ids.contains(&"D"), "D is at depth 3 and must be excluded with max_depth=1");
    }

    #[test]
    fn trace_descendants_starting_node_not_in_result() {
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .edge("A", "B")
            .build();
        let descendants = trace_descendants(&graph, "A", 10);
        let ids: Vec<&str> = descendants.iter().map(|d| d.id.as_str()).collect();
        assert!(!ids.contains(&"A"), "the starting node must not appear in its own descendants");
    }

    #[test]
    fn trace_descendants_depth_field_matches_hop_count() {
        // A→B: B must have depth=1. A→B→C: C must have depth=2.
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .node("C", "task")
            .edge("A", "B")
            .edge("B", "C")
            .build();
        let descendants = trace_descendants(&graph, "A", 10);
        let b = descendants.iter().find(|d| d.id == "B").expect("B not found");
        let c = descendants.iter().find(|d| d.id == "C").expect("C not found");
        assert_eq!(b.depth, 1);
        assert_eq!(c.depth, 2);
    }

    #[test]
    fn trace_descendants_cycle_terminates() {
        // A→B→A: cycle guard must prevent infinite BFS.
        let graph = GraphBuilder::new()
            .node("A", "task")
            .node("B", "task")
            .edge("A", "B")
            .edge("B", "A")
            .build();
        let descendants = trace_descendants(&graph, "A", 10);
        // B is the only descendant (A is already visited).
        assert_eq!(descendants.len(), 1);
        assert_eq!(descendants[0].id, "B");
    }

    #[test]
    fn trace_descendants_impact_radius_counts_within_two_hops() {
        // A→B→C→D: descendants at depth ≤2 are B and C → impact_radius=2.
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .node("C", "task")
            .node("D", "task")
            .edge("A", "B")
            .edge("B", "C")
            .edge("C", "D")
            .build();
        let result = compute_traceability(&graph, "A");
        assert_eq!(result.impact_radius, 2,
            "only B (depth 1) and C (depth 2) are within the 2-hop impact radius");
    }

    // -------------------------------------------------------------------------
    // find_siblings
    // -------------------------------------------------------------------------

    #[test]
    fn siblings_two_tasks_delivering_same_epic_are_siblings() {
        // T-1 and T-2 both deliver EP-1; they are siblings of each other.
        let graph = GraphBuilder::new()
            .node("EP-1", "epic")
            .node("T-1", "task")
            .node("T-2", "task")
            .edge("T-1", "EP-1")
            .edge("T-2", "EP-1")
            .build();
        let siblings = find_siblings(&graph, "T-1");
        assert!(siblings.contains(&"T-2".to_owned()),
            "T-2 must be a sibling of T-1 because both deliver EP-1");
    }

    #[test]
    fn siblings_task_with_no_parent_returns_empty() {
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let siblings = find_siblings(&graph, "T-1");
        assert!(siblings.is_empty(),
            "isolated task has no siblings");
    }

    #[test]
    fn siblings_do_not_include_self() {
        let graph = GraphBuilder::new()
            .node("EP-1", "epic")
            .node("T-1", "task")
            .node("T-2", "task")
            .edge("T-1", "EP-1")
            .edge("T-2", "EP-1")
            .build();
        let siblings = find_siblings(&graph, "T-1");
        assert!(!siblings.contains(&"T-1".to_owned()),
            "a task must not appear as its own sibling");
    }

    #[test]
    fn siblings_only_children_of_same_parent_are_included() {
        // T-1 delivers EP-1. T-3 delivers EP-2. T-2 delivers EP-1.
        // Siblings of T-1 should include T-2 but NOT T-3.
        let graph = GraphBuilder::new()
            .node("EP-1", "epic")
            .node("EP-2", "epic")
            .node("T-1", "task")
            .node("T-2", "task")
            .node("T-3", "task")
            .edge("T-1", "EP-1")
            .edge("T-2", "EP-1")
            .edge("T-3", "EP-2")
            .build();
        let siblings = find_siblings(&graph, "T-1");
        assert!(siblings.contains(&"T-2".to_owned()));
        assert!(!siblings.contains(&"T-3".to_owned()),
            "T-3 delivers a different epic and must not appear as sibling of T-1");
    }

    #[test]
    fn siblings_unknown_artifact_returns_empty() {
        let graph = GraphBuilder::new().node("T-1", "task").build();
        let siblings = find_siblings(&graph, "NONEXISTENT");
        assert!(siblings.is_empty());
    }

    // -------------------------------------------------------------------------
    // Private helpers — count_total_edges via compute_health
    // -------------------------------------------------------------------------

    #[test]
    fn total_edges_correct_for_known_graph() {
        // A→B, A→C, B→C: 3 edges total.
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .node("C", "task")
            .edge("A", "B")
            .edge("A", "C")
            .edge("B", "C")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.total_edges, 3);
    }

    // -------------------------------------------------------------------------
    // Private helpers — compute_avg_degree via compute_health
    // -------------------------------------------------------------------------

    #[test]
    fn avg_degree_correct_for_star_graph() {
        // Hub A → B, C, D.  A: out=3, in=0 → degree 3.  B,C,D: out=0, in=1 → degree 1 each.
        // Total degree = 3 + 1 + 1 + 1 = 6. Nodes = 4. Avg = 1.5.
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "task")
            .node("C", "task")
            .node("D", "task")
            .edge("A", "B")
            .edge("A", "C")
            .edge("A", "D")
            .build();
        let health = compute_health(&graph);
        assert!(
            (health.avg_degree - 1.5).abs() < 1e-9,
            "expected avg_degree=1.5 for star graph, got {}",
            health.avg_degree
        );
    }

    // -------------------------------------------------------------------------
    // Private helpers — count_broken_refs via compute_health
    // -------------------------------------------------------------------------

    #[test]
    fn broken_ref_count_two_out_of_three_edges() {
        // T-1 has 3 outgoing edges: 1 valid (EP-1 exists), 2 broken.
        let graph = GraphBuilder::new()
            .node("T-1", "task")
            .node("EP-1", "epic")
            .edge("T-1", "EP-1")
            .broken_edge("T-1", "GONE-X")
            .broken_edge("T-1", "GONE-Y")
            .build();
        let health = compute_health(&graph);
        assert_eq!(health.broken_ref_count, 2);
    }

    // -------------------------------------------------------------------------
    // Private helpers — compute_components via largest_component_ratio
    // -------------------------------------------------------------------------

    #[test]
    fn three_separate_components_each_size_one() {
        // Three isolated nodes. Each is its own component.
        let graph = GraphBuilder::new()
            .node("A", "epic")
            .node("B", "epic")
            .node("C", "epic")
            .build();
        let health = compute_health(&graph);
        // Largest component = 1 out of 3.
        assert!(
            (health.largest_component_ratio - (1.0 / 3.0)).abs() < 1e-9,
            "three isolated nodes → ratio 1/3, got {}",
            health.largest_component_ratio
        );
    }

    // -------------------------------------------------------------------------
    // Private helpers — reverse_bfs_from_pillars via pillar_traceability
    // -------------------------------------------------------------------------

    #[test]
    fn reverse_bfs_from_pillars_reaches_all_connected_nodes() {
        // task→epic→pillar: all three should be traceable to the pillar.
        let graph = GraphBuilder::new()
            .node("PIL-1", "pillar")
            .node("EP-1", "epic")
            .node("T-1", "task")
            .edge("T-1", "EP-1")
            .edge("EP-1", "PIL-1")
            .build();
        let health = compute_health(&graph);
        // All 3 non-doc nodes should be reachable → 100%.
        assert!(
            (health.pillar_traceability - 100.0).abs() < 1e-9,
            "reverse BFS from pillar should reach all connected nodes, got {}%",
            health.pillar_traceability
        );
    }

    #[test]
    fn reverse_bfs_from_pillars_isolated_node_not_traceable() {
        // Pillar + isolated task: only pillar can trace to itself.
        // Non-doc nodes: pillar + task = 2. Reachable from pillar = 1 (pillar only).
        // Traceability = 1/2 = 50%.
        let graph = GraphBuilder::new()
            .node("PIL-1", "pillar")
            .node("T-1", "task")
            .build();
        let health = compute_health(&graph);
        assert!(
            (health.pillar_traceability - 50.0).abs() < 1e-9,
            "pillar traces to itself, task is unreachable → 50%, got {}",
            health.pillar_traceability
        );
    }
}
