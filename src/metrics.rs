//! Graph-theoretic metrics computation.
//!
//! All metrics are computed in Rust from the `ArtifactGraph` data structure.
//! No delegation to JavaScript or external services.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::ArtifactGraph;
use crate::types::GraphHealth;

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
            if node.artifact_type == "doc" { None } else { Some(*id) }
        })
        .collect();

    let non_doc_count = non_doc_ids.len();
    if non_doc_count == 0 {
        return 0.0;
    }

    let traceable = non_doc_ids.iter().filter(|id| reachable.contains(*id)).count();
    (traceable as f64 / non_doc_count as f64) * 100.0
}

/// Compute the fraction of typed relationship edges that have their inverse present.
///
/// Only considers edges with a `relationship_type` set (relationship-array edges).
fn compute_bidirectionality_ratio(graph: &ArtifactGraph, primary_ids: &[&str]) -> f64 {
    // Build a set of (source_id, target_id, rel_type) for quick lookup.
    let edge_set: HashSet<(String, String, String)> = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .flat_map(|node| {
            node.references_out
                .iter()
                .filter_map(|r| {
                    r.relationship_type.as_ref().map(|rt| {
                        (
                            node.id.clone(),
                            r.target_id.clone(),
                            rt.clone(),
                        )
                    })
                })
        })
        .collect();

    // Build inverse map from platform config.
    let mut inverse_map: HashMap<String, String> = HashMap::new();
    for rel in &crate::platform::PLATFORM.relationships {
        inverse_map.insert(rel.key.clone(), rel.inverse.clone());
        if rel.inverse != rel.key {
            inverse_map.insert(rel.inverse.clone(), rel.key.clone());
        }
    }

    let total = edge_set.len();
    if total == 0 {
        return 1.0; // vacuously true
    }

    let bidirectional = edge_set
        .iter()
        .filter(|(source, target, rel_type)| {
            if let Some(inverse) = inverse_map.get(rel_type.as_str()) {
                edge_set.contains(&(target.clone(), source.clone(), inverse.clone()))
            } else {
                false
            }
        })
        .count();

    bidirectional as f64 / total as f64
}
