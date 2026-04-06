/**
 * GraphVisualiser — reactive Svelte 5 wrapper for graph visualization.
 *
 * Consumes graph data from the SDK (via a ReadonlyMap<string, ArtifactNode>)
 * and provides reactive derived properties for visualization elements.
 *
 * Analysis (health, backbone, gaps, traceability, impact) is handled by the
 * Rust daemon — see the SDK's ArtifactGraphSDK for those APIs.
 *
 * Usage:
 *   const viz = new GraphVisualiser();
 *   viz.update(artifactGraphSDK.graph);  // call after each graph refresh
 *   viz.graphElements     // reactive Cytoscape element definitions
 */

import type cytoscape from "cytoscape";
import type { ArtifactNode } from "@orqastudio/types";
import type { NodePosition } from "./types.js";
import { SvelteMap } from "svelte/reactivity";
import { buildVisualizationElements } from "./elements.js";

/**
 * Reactive visualiser class that wraps graph data and exposes derived Cytoscape elements.
 */
export class GraphVisualiser {
	/** Current graph data reference. */
	private _graph: ReadonlyMap<string, ArtifactNode> = $state(new SvelteMap());

	/** Version counter for reactive tracking. */
	private _version = $state(0);

	/** Cached node positions from the last layout computation. */
	cachedPositions = $state<NodePosition[]>([]);

	// -----------------------------------------------------------------------
	// Lifecycle
	// -----------------------------------------------------------------------

	/**
	 * Update the visualiser with new graph data.
	 * Call this whenever the SDK's graph refreshes.
	 * @param graph - Latest artifact graph map from the SDK.
	 */
	update(graph: ReadonlyMap<string, ArtifactNode>): void {
		this._graph = graph;
		this._version++;
		this.cachedPositions = [];
	}

	// -----------------------------------------------------------------------
	// Reactive derived properties
	// -----------------------------------------------------------------------

	graphElements = $derived.by((): cytoscape.ElementDefinition[] => {
		void this._version;
		return buildVisualizationElements(this._graph);
	});
}
