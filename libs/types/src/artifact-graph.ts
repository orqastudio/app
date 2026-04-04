/**
 * TypeScript mirrors of the Rust types in `src-tauri/src/domain/artifact_graph.rs`.
 *
 * These types flow across the Tauri IPC boundary and must stay in sync with
 * the Rust structs. The Rust side serialises with serde, so field names use
 * snake_case (matching the Rust struct fields directly).
 */

/** A single artifact node in the bidirectional graph. */
export interface ArtifactNode {
    /** Frontmatter `id` field (e.g. "EPIC-048"). */
    readonly id: string;
    /** Source project name in organisation mode, or null for single-project mode. */
    readonly project?: string | null;
    /** Relative path from the project root (e.g. ".orqa/implementation/epics/EPIC-048.md"). */
    readonly path: string;
    /** Inferred category string (e.g. "epic", "task", "milestone", "idea", "decision"). */
    readonly artifact_type: string;
    /** Frontmatter `title` field, or a humanized fallback from the filename. */
    readonly title: string;
    /** Frontmatter `description` field. */
    readonly description: string | null;
    /** Frontmatter `status` field. */
    readonly status: string | null;
    /** Frontmatter `priority` field (e.g. "P1", "P2", "P3"). */
    readonly priority: string | null;
    /** Full YAML frontmatter parsed into a generic JSON object. */
    readonly frontmatter: Readonly<Record<string, unknown>>;
    /** Forward references declared in this node's frontmatter. */
    readonly references_out: readonly ArtifactRef[];
    /** Backlinks computed from other nodes' `references_out` during graph construction. */
    readonly references_in: readonly ArtifactRef[];
}

/** A directed reference from one artifact to another. */
export interface ArtifactRef {
    /** The artifact ID that is referenced (the link target). */
    readonly target_id: string;
    /** Name of the frontmatter field that contains this reference. */
    readonly field: string;
    /** ID of the artifact that declares this reference (the link source). */
    readonly source_id: string;
    /** Semantic relationship type (e.g. "enforced-by", "grounded"). Only set for refs from the relationships array. */
    readonly relationship_type: string | null;
}

/** Summary statistics about the artifact graph. */
export interface GraphStats {
    /** Total number of nodes (artifacts with an `id` field). */
    readonly node_count: number;
    /** Total number of directed edges (sum of all `references_out` lengths). */
    readonly edge_count: number;
    /** Nodes that have no `references_out` and no `references_in`. */
    readonly orphan_count: number;
    /** References whose `target_id` does not exist in the graph. */
    readonly broken_ref_count: number;
}


/** Artifact type key string — from plugin registry, not a hardcoded enum. */
export type ArtifactGraphType = string;

/** Artifact status string — from project.json, not a hardcoded enum. */
export type CanonicalStatus = string;

/** Alias for CanonicalStatus — used by the frontend. */
export type ArtifactStatus = string;

/**
 * Category of integrity issue found in the artifact graph.
 *
 * Must stay in sync with `IntegrityCategory` in `engine/validation/src/types.rs`
 * and `libs/types/src/platform/validation.schema.json`.
 */
export type IntegrityCategory =
    | "BrokenLink"
    | "TypeConstraintViolation"
    | "RequiredRelationshipMissing"
    | "CardinalityViolation"
    | "CircularDependency"
    | "InvalidStatus"
    | "BodyTextRefWithoutRelationship"
    | "ParentChildInconsistency"
    | "DeliveryPathMismatch"
    | "MissingType"
    | "MissingStatus"
    | "DuplicateRelationship"
    | "FilenameMismatch";

/** Severity of an integrity finding. */
export type IntegritySeverity = "Error" | "Warning" | "Info";

/** A single integrity finding from the graph. */
export interface IntegrityCheck {
    readonly category: IntegrityCategory;
    readonly severity: IntegritySeverity;
    readonly artifact_id: string;
    readonly message: string;
    readonly auto_fixable: boolean;
    readonly fix_description: string | null;
}

/** A fix that was applied to resolve an integrity issue. */
export interface AppliedFix {
    readonly artifact_id: string;
    readonly description: string;
    readonly file_path: string;
}

/** A status transition proposed by the backend transition engine. */
export interface ProposedTransition {
    /** Artifact identifier, e.g. `"EPIC-048"`. */
    readonly artifact_id: string;
    /** Relative path from the project root, e.g. `".orqa/implementation/epics/EPIC-048.md"`. */
    readonly artifact_path: string;
    /** Current `status` frontmatter value. */
    readonly current_status: string;
    /** Status value to transition to. */
    readonly proposed_status: string;
    /** Human-readable explanation of why this transition is proposed. */
    readonly reason: string;
    /** When `true` the backend already applied this transition automatically. */
    readonly auto_apply: boolean;
}

/** A point-in-time snapshot of graph health metrics. */
export interface HealthSnapshot {
    readonly id: number;
    readonly project_id: number;
    readonly node_count: number;
    readonly edge_count: number;
    readonly orphan_count: number;
    readonly broken_ref_count: number;
    readonly error_count: number;
    readonly warning_count: number;
    /** Largest connected component size / total nodes (0.0–1.0). */
    readonly largest_component_ratio: number;
    /** Orphan count as a percentage of total nodes (0.0–100.0). */
    readonly orphan_percentage: number;
    /** Average degree: (edges * 2) / nodes. */
    readonly avg_degree: number;
    /** Edge density: edges / (nodes * (nodes - 1)). */
    readonly graph_density: number;
    /** Number of weakly-connected components. */
    readonly component_count: number;
    /** Percentage of rules with at least one grounded-by → pillar relationship. */
    readonly pillar_traceability: number;
    /** Ratio of typed relationship edges that have their inverse present (0.0–1.0). */
    readonly bidirectionality_ratio: number;
    readonly created_at: string;
}

// ---------------------------------------------------------------------------
// Traceability types
// ---------------------------------------------------------------------------

/**
 * A single node in an ancestry chain, ordered from the query artifact up to
 * the pillar or vision root.
 */
export interface AncestryNode {
    /** Artifact ID (e.g. "EPIC-048"). */
    readonly id: string;
    /** Human-readable title. */
    readonly title: string;
    /** Artifact type string (e.g. "epic", "pillar"). */
    readonly artifact_type: string;
    /**
     * The relationship type connecting this node to the next node upward.
     * Empty string for the terminal (pillar/vision) node.
     */
    readonly relationship: string;
}

/** An ordered path from the query artifact to a pillar or vision root. */
export interface AncestryChain {
    /** Ordered from current artifact (index 0) to pillar/vision root (last). */
    readonly path: readonly AncestryNode[];
}

/** A downstream artifact with its BFS distance from the query artifact. */
export interface TracedArtifact {
    /** Artifact ID. */
    readonly id: string;
    /** BFS hops from the query artifact. */
    readonly depth: number;
}

/** Full traceability result for a single artifact. */
export interface TraceabilityResult {
    /** All paths from the artifact upward to any pillar or vision. */
    readonly ancestry_chains: readonly AncestryChain[];
    /** All downstream artifacts with their BFS distance. */
    readonly descendants: readonly TracedArtifact[];
    /** IDs of artifacts that share at least one direct parent with this artifact. */
    readonly siblings: readonly string[];
    /** Count of distinct descendants within 2 hops. */
    readonly impact_radius: number;
    /** True when no path exists to any pillar or vision artifact. */
    readonly disconnected: boolean;
}

/**
 * Age distribution of pipeline outliers, bucketed by how long ago they were created.
 *
 * Only artifacts that pass all outlier filters (active, in-scope type, past grace period)
 * contribute to the distribution. Artifacts without a `created` field go into `stale`
 * because their age is unknown and they should be investigated.
 */
export interface OutlierAgeDistribution {
    /** Outliers created within the last 7 days — within grace period, informational only. */
    readonly fresh: number;
    /** Outliers created 7–30 days ago — aging, should be connected or archived soon. */
    readonly aging: number;
    /** Outliers created more than 30 days ago (or with no `created` date) — priority action items. */
    readonly stale: number;
}

/**
 * Extended structural health metrics from the backend artifact graph analysis.
 *
 * Returned by the `get_graph_health` Tauri command. Models two named pipelines:
 * - Delivery: task, epic, milestone, idea, research, decision, wireframe
 * - Learning: lesson, rule
 *
 * Artifacts outside both pipelines that are not archived/surpassed/knowledge/doc
 * are counted as outliers needing attention.
 */
export interface GraphHealthData {
    /** Total number of nodes (excluding alias nodes). */
    readonly total_nodes: number;
    /** Total number of directed edges. */
    readonly total_edges: number;
    /** Number of active (non-archived, non-knowledge) artifacts outside both pipelines past their grace period. */
    readonly outlier_count: number;
    /** outlier_count / active_nodes * 100 (0.0–100.0). */
    readonly outlier_percentage: number;
    /** Age distribution of all candidate outliers (fresh ≤7d, aging 7–30d, stale 30d+ or no date). */
    readonly outlier_age_distribution: OutlierAgeDistribution;
    /** Fraction of delivery artifacts in the main delivery component (0.0–1.0). */
    readonly delivery_connectivity: number;
    /** Fraction of learning artifacts connected to each other or to decisions (0.0–1.0). */
    readonly learning_connectivity: number;
    /** Largest component size / total nodes (0.0–1.0). */
    readonly largest_component_ratio: number;
    /** Average number of relationships per node (edges * 2 / nodes). */
    readonly avg_degree: number;
    /** Percentage of non-doc nodes that can trace a path to a pillar artifact (0.0–100.0). */
    readonly pillar_traceability: number;
    /** Number of broken references (target not in graph). */
    readonly broken_ref_count: number;
}
