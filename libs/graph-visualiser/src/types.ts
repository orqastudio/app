import type { ArtifactNode, ArtifactRef } from "@orqastudio/types";

/** Cached node position from a layout computation. */
export interface NodePosition {
    id: string;
    x: number;
    y: number;
}

/** Structural health metrics derived from the artifact graph topology. */
export interface GraphHealth {
    /** Number of disconnected subgraphs (weakly connected components). */
    componentCount: number;
    /** Nodes with zero in-degree (nothing points to them). */
    orphanCount: number;
    /** orphanCount / totalNodes * 100, rounded to 1 decimal place. */
    orphanPercentage: number;
    /** Average number of connections (in + out) per node. */
    avgDegree: number;
    /** Largest component size / totalNodes. 1.0 means a fully connected graph. */
    largestComponentRatio: number;
    /** Total number of nodes in the graph. */
    totalNodes: number;
    /** Total number of directed edges in the graph. */
    totalEdges: number;
}

/** A high-PageRank artifact that many others reference or depend upon. */
export interface BackboneArtifact {
    /** Artifact ID (e.g. "RULE-006"). */
    id: string;
    /** Display title. */
    title: string;
    /** Artifact type string (e.g. "rule", "epic"). */
    type: string;
    /** Normalised PageRank score (0–1). */
    rank: number;
}

/** Knowledge gaps detected from the relationship graph. */
export interface KnowledgeGaps {
    /** Rule IDs that have no `grounded-by` edge pointing to any PILLAR artifact. */
    ungroundedRules: string[];
    /** Knowledge IDs that have no `employs` edge from any AGENT artifact. */
    unusedKnowledge: string[];
    /** Decision IDs that carry no outgoing `governs` or `drives` edge. */
    unenforcedDecisions: string[];
}

/** Impact analysis result for a single artifact. */
export interface ImpactResult {
    total: number;
    artifacts: Array<{ id: string; type: string; distance: number }>;
}

/**
 * Read-only view of the artifact graph data.
 * Implemented by the SDK's ArtifactGraphSDK — this interface decouples
 * the visualiser from the SDK's full API.
 */
export interface GraphDataSource {
    /** All nodes keyed by artifact ID. */
    readonly graph: ReadonlyMap<string, ArtifactNode>;
}
