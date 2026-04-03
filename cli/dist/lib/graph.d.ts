/**
 * Artifact graph scanner and query engine for CLI usage.
 *
 * Delegates to the orqa-validation daemon (localhost:10100 by default) for all graph
 * operations. Falls back to the `orqa-validation` binary when the daemon
 * is unreachable. The CLI no longer reimplements scanning, type inference,
 * query filtering, or stats computation — these all live canonically in
 * the Rust validation crate.
 */
export interface GraphNode {
    /** Artifact ID (e.g. "EPIC-082"). */
    id: string;
    /** Artifact type (e.g. "epic", "task", "decision"). */
    type: string;
    /** Title from frontmatter or first heading. */
    title: string;
    /** Current status. */
    status: string;
    /** Relative file path from project root. */
    path: string;
    /** Relationships declared in frontmatter. */
    relationships: Array<{
        target: string;
        type: string;
    }>;
    /** Raw frontmatter fields. */
    frontmatter: Record<string, unknown>;
}
export interface GraphQueryOptions {
    /** Filter by artifact type(s). */
    type?: string | string[];
    /** Filter by status(es). */
    status?: string | string[];
    /** Filter by relationship target. */
    relatedTo?: string;
    /** Filter by relationship type. */
    relationshipType?: string;
    /** Text search in title. */
    search?: string;
    /** Limit number of results. */
    limit?: number;
}
export interface GraphStats {
    totalNodes: number;
    totalRelationships: number;
    byType: Record<string, number>;
    byStatus: Record<string, number>;
}
/**
 * Scan the `.orqa/` directory and build an in-memory artifact graph.
 *
 * Delegates to the daemon's GET /artifacts endpoint (no filters = all nodes).
 * @returns Array of all graph nodes.
 */
export declare function scanArtifactGraph(): Promise<GraphNode[]>;
/**
 * Query the artifact graph with filters.
 *
 * Delegates to the daemon's GET /artifacts endpoint with type/status/search
 * filters. Post-filters locally for relatedTo, relationshipType, and limit
 * since the daemon doesn't support those directly.
 * @param _nodesOrOptions - Legacy nodes array (unused) or query options.
 * @param optionsArg - Query options when called with the legacy two-argument signature.
 * @returns Array of matching graph nodes.
 */
export declare function queryGraph(_nodesOrOptions: GraphNode[] | GraphQueryOptions, optionsArg?: GraphQueryOptions): Promise<GraphNode[]>;
/**
 * Get aggregate statistics for the graph.
 *
 * Fetches all nodes from the daemon and computes per-type/per-status
 * breakdowns client-side, since the daemon's /health endpoint only
 * provides total counts.
 * @param _nodes - Optional pre-fetched nodes to avoid an extra daemon call.
 * @returns Aggregate statistics for the graph.
 */
export declare function getGraphStats(_nodes?: GraphNode[]): Promise<GraphStats>;
//# sourceMappingURL=graph.d.ts.map