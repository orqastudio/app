/* eslint-disable */
// THIS FILE IS AUTO-GENERATED — DO NOT EDIT BY HAND.
// Source: libs/types/src/platform/*.schema.json
// Regenerate: node scripts/generate-types.mjs

/**
 * Graph health metrics and traceability types computed by the Rust backend from the artifact graph data structure.
 */
export type MetricsTypes =
  | GraphHealth
  | HealthSnapshot
  | AncestryNode
  | AncestryChain
  | TracedArtifact
  | TraceabilityResult;

/**
 * Age distribution of pipeline outliers, bucketed by how long ago they were created.
 *
 * Only artifacts that pass all outlier filters (active, in-scope type, past grace period)
 * are counted here. Artifacts without a `created` field are placed in the `stale` bucket
 * because their age is unknown and they should be investigated.
 */
export interface OutlierAgeDistribution {
  /** Outliers created within the last 7 days — within grace period, informational only. */
  fresh: number;
  /** Outliers created 7–30 days ago — aging, should be connected or archived soon. */
  aging: number;
  /** Outliers created more than 30 days ago (or with no `created` date) — priority action items. */
  stale: number;
}

/**
 * Graph-theoretic health metrics for the artifact graph. All values are computed purely in Rust from the graph data structure.
 *
 * Models two named pipelines:
 * - Delivery: task, epic, milestone, idea, research, decision, wireframe
 * - Learning: lesson, rule
 *
 * Artifacts outside both pipelines that are not archived/surpassed/knowledge/doc are counted as outliers.
 */
export interface GraphHealth {
  /**
   * Total number of primary nodes (excluding alias nodes in org mode).
   */
  total_nodes: number;
  /**
   * Total number of directed edges.
   */
  total_edges: number;
  /**
   * Number of non-archived, non-knowledge artifacts disconnected from both pipelines and past their grace period.
   */
  outlier_count: number;
  /**
   * outlier_count / active_nodes * 100 (0.0–100.0).
   */
  outlier_percentage: number;
  /**
   * Age distribution of all candidate outliers (fresh ≤7d, aging 7–30d, stale 30d+ or no date).
   */
  outlier_age_distribution: OutlierAgeDistribution;
  /**
   * Fraction of delivery artifacts (task/epic/milestone/idea/research/decision/wireframe) in the main delivery component (0.0–1.0).
   */
  delivery_connectivity: number;
  /**
   * Fraction of learning artifacts (lesson/rule) connected to each other or to decisions (0.0–1.0).
   */
  learning_connectivity: number;
  /**
   * Largest connected component size / total nodes (0.0–1.0).
   */
  largest_component_ratio: number;
  /**
   * Average number of relationships per node (edges * 2 / nodes).
   */
  avg_degree: number;
  /**
   * Percentage of non-doc nodes that can trace a path to a pillar artifact (0.0–100.0).
   */
  pillar_traceability: number;
  /**
   * Number of broken references (target not in graph).
   */
  broken_ref_count: number;
}
/**
 * A point-in-time snapshot of graph health metrics stored in SQLite.
 */
export interface HealthSnapshot {
  /**
   * Auto-incremented SQLite row ID.
   */
  id: number;
  /**
   * Foreign key to the projects table.
   */
  project_id: number;
  /**
   * ISO 8601 timestamp when this snapshot was recorded.
   */
  created_at: string;
  node_count: number;
  edge_count: number;
  orphan_count: number;
  broken_ref_count: number;
  /**
   * Number of Error-severity integrity findings at snapshot time.
   */
  error_count: number;
  /**
   * Number of Warning-severity integrity findings at snapshot time.
   */
  warning_count: number;
  largest_component_ratio: number;
  orphan_percentage: number;
  avg_degree: number;
  graph_density: number;
  component_count: number;
  pillar_traceability: number;
  bidirectionality_ratio: number;
}
/**
 * A single node in an ancestry chain, ordered from the query artifact up to the pillar or vision root.
 */
export interface AncestryNode {
  /**
   * Artifact ID (e.g. 'EPIC-048').
   */
  id: string;
  /**
   * Human-readable title.
   */
  title: string;
  /**
   * Artifact type string (e.g. 'epic', 'pillar').
   */
  artifact_type: string;
  /**
   * The relationship type connecting this node to the next node upward. Empty string for the terminal (pillar/vision) node.
   */
  relationship: string;
}
/**
 * An ordered path from the query artifact to a pillar or vision root.
 */
export interface AncestryChain {
  /**
   * Ordered from current artifact (index 0) to pillar/vision root (last).
   */
  path: AncestryNode[];
}
/**
 * A downstream artifact with its BFS distance from the query artifact.
 */
export interface TracedArtifact {
  /**
   * Artifact ID.
   */
  id: string;
  /**
   * BFS hops from the query artifact.
   */
  depth: number;
}
/**
 * Full traceability result for a single artifact.
 */
export interface TraceabilityResult {
  /**
   * All paths from the artifact upward to any pillar or vision.
   */
  ancestry_chains: AncestryChain[];
  /**
   * All downstream artifacts with their BFS distance.
   */
  descendants: TracedArtifact[];
  /**
   * IDs of artifacts that share at least one direct parent with this artifact.
   */
  siblings: string[];
  /**
   * Count of distinct descendants within 2 hops.
   */
  impact_radius: number;
  /**
   * True when no path exists to any pillar or vision artifact.
   */
  disconnected: boolean;
}
