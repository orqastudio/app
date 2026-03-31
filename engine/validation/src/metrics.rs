//! Graph-theoretic metrics computation.
//!
//! All metrics are computed in Rust from the `ArtifactGraph` data structure.
//! No delegation to JavaScript or external services.
//!
//! The health model tracks two named pipelines:
//! - Delivery pipeline: task, epic, milestone, idea, research, decision, wireframe
//! - Learning pipeline: lesson, rule
//!
//! Artifacts outside both pipelines (excluding archived/surpassed status and
//! knowledge/doc types) are counted as outliers once they have exceeded the
//! grace period for their artifact type. Grace periods give new artifacts time
//! to be connected before they are flagged as attention items.

use std::collections::{HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::graph::ArtifactGraph;
use crate::types::{GraphHealth, OutlierAgeDistribution};

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

/// Artifact types belonging to the delivery pipeline.
const DELIVERY_TYPES: &[&str] = &[
    "task", "epic", "milestone", "idea", "research", "decision", "wireframe",
];

/// Artifact types belonging to the learning pipeline.
const LEARNING_TYPES: &[&str] = &["lesson", "rule"];

/// Status values that exclude an artifact from outlier analysis.
const EXCLUDED_STATUSES: &[&str] = &["archived", "surpassed"];

/// Artifact types that are excluded from outlier analysis entirely (connected
/// via prompt injection or documentation, not graph relationships).
const EXCLUDED_TYPES: &[&str] = &["knowledge", "doc"];

/// Grace period in days before an artifact outside both pipelines is counted
/// as an outlier. New artifacts need time to find their place in the system.
/// All types use 30 days — real projects need breathing room.
const DEFAULT_GRACE_DAYS: i64 = 30;

/// Age thresholds for the outlier distribution buckets.
/// - Fresh: within grace period (≤30d) — expected, no action needed
/// - Aging: between grace and stale (30–90d) — should be progressed or archived soon
/// - Stale: older than 3 months (>90d) or no created date — priority action
const AGING_THRESHOLD_DAYS: i64 = 30;
const STALE_THRESHOLD_DAYS: i64 = 90;

/// Return the grace period in days for a given artifact type.
/// Currently uniform across all types (30 days).
fn grace_days(_artifact_type: &str) -> i64 {
    DEFAULT_GRACE_DAYS
}

/// Parse a `created` frontmatter value ("YYYY-MM-DD") and return age in whole days
/// relative to `today_days` (days since Unix epoch). Returns `None` when the field
/// is absent or cannot be parsed.
fn parse_created_age(frontmatter: &serde_json::Value, today_days: i64) -> Option<i64> {
    let date_str = frontmatter.get("created")?.as_str()?;
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year: i64 = parts[0].parse().ok()?;
    let month: i64 = parts[1].parse().ok()?;
    let day: i64 = parts[2].parse().ok()?;

    // Days-since-epoch via the civil date formula (no external crates required).
    // Gregorian proleptic calendar. Accurate for all modern dates.
    let m = (month - 3).rem_euclid(12);
    let y = year - (month < 3) as i64;
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let doy = (153 * m + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let artifact_days = era * 146097 + doe - 719468;

    Some((today_days - artifact_days).max(0))
}

/// Return today as days since the Unix epoch, using the system clock.
///
/// Falls back to 0 if the system time is unavailable (test environments, etc.).
fn today_days_since_epoch() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64 / 86400)
        .unwrap_or(0)
}

impl GraphHealth {
    /// Return a zeroed `GraphHealth` for an empty graph.
    fn empty() -> Self {
        Self {
            outlier_count: 0,
            outlier_percentage: 0.0,
            outlier_age_distribution: OutlierAgeDistribution::default(),
            delivery_connectivity: 0.0,
            learning_connectivity: 0.0,
            avg_degree: 0.0,
            largest_component_ratio: 0.0,
            total_nodes: 0,
            total_edges: 0,
            pillar_traceability: 0.0,
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
///
/// Uses the two-pipeline model: delivery (task/epic/milestone/idea/research/decision/wireframe)
/// and learning (lesson/rule). Artifacts outside both pipelines that are not
/// archived, surpassed, knowledge, or doc, and that have exceeded their type's
/// grace period, are counted as outliers.
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

    let today = today_days_since_epoch();
    let primary_set: HashSet<&str> = primary_ids.iter().copied().collect();
    let total_edges = count_total_edges(graph, &primary_ids);
    let avg_degree = compute_avg_degree(graph, &primary_ids, total_nodes);
    let (_, largest_component_size) =
        compute_components(graph, &primary_ids, &primary_set);
    let largest_component_ratio = largest_component_size as f64 / total_nodes as f64;
    let pillar_traceability = compute_pillar_traceability(graph, &primary_ids);
    let broken_ref_count = count_broken_refs(graph, &primary_ids);
    let (outlier_count, outlier_percentage, outlier_age_distribution) =
        compute_outliers(graph, &primary_ids, today);
    let delivery_connectivity = compute_delivery_connectivity(graph, &primary_ids);
    let learning_connectivity = compute_learning_connectivity(graph, &primary_ids);

    GraphHealth {
        outlier_count,
        outlier_percentage,
        outlier_age_distribution,
        delivery_connectivity,
        learning_connectivity,
        avg_degree,
        largest_component_ratio,
        total_nodes,
        total_edges,
        pillar_traceability,
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

/// Return the average (in + out) degree across all primary nodes.
fn compute_avg_degree(graph: &ArtifactGraph, primary_ids: &[&str], total_nodes: usize) -> f64 {
    let total_degree: usize = primary_ids
        .iter()
        .filter_map(|id| graph.nodes.get(*id))
        .map(|n| n.references_out.len() + n.references_in.len())
        .sum();
    total_degree as f64 / total_nodes as f64
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

/// Compute outlier count, outlier percentage, and age distribution.
///
/// An artifact is an outlier if ALL of the following are true:
/// - Its status is NOT in `EXCLUDED_STATUSES` (not archived or surpassed)
/// - Its type is NOT in `EXCLUDED_TYPES` (not knowledge or doc)
/// - Its type is NOT in either `DELIVERY_TYPES` or `LEARNING_TYPES`
/// - Its age exceeds the grace period for its type (new artifacts get time to be connected)
///
/// The age distribution covers all eligible outliers (pre- and post-grace-period), bucketed
/// into fresh (≤30d), aging (30–90d), and stale (90d+ or no `created` date).
/// The `outlier_count` only counts those that have exceeded the 30-day grace period.
/// The percentage is computed over active (non-excluded) nodes only.
fn compute_outliers(
    graph: &ArtifactGraph,
    primary_ids: &[&str],
    today: i64,
) -> (usize, f64, OutlierAgeDistribution) {
    let delivery_set: HashSet<&str> = DELIVERY_TYPES.iter().copied().collect();
    let learning_set: HashSet<&str> = LEARNING_TYPES.iter().copied().collect();
    let excluded_type_set: HashSet<&str> = EXCLUDED_TYPES.iter().copied().collect();
    let excluded_status_set: HashSet<&str> = EXCLUDED_STATUSES.iter().copied().collect();

    let mut active_count: usize = 0;
    let mut outlier_count: usize = 0;
    let mut dist = OutlierAgeDistribution::default();

    for id in primary_ids {
        let Some(node) = graph.nodes.get(*id) else {
            continue;
        };

        // Skip excluded types (knowledge, doc).
        if excluded_type_set.contains(node.artifact_type.as_str()) {
            continue;
        }

        // Skip archived or surpassed artifacts.
        let status = node.status.as_deref().unwrap_or("");
        if excluded_status_set.contains(status) {
            continue;
        }

        active_count += 1;

        // Artifact is a candidate outlier if it belongs to neither pipeline.
        if delivery_set.contains(node.artifact_type.as_str())
            || learning_set.contains(node.artifact_type.as_str())
        {
            continue;
        }

        // Age bucket and grace period check.
        let age_days = parse_created_age(&node.frontmatter, today);
        let grace = grace_days(&node.artifact_type);

        // Classify into age bucket (stale = unknown age or 90d+).
        match age_days {
            Some(age) if age <= AGING_THRESHOLD_DAYS => dist.fresh += 1,
            Some(age) if age <= STALE_THRESHOLD_DAYS => dist.aging += 1,
            _ => dist.stale += 1,
        }

        // Only count as an outlier once the grace period has elapsed.
        // Artifacts without a `created` date are treated as stale (age unknown ≥ grace).
        let past_grace = age_days.is_none_or(|age| age >= grace);
        if past_grace {
            outlier_count += 1;
        }
    }

    let outlier_percentage = if active_count > 0 {
        (outlier_count as f64 / active_count as f64) * 100.0
    } else {
        0.0
    };

    (outlier_count, outlier_percentage, dist)
}

/// Return the size of the largest weakly-connected component within a subset of nodes.
///
/// BFS traverses only edges where both endpoints are in `subset`. Used by the
/// pipeline connectivity metrics to measure cohesion within a type-filtered group.
fn largest_component_in_subset<'a>(
    graph: &'a ArtifactGraph,
    subset: &[&'a str],
) -> usize {
    let subset_set: HashSet<&str> = subset.iter().copied().collect();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut largest: usize = 0;

    for &start in subset {
        if visited.contains(start) {
            continue;
        }
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        let mut size: usize = 0;
        while let Some(cur) = queue.pop_front() {
            size += 1;
            let Some(node) = graph.nodes.get(cur) else { continue };
            for r in &node.references_out {
                let t = r.target_id.as_str();
                if subset_set.contains(t) && !visited.contains(t) {
                    visited.insert(t);
                    queue.push_back(t);
                }
            }
            for r in &node.references_in {
                let s = r.source_id.as_str();
                if subset_set.contains(s) && !visited.contains(s) {
                    visited.insert(s);
                    queue.push_back(s);
                }
            }
        }
        if size > largest { largest = size; }
    }
    largest
}

/// Compute what fraction of delivery-pipeline artifacts are in the largest
/// weakly-connected component formed by delivery artifacts only.
///
/// Returns 0.0 when there are no delivery artifacts.
fn compute_delivery_connectivity(graph: &ArtifactGraph, primary_ids: &[&str]) -> f64 {
    let delivery_set: HashSet<&str> = DELIVERY_TYPES.iter().copied().collect();
    let delivery_ids: Vec<&str> = primary_ids
        .iter()
        .copied()
        .filter(|id| graph.nodes.get(*id).is_some_and(|n| delivery_set.contains(n.artifact_type.as_str())))
        .collect();
    let total = delivery_ids.len();
    if total == 0 { return 0.0; }
    largest_component_in_subset(graph, &delivery_ids) as f64 / total as f64
}

/// Compute what fraction of learning-pipeline artifacts (lesson, rule) are
/// connected to each other or to decision artifacts.
///
/// A learning artifact is "connected" when it has at least one edge (in or out)
/// to another learning artifact OR to a decision artifact.
///
/// Returns 0.0 when there are no learning artifacts.
fn compute_learning_connectivity(graph: &ArtifactGraph, primary_ids: &[&str]) -> f64 {
    let learning_set: HashSet<&str> = LEARNING_TYPES.iter().copied().collect();

    let learning_ids: Vec<&str> = primary_ids
        .iter()
        .copied()
        .filter(|id| {
            graph
                .nodes
                .get(*id)
                .is_some_and(|n| learning_set.contains(n.artifact_type.as_str()))
        })
        .collect();

    let total_learning = learning_ids.len();
    if total_learning == 0 {
        return 0.0;
    }

    let connected_count = learning_ids
        .iter()
        .filter(|&&id| {
            let Some(node) = graph.nodes.get(id) else {
                return false;
            };
            // Connected if any outgoing edge reaches a learning or decision artifact.
            let out_connected = node.references_out.iter().any(|r| {
                graph
                    .nodes
                    .get(&r.target_id)
                    .is_some_and(|t| learning_set.contains(t.artifact_type.as_str()) || t.artifact_type == "decision")
            });
            // Connected if any incoming edge comes from a learning or decision artifact.
            let in_connected = node.references_in.iter().any(|r| {
                graph
                    .nodes
                    .get(&r.source_id)
                    .is_some_and(|s| learning_set.contains(s.artifact_type.as_str()) || s.artifact_type == "decision")
            });
            out_connected || in_connected
        })
        .count();

    connected_count as f64 / total_learning as f64
}
