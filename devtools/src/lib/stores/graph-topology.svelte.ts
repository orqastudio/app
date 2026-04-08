// Graph topology store — receives the dependency graph from the process manager
// and tracks per-node status from log events. Provides the data model for the
// process graph view.

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { events } from "./log-store.svelte.js";

/** A node in the dependency graph as emitted by the process manager. */
export interface GraphNode {
	readonly id: string;
	readonly name: string;
	readonly kind:
		| "ts-library"
		| "svelte-library"
		| "rust-workspace"
		| "tauri-app"
		| "service"
		| "plugin";
	status: string;
	readonly dependsOn: string[];
	readonly dependents: string[];
}

/** The full topology payload emitted once at startup. */
interface GraphTopologyPayload {
	type: "graph-topology";
	nodes: GraphNode[];
}

// Reactive topology state.
export const topology = $state<{ nodes: GraphNode[] }>({ nodes: [] });

// Whether the topology has been loaded.
export const topologyLoaded = $state<{ value: boolean }>({ value: false });

/**
 * Derive error and warning counts per node from the log event buffer.
 * Keys are node IDs, values are { errors, warnings }.
 * @returns Record mapping node ID to error/warning counts.
 */
export function nodeCounts(): Record<string, { errors: number; warnings: number }> {
	const counts: Record<string, { errors: number; warnings: number }> = {};

	for (const node of topology.nodes) {
		counts[node.id] = { errors: 0, warnings: 0 };
	}

	for (const ev of events) {
		// Match process:{nodeId} categories to graph nodes.
		if (ev.category.startsWith("process:")) {
			const nodeId = ev.category.slice("process:".length);
			if (counts[nodeId]) {
				if (ev.level === "Error") counts[nodeId].errors++;
				if (ev.level === "Warn") counts[nodeId].warnings++;
			}
		}
	}

	return counts;
}

/**
 * Get the category filter string for a graph node. Used for click-through
 * to the Stream tab pre-filtered for that node's events.
 * @param nodeId - The graph node ID to build a filter for.
 * @returns The category string to filter the stream by.
 */
export function categoryFilterForNode(nodeId: string): string {
	return `process:${nodeId}`;
}

/**
 * Initialize the topology store. Queries the backend for the current topology
 * (if already emitted) and subscribes to the orqa://graph-topology Tauri event
 * for live updates.
 */
export async function initTopology(): Promise<void> {
	// Try to load from IPC (topology may have been emitted before we mounted).
	try {
		const result = await invoke<GraphTopologyPayload | null>("devtools_graph_topology");
		if (result?.nodes) {
			topology.nodes = result.nodes;
			topologyLoaded.value = true;
		}
	} catch {
		// Not available yet — will arrive via event.
	}

	// Subscribe to live topology events.
	listen<GraphTopologyPayload>("orqa://graph-topology", (event) => {
		if (event.payload?.nodes) {
			topology.nodes = event.payload.nodes;
			topologyLoaded.value = true;
		}
	});
}

/**
 * Update a node's status from a process manager event.
 * Called from the process tracker when PM events arrive.
 * @param nodeId - The graph node ID.
 * @param status - The new status string.
 */
export function updateNodeStatus(nodeId: string, status: string): void {
	const node = topology.nodes.find((n) => n.id === nodeId);
	if (node) {
		node.status = status;
	}
}

/**
 * Ensure a daemon-reported subprocess appears in the topology graph.
 * Adds the node if it doesn't already exist. Used for processes managed
 * by the daemon (LPS, MCP/sidecar) that aren't in the PM dependency graph.
 * @param id - The process source ID (e.g. "lsp", "mcp").
 * @param name - Human-readable process name.
 * @param status - Current process status string.
 * @param parentId - ID of the parent node (typically "daemon").
 */
export function ensureDaemonSubprocess(
	id: string,
	name: string,
	status: string,
	parentId: string,
): void {
	const existing = topology.nodes.find((n) => n.id === id);
	if (existing) {
		existing.status = status;
		return;
	}
	// Add as a new node with a dependency on its parent.
	topology.nodes.push({
		id,
		name,
		kind: "service",
		status,
		dependsOn: [parentId],
		dependents: [],
	});
}
