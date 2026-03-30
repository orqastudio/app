//! Graph-theoretic metrics computation.
//!
//! All metrics are computed in Rust from the `ArtifactGraph` data structure.
//! No delegation to JavaScript or external services.

use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::graph::ArtifactGraph;
use crate::types::GraphHealth;

// ---------------------------------------------------------------------------
// Traceability types
// ---------------------------------------------------------------------------

/// A single node in an ancestry chain, ordered from the query artifact up to
/// the pillar or vision root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryNode {
    /// Artifact ID (e.g. "EPIC-048").
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Artifact type string (e.g. "epic", "pillar").
    pub artifact_type: String,
    /// The relationship type that connects this node to the *next* node in the
    /// chain (i.e. the edge leading upward toward the pillar).
    /// Empty string for the terminal node.
    pub relationship: String,
}

/// An ordered path from the query artifact to a pillar or vision root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryChain {
    /// Ordered from current artifact (index 0) to pillar/vision root (last).
    pub path: Vec<AncestryNode>,
}

/// A downstream artifact with its BFS distance from the query artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracedArtifact {
    /// Artifact ID.
    pub id: String,
    /// BFS hops from the query artifact.
    pub depth: usize,
}

/// Full traceability result for a single artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilityResult {
    /// All paths from the artifact upward to any pillar or vision.
    pub ancestry_chains: Vec<AncestryChain>,
    /// All downstream artifacts (following references_out), with distance.
    pub descendants: Vec<TracedArtifact>,
    /// Artifacts that share at least one direct parent with the query artifact.
    pub siblings: Vec<String>,
    /// Count of distinct descendants within 2 hops (impact radius).
    pub impact_radius: usize,
    /// True when no path exists to any pillar or vision artifact.
    pub disconnected: bool,
}

// ---------------------------------------------------------------------------
// GraphHealth helpers
// ---------------------------------------------------------------------------

impl GraphHealth {
    /// Return a zeroed `GraphHealth` for an empty graph.
    fn empty() -> Self {
        Self {
            component_count: 0,
            orphan_count: 0,
            orphan_percentage: 0.0,
            avg_degree: 0.0,
            graph_density: 0.0,
            largest_component_ratio: 0.0,
            total_nodes: 0,
            total_edges: 0,
            pillar_traceability: 0.0,
            bidirectionality_ratio: 0.0,
            broken_ref_count: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Traceability queries
// ---------------------------------------------------------------------------

/// Return ALL directed paths from `artifact_id` upward to any pillar or vision
/// artifact, following `references_out` edges.
///
/// Each path is returned as an [`AncestryChain`] with nodes ordered from the
/// query artifact (index 0) to the pillar/vision root (last index).
///
/// Uses iterative DFS with a cycle guard to avoid infinite loops.
pub fn trace_to_pillars(graph: &ArtifactGraph, artifact_id: &str) -> Vec<AncestryChain> {
    let target_types: HashSet<&str> = ["pillar", "vision"].iter().copied().collect();
    // Hard limits to prevent path explosion in dense graphs.
    const MAX_DEPTH: usize = 15;
    const MAX_RESULTS: usize = 20;
    const MAX_STACK_SIZE: usize = 5_000;

    // Each stack frame is (current_id, path_so_far, visited_in_this_path)
    let mut stack: Vec<(String, Vec<AncestryNode>, HashSet<String>)> = Vec::new();
    let mut results: Vec<AncestryChain> = Vec::new();

    let Some(start) = graph.nodes.get(artifact_id) else {
        return results;
    };

    let start_node = AncestryNode {
        id: start.id.clone(),
        title: start.title.clone(),
        artifact_type: start.artifact_type.clone(),
        relationship: String::new(),
    };
    let mut initial_visited = HashSet::new();
    initial_visited.insert(artifact_id.to_owned());
    stack.push((artifact_id.to_owned(), vec![start_node], initial_visited));

    while let Some((current_id, path, visited)) = stack.pop() {
        // Safety limits: stop exploring if we have enough results or the stack is too deep.
        if results.len() >= MAX_RESULTS || stack.len() > MAX_STACK_SIZE {
            break;
        }
        if path.len() > MAX_DEPTH {
            continue;
        }

        let Some(current_node) = graph.nodes.get(&current_id) else {
            continue;
        };

        // If we reached a pillar/vision with a non-trivial path, record the chain.
        if target_types.contains(current_node.artifact_type.as_str()) && path.len() > 1 {
            results.push(AncestryChain { path: path.clone() });
            continue;
        }

        let has_upward_edge = expand_dfs_node(graph, current_node, &path, &visited, &mut stack);

        // If this node has no upward edges and it IS a pillar/vision (len == 1
        // means we started on a pillar), record as a trivially connected chain.
        if !has_upward_edge && target_types.contains(current_node.artifact_type.as_str()) {
            results.push(AncestryChain { path: path.clone() });
        }
    }

    results
}

/// Push unvisited outgoing neighbours of `current_node` onto the DFS stack.
///
/// Each new frame extends `path` with an annotated edge and the new target node.
/// Returns `true` if at least one unvisited neighbour was found.
fn expand_dfs_node(
    graph: &ArtifactGraph,
    current_node: &crate::graph::ArtifactNode,
    path: &[AncestryNode],
    visited: &HashSet<String>,
    stack: &mut Vec<(String, Vec<AncestryNode>, HashSet<String>)>,
) -> bool {
    let mut has_upward_edge = false;
    for ref_entry in &current_node.references_out {
        let target = &ref_entry.target_id;
        if visited.contains(target.as_str()) {
            continue;
        }
        let Some(target_node) = graph.nodes.get(target) else {
            continue;
        };
        has_upward_edge = true;

        let rel_type = ref_entry
            .relationship_type
            .clone()
            .unwrap_or_else(|| ref_entry.field.clone());

        let mut new_path = path.to_vec();
        if let Some(last) = new_path.last_mut() {
            last.relationship = rel_type;
        }
        new_path.push(AncestryNode {
            id: target_node.id.clone(),
            title: target_node.title.clone(),
            artifact_type: target_node.artifact_type.clone(),
            relationship: String::new(),
        });

        let mut new_visited = visited.clone();
        new_visited.insert(target.clone());
        stack.push((target.clone(), new_path, new_visited));
    }
    has_upward_edge
}

/// Return all artifacts reachable downstream from `artifact_id` (following
/// `references_out`) up to `max_depth` hops, with their BFS distance.
///
/// The starting artifact itself is NOT included in the result.
pub fn trace_descendants(
    graph: &ArtifactGraph,
    artifact_id: &str,
    max_depth: usize,
) -> Vec<TracedArtifact> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<(&str, usize)> = VecDeque::new();
    let mut result: Vec<TracedArtifact> = Vec::new();

    visited.insert(artifact_id);
    queue.push_back((artifact_id, 0));

    while let Some((current_id, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }
        let Some(node) = graph.nodes.get(current_id) else {
            continue;
        };
        for ref_entry in &node.references_out {
            let target = ref_entry.target_id.as_str();
            if visited.contains(target) {
                continue;
            }
            visited.insert(target);
            result.push(TracedArtifact {
                id: target.to_owned(),
                depth: depth + 1,
            });
            queue.push_back((target, depth + 1));
        }
    }

    result
}

/// Return all artifacts that share at least one direct parent with
/// `artifact_id` (i.e. artifacts whose `references_out` overlap with the
/// parents of `artifact_id`).
///
/// "Parent" is defined as any artifact that `artifact_id` references directly
/// via `references_out`.
pub fn find_siblings(graph: &ArtifactGraph, artifact_id: &str) -> Vec<String> {
    let Some(node) = graph.nodes.get(artifact_id) else {
        return Vec::new();
    };

    // Collect the set of direct parents.
    let parents: HashSet<&str> = node
        .references_out
        .iter()
        .map(|r| r.target_id.as_str())
        .collect();

    if parents.is_empty() {
        return Vec::new();
    }

    // Any artifact (other than self) that references at least one of our parents
    // is a sibling.
    let mut siblings: HashSet<String> = HashSet::new();
    for parent_id in &parents {
        let Some(parent_node) = graph.nodes.get(*parent_id) else {
            continue;
        };
        // references_in on the parent give us all of the parent's children.
        for ref_entry in &parent_node.references_in {
            let sibling_id = &ref_entry.source_id;
            if sibling_id != artifact_id {
                siblings.insert(sibling_id.clone());
            }
        }
    }

    let mut result: Vec<String> = siblings.into_iter().collect();
    result.sort();
    result
}

/// Compute the full traceability result for a single artifact.
pub fn compute_traceability(graph: &ArtifactGraph, artifact_id: &str) -> TraceabilityResult {
    let ancestry_chains = trace_to_pillars(graph, artifact_id);
    let descendants = trace_descendants(graph, artifact_id, 10);
    let siblings = find_siblings(graph, artifact_id);

    // Impact radius: distinct descendants within 2 hops.
    let impact_radius = descendants.iter().filter(|d| d.depth <= 2).count();

    let disconnected = ancestry_chains.is_empty()
        || ancestry_chains.iter().all(|chain| {
            chain
                .path
                .last()
                .is_none_or(|n| n.artifact_type != "pillar" && n.artifact_type != "vision")
        });

    TraceabilityResult {
        ancestry_chains,
        descendants,
        siblings,
        impact_radius,
        disconnected,
    }
}

/// Compute graph health metrics for the artifact graph.
pub fn compute_health(graph: &ArtifactGraph) -> GraphHealth {
    // Work only with primary nodes (exclude bare-ID aliases in org mode).
    let primary_ids: Vec<&str> = graph
        .nodes
        .iter()
        .filter(|(key, node)| !(key.as_str() == node.id && node.project.is_some()))
        .map(|(key, _)| key.as_str())
        .collect();

    let total_nodes = primary_ids.len();

    if total_nodes == 0 {
        return GraphHealth::empty();
    }

    let primary_set: HashSet<&str> = primary_ids.iter().copied().collect();
    let total_edges = count_total_edges(graph, &primary_ids);
    let (orphan_count, orphan_percentage) = count_orphans(graph, &primary_ids, total_nodes);
    let avg_degree = compute_avg_degree(graph, &primary_ids, total_nodes);
    let graph_density = compute_graph_density(total_edges, total_nodes);
    let (component_count, largest_component_size) =
        compute_components(graph, &primary_ids, &primary_set);
    let largest_component_ratio = largest_component_size as f64 / total_nodes as f64;
    let pillar_traceability = compute_pillar_traceability(graph, &primary_ids);
    let bidirectionality_ratio = compute_bidirectionality_ratio(graph, &primary_ids);
    let broken_ref_count = count_broken_refs(graph, &primary_ids);

    GraphHealth {
        component_count,
        orphan_count,
        orphan_percentage,
        avg_degree,
        graph_density,
        largest_component_ratio,
        total_nodes,
        total_edges,
        pillar_traceability,
        bidirectionality_ratio,
        broken_ref_count,
    }
}

/// Return the total number of directed outgoing edges among primary nodes.
fn count_total_edges(graph: &ArtifactGraph, primary_ids: &[&str]) -> usize {
    primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .map(|n| n.references_out.len())
        .sum()
}

/// Return `(orphan_count, orphan_percentage)` for non-doc nodes with no edges.
fn count_orphans(graph: &ArtifactGraph, primary_ids: &[&str], total_nodes: usize) -> (usize, f64) {
    let orphan_count = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .filter(|n| {
            n.artifact_type != "doc" && n.references_out.is_empty() && n.references_in.is_empty()
        })
        .count();
    let orphan_percentage = (orphan_count as f64 / total_nodes as f64) * 100.0;
    (orphan_count, orphan_percentage)
}

/// Return the average (in + out) degree across all primary nodes.
fn compute_avg_degree(graph: &ArtifactGraph, primary_ids: &[&str], total_nodes: usize) -> f64 {
    let total_degree: usize = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .map(|n| n.references_out.len() + n.references_in.len())
        .sum();
    total_degree as f64 / total_nodes as f64
}

/// Return the directed graph density: `edges / (n * (n - 1))`.
fn compute_graph_density(total_edges: usize, total_nodes: usize) -> f64 {
    let max_edges = total_nodes.saturating_mul(total_nodes.saturating_sub(1));
    if max_edges > 0 {
        total_edges as f64 / max_edges as f64
    } else {
        0.0
    }
}

/// Return the count of edges whose target is not present in the graph.
fn count_broken_refs(graph: &ArtifactGraph, primary_ids: &[&str]) -> usize {
    primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .flat_map(|n| n.references_out.iter())
        .filter(|r| !graph.nodes.contains_key(&r.target_id))
        .count()
}

/// Compute weakly connected components using BFS on the undirected projection.
///
/// Returns `(component_count, largest_component_size)`.
fn compute_components(
    graph: &ArtifactGraph,
    primary_ids: &[&str],
    primary_set: &HashSet<&str>,
) -> (usize, usize) {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut component_count = 0;
    let mut largest = 0;

    for &start_id in primary_ids {
        if visited.contains(start_id) {
            continue;
        }

        // BFS from start_id over the undirected graph.
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(start_id);
        visited.insert(start_id);
        let mut size = 0;

        while let Some(current_id) = queue.pop_front() {
            size += 1;

            let Some(node) = graph.nodes.get(current_id) else {
                continue;
            };

            // Follow outgoing edges.
            for ref_entry in &node.references_out {
                let target = ref_entry.target_id.as_str();
                if primary_set.contains(target) && !visited.contains(target) {
                    visited.insert(target);
                    queue.push_back(target);
                }
            }

            // Follow incoming edges.
            for ref_entry in &node.references_in {
                let source = ref_entry.source_id.as_str();
                if primary_set.contains(source) && !visited.contains(source) {
                    visited.insert(source);
                    queue.push_back(source);
                }
            }
        }

        component_count += 1;
        if size > largest {
            largest = size;
        }
    }

    (component_count, largest)
}

/// Compute what percentage of non-doc artifacts can trace a path to a pillar artifact.
///
/// Uses reverse BFS from all pillar nodes to find every node that can reach a pillar.
fn compute_pillar_traceability(graph: &ArtifactGraph, primary_ids: &[&str]) -> f64 {
    let pillar_ids: Vec<&str> = primary_ids
        .iter()
        .filter_map(|id| {
            let node = graph.nodes.get(*id)?;
            (node.artifact_type == "pillar").then_some(*id)
        })
        .collect();

    if pillar_ids.is_empty() {
        return 0.0;
    }

    let reachable = reverse_bfs_from_pillars(graph, &pillar_ids);

    let non_doc_ids: Vec<&str> = primary_ids
        .iter()
        .filter_map(|id| {
            let node = graph.nodes.get(*id)?;
            (node.artifact_type != "doc").then_some(*id)
        })
        .collect();

    let non_doc_count = non_doc_ids.len();
    if non_doc_count == 0 {
        return 0.0;
    }

    let traceable = non_doc_ids
        .iter()
        .filter(|id| reachable.contains(*id))
        .count();
    (traceable as f64 / non_doc_count as f64) * 100.0
}

/// BFS backwards from `pillar_ids` via `references_in` edges.
///
/// Returns the set of artifact IDs that have at least one directed path to a pillar.
fn reverse_bfs_from_pillars<'a>(
    graph: &'a ArtifactGraph,
    pillar_ids: &[&'a str],
) -> HashSet<&'a str> {
    let mut reachable: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();

    for &pid in pillar_ids {
        reachable.insert(pid);
        queue.push_back(pid);
    }

    while let Some(current_id) = queue.pop_front() {
        let Some(node) = graph.nodes.get(current_id) else {
            continue;
        };
        for ref_entry in &node.references_in {
            let source = ref_entry.source_id.as_str();
            if !reachable.contains(source) {
                reachable.insert(source);
                queue.push_back(source);
            }
        }
    }

    reachable
}

/// Compute the fraction of typed relationship edges whose type has a schema-defined inverse.
///
/// Under forward-only storage, inverse edges are computed at query time by the
/// graph engine (Pass 2) rather than stored in artifact files. This metric
/// therefore counts whether the relationship *schema* defines an inverse for
/// each edge's type — not whether a stored inverse edge exists.
fn compute_bidirectionality_ratio(graph: &ArtifactGraph, primary_ids: &[&str]) -> f64 {
    // Build the set of relationship types that have a schema-defined inverse.
    let inverse_map: HashMap<String, String> = crate::platform::PLATFORM
        .relationships
        .iter()
        .flat_map(|rel| {
            let mut pairs = vec![(rel.key.clone(), rel.inverse.clone())];
            if rel.inverse != rel.key {
                pairs.push((rel.inverse.clone(), rel.key.clone()));
            }
            pairs
        })
        .collect();

    // Count typed relationship edges and those with a schema-defined inverse.
    let mut total: usize = 0;
    let mut with_inverse: usize = 0;

    for id in primary_ids {
        let Some(node) = graph.nodes.get(*id) else {
            continue;
        };
        for ref_entry in &node.references_out {
            let Some(rel_type) = ref_entry.relationship_type.as_deref() else {
                continue;
            };
            total += 1;
            if inverse_map.contains_key(rel_type) {
                with_inverse += 1;
            }
        }
    }

    if total == 0 {
        return 1.0; // vacuously true
    }

    with_inverse as f64 / total as f64
}
