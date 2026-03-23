/**
 * GraphVisualiser — reactive Svelte 5 wrapper around graph analysis.
 *
 * Consumes graph data from the SDK (via a ReadonlyMap<string, ArtifactNode>)
 * and provides reactive derived properties for health, backbone, gaps,
 * and visualization elements.
 *
 * Usage:
 *   const viz = new GraphVisualiser();
 *   viz.update(artifactGraphSDK.graph);  // call after each graph refresh
 *   viz.graphHealth       // reactive
 *   viz.backboneArtifacts // reactive
 *   viz.knowledgeGaps     // reactive
 *   viz.graphElements     // reactive
 */

import cytoscape from "cytoscape";
import type { ArtifactNode } from "@orqastudio/types";
import type { GraphHealth, BackboneArtifact, KnowledgeGaps, ImpactResult, NodePosition } from "./types.js";
import { computeGraphHealth, computeBackboneArtifacts, computeKnowledgeGaps, traceChain as _traceChain, computeImpact } from "./analysis.js";
import { buildAnalysisElements, buildVisualizationElements } from "./elements.js";

export class GraphVisualiser {
    /** Cached headless Cytoscape instance. Rebuilt on each update(). */
    private _cy: cytoscape.Core | null = null;

    /** Current graph data reference. */
    private _graph: ReadonlyMap<string, ArtifactNode> = new Map();

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
     */
    update(graph: ReadonlyMap<string, ArtifactNode>): void {
        this._graph = graph;
        if (this._cy) {
            this._cy.destroy();
            this._cy = null;
        }
        this._version++;
        this.cachedPositions = [];
    }

    // -----------------------------------------------------------------------
    // Reactive derived properties
    // -----------------------------------------------------------------------

    graphHealth = $derived.by((): GraphHealth => {
        void this._version;
        return computeGraphHealth(this._getCy());
    });

    backboneArtifacts = $derived.by((): BackboneArtifact[] => {
        void this._version;
        return computeBackboneArtifacts(this._getCy(), this._graph);
    });

    knowledgeGaps = $derived.by((): KnowledgeGaps => {
        void this._version;
        return computeKnowledgeGaps(this._graph);
    });

    graphElements = $derived.by((): cytoscape.ElementDefinition[] => {
        void this._version;
        return buildVisualizationElements(this._graph);
    });

    // -----------------------------------------------------------------------
    // Imperative analysis methods
    // -----------------------------------------------------------------------

    /** BFS trace from an artifact. */
    traceChain(id: string, direction: "up" | "down"): string[] {
        return _traceChain(this._getCy(), id, direction);
    }

    /** Impact radius analysis. */
    impactOf(id: string, maxDepth = 2): ImpactResult {
        return computeImpact(this._getCy(), this._graph, id, maxDepth);
    }

    // -----------------------------------------------------------------------
    // Private
    // -----------------------------------------------------------------------

    private _getCy(): cytoscape.Core {
        if (this._cy) return this._cy;
        this._cy = cytoscape({ elements: buildAnalysisElements(this._graph), headless: true });
        return this._cy;
    }
}
