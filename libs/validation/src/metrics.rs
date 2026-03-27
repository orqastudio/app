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
// Traceability queries
// ---------------------------------------------------------------------------

/// Return ALL directed paths from `artifact_id` upward to any pillar or vision
/// artifact, following `references_out` edges.
///
/// Each path is returned as an [`AncestryChain`] with nodes ordered from the
/// query artifact (index 0) to the pillar/vision root (last index).
///
/// Uses iterative DFS with a cycle guard to avoid infinite loops.
#[allow(clippy::too_many_lines)]
pub fn trace_to_pillars(graph: &ArtifactGraph, artifact_id: &str) -> Vec<AncestryChain> {
    let target_types: HashSet<&str> = ["pillar", "vision"].iter().copied().collect();

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
        let Some(current_node) = graph.nodes.get(&current_id) else {
            continue;
        };

        // If we reached a pillar/vision, record the chain.
        if target_types.contains(current_node.artifact_type.as_str()) && path.len() > 1 {
            results.push(AncestryChain { path: path.clone() });
            continue;
        }

        // Expand: follow outgoing edges (forward references leading upward).
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

            // Annotate the *last* node in the current path with the edge type.
            let mut new_path = path.clone();
            if let Some(last) = new_path.last_mut() {
                last.relationship = rel_type;
            }

            let next_node = AncestryNode {
                id: target_node.id.clone(),
                title: target_node.title.clone(),
                artifact_type: target_node.artifact_type.clone(),
                relationship: String::new(),
            };
            new_path.push(next_node);

            let mut new_visited = visited.clone();
            new_visited.insert(target.clone());
            stack.push((target.clone(), new_path, new_visited));
        }

        // If this node has no upward edges and it IS a pillar/vision (len == 1
        // means we started on a pillar), record as a trivially connected chain.
        if !has_upward_edge && target_types.contains(current_node.artifact_type.as_str()) {
            results.push(AncestryChain { path: path.clone() });
        }
    }

    results
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

#[allow(clippy::too_many_lines)]
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
        return GraphHealth {
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
        };
    }

    // Build undirected adjacency for connected-component analysis.
    let primary_set: HashSet<&str> = primary_ids.iter().copied().collect();

    // Total directed edges among primary nodes.
    let total_edges: usize = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .map(|n| n.references_out.len())
        .sum();

    // Orphan count: non-doc nodes with no edges in either direction.
    let orphan_count = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .filter(|n| {
            n.artifact_type != "doc" && n.references_out.is_empty() && n.references_in.is_empty()
        })
        .count();

    let orphan_percentage = if total_nodes > 0 {
        (orphan_count as f64 / total_nodes as f64) * 100.0
    } else {
        0.0
    };

    // Average degree: sum of (out + in) edges across all primary nodes, divided by node count.
    // Each undirected edge is counted once for each endpoint.
    let total_degree: usize = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .map(|n| n.references_out.len() + n.references_in.len())
        .sum();
    let avg_degree = if total_nodes > 0 {
        total_degree as f64 / total_nodes as f64
    } else {
        0.0
    };

    // Graph density: directed density = edges / (n * (n-1))
    let max_edges = total_nodes.saturating_mul(total_nodes.saturating_sub(1));
    let graph_density = if max_edges > 0 {
        total_edges as f64 / max_edges as f64
    } else {
        0.0
    };

    // Connected components via BFS on the undirected graph.
    let (component_count, largest_component_size) =
        compute_components(graph, &primary_ids, &primary_set);

    let largest_component_ratio = if total_nodes > 0 {
        largest_component_size as f64 / total_nodes as f64
    } else {
        0.0
    };

    // Pillar traceability: fraction of non-doc nodes reachable from a pillar artifact.
    let pillar_traceability = compute_pillar_traceability(graph, &primary_ids);

    // Bidirectionality ratio: fraction of typed relationship edges that have their inverse.
    let bidirectionality_ratio = compute_bidirectionality_ratio(graph, &primary_ids);

    // Broken references: edges whose target is not in the graph.
    let broken_ref_count: usize = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .flat_map(|n| n.references_out.iter())
        .filter(|r| !graph.nodes.contains_key(&r.target_id))
        .count();

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
#[allow(clippy::too_many_lines)]
///
/// Uses reverse BFS from all pillar nodes to find every node that can reach a pillar.
fn compute_pillar_traceability(graph: &ArtifactGraph, primary_ids: &[&str]) -> f64 {
    // Collect all pillar IDs.
    let pillar_ids: Vec<&str> = primary_ids
        .iter()
        .filter_map(|id| {
            let node = graph.nodes.get(*id)?;
            if node.artifact_type == "pillar" {
                Some(*id)
            } else {
                None
            }
        })
        .collect();

    if pillar_ids.is_empty() {
        return 0.0;
    }

    // BFS outward from pillars following INCOMING edges (backwards traversal).
    // A node is "pillar-traceable" if there is a directed path FROM it TO a pillar.
    // We achieve this by reversing the direction: starting from pillars and following references_in.
    let mut reachable: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();

    for pid in &pillar_ids {
        reachable.insert(*pid);
        queue.push_back(*pid);
    }

    while let Some(current_id) = queue.pop_front() {
        let Some(node) = graph.nodes.get(current_id) else {
            continue;
        };
        // Follow references_in backwards: nodes that reference current can reach current.
        for ref_entry in &node.references_in {
            let source = ref_entry.source_id.as_str();
            if !reachable.contains(source) {
                reachable.insert(source);
                queue.push_back(source);
            }
        }
    }

    // Count non-doc primary nodes and those that are pillar-traceable.
    let non_doc_ids: Vec<&str> = primary_ids
        .iter()
        .filter_map(|id| {
            let node = graph.nodes.get(*id)?;
            if node.artifact_type == "doc" {
                None
            } else {
                Some(*id)
            }
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
