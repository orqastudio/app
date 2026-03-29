/**
 * Artifact graph scanner and query engine for CLI usage.
 *
 * Delegates to the orqa-validation daemon (localhost:9120 by default) for all graph
 * operations. Falls back to the `orqa-validation` binary when the daemon
 * is unreachable. The CLI no longer reimplements scanning, type inference,
 * query filtering, or stats computation — these all live canonically in
 * the Rust validation crate.
 */

import {
	callDaemonGraph,
	type DaemonArtifactNode,
} from "./daemon-client.js";

// ---------------------------------------------------------------------------
// Types (preserved for callers)
// ---------------------------------------------------------------------------

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
	relationships: Array<{ target: string; type: string }>;
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

// ---------------------------------------------------------------------------
// Conversion: daemon ArtifactNode → CLI GraphNode
// ---------------------------------------------------------------------------

function toGraphNode(node: DaemonArtifactNode): GraphNode {
	// Merge references_out into the relationships format callers expect.
	// Only include refs that have an explicit relationship_type (from the
	// `relationships` frontmatter array). Field-based refs (e.g. "epic",
	// "milestone") are still accessible via frontmatter but don't appear
	// as typed relationships.
	const relationships: Array<{ target: string; type: string }> = [];
	for (const ref of node.references_out) {
		relationships.push({
			target: ref.target_id,
			type: ref.relationship_type ?? ref.field,
		});
	}

	return {
		id: node.id,
		type: node.artifact_type,
		title: node.title,
		status: node.status ?? "unknown",
		path: node.path,
		relationships,
		frontmatter: node.frontmatter,
	};
}

// ---------------------------------------------------------------------------
// Public API (signatures preserved)
// ---------------------------------------------------------------------------

/**
 * Scan the `.orqa/` directory and build an in-memory artifact graph.
 *
 * Delegates to the daemon's POST /query endpoint (no filters = all nodes).
 * @returns Array of all graph nodes.
 */
export async function scanArtifactGraph(): Promise<GraphNode[]> {
	const daemonNodes = await callDaemonGraph<DaemonArtifactNode[]>(
		"POST",
		"/query",
		{},
	);
	return daemonNodes.map(toGraphNode);
}

/**
 * Query the artifact graph with filters.
 *
 * Delegates to the daemon's POST /query endpoint with type/status/search
 * filters. Post-filters locally for relatedTo, relationshipType, and limit
 * since the daemon doesn't support those directly.
 * @param _nodesOrOptions - Legacy nodes array (unused) or query options.
 * @param optionsArg - Query options when called with the legacy two-argument signature.
 * @returns Array of matching graph nodes.
 */
export async function queryGraph(
	_nodesOrOptions: GraphNode[] | GraphQueryOptions,
	optionsArg?: GraphQueryOptions,
): Promise<GraphNode[]> {
	// Support both old signature (nodes, options) and direct (options) call.
	const options: GraphQueryOptions =
		optionsArg ?? (_nodesOrOptions as GraphQueryOptions);

	// Build daemon query body with the filters it supports natively.
	const body: Record<string, unknown> = {};
	if (options.type) {
		// Daemon only accepts a single type string, not an array.
		body.type = Array.isArray(options.type) ? options.type[0] : options.type;
	}
	if (options.status) {
		body.status = Array.isArray(options.status) ? options.status[0] : options.status;
	}
	if (options.search) {
		body.search = options.search;
	}

	const daemonNodes = await callDaemonGraph<DaemonArtifactNode[]>(
		"POST",
		"/query",
		body,
	);

	let results = daemonNodes.map(toGraphNode);

	// Apply client-side filters the daemon doesn't support.
	if (options.type && Array.isArray(options.type) && options.type.length > 1) {
		const types = options.type;
		results = results.filter((n) => types.includes(n.type));
	}

	if (options.status && Array.isArray(options.status) && options.status.length > 1) {
		const statuses = options.status;
		results = results.filter((n) => statuses.includes(n.status));
	}

	if (options.relatedTo) {
		const target = options.relatedTo;
		results = results.filter((n) =>
			n.relationships.some((r) => r.target === target),
		);
	}

	if (options.relationshipType) {
		const relType = options.relationshipType;
		results = results.filter((n) =>
			n.relationships.some((r) => r.type === relType),
		);
	}

	if (options.limit) {
		results = results.slice(0, options.limit);
	}

	return results;
}

/**
 * Get aggregate statistics for the graph.
 *
 * Fetches all nodes from the daemon and computes per-type/per-status
 * breakdowns client-side, since the daemon's /health endpoint only
 * provides total counts.
 * @param _nodes - Optional pre-fetched nodes to avoid an extra daemon call.
 * @returns Aggregate statistics for the graph.
 */
export async function getGraphStats(_nodes?: GraphNode[]): Promise<GraphStats> {
	// If caller already has nodes, compute locally to avoid extra daemon call.
	if (_nodes && _nodes.length > 0) {
		return computeStatsLocally(_nodes);
	}

	// Otherwise fetch all nodes for full breakdown.
	const nodes = await scanArtifactGraph();
	return computeStatsLocally(nodes);
}

function computeStatsLocally(nodes: GraphNode[]): GraphStats {
	const byType: Record<string, number> = {};
	const byStatus: Record<string, number> = {};
	let totalRelationships = 0;

	for (const node of nodes) {
		byType[node.type] = (byType[node.type] ?? 0) + 1;
		byStatus[node.status] = (byStatus[node.status] ?? 0) + 1;
		totalRelationships += node.relationships.length;
	}

	return {
		totalNodes: nodes.length,
		totalRelationships,
		byType,
		byStatus,
	};
}
