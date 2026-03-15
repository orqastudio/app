/**
 * Graph Layout Service
 *
 * A singleton Svelte 5 runes service that owns the graph-layout Web Worker.
 * The worker is created once and lives for the lifetime of the app — it is
 * never terminated between layouts.
 *
 * Usage:
 *   graphLayoutService.requestLayout(elements);  // fire-and-forget
 *   graphLayoutService.positions                 // reactive, updated when done
 *   graphLayoutService.layoutRunning             // true while computing
 *   graphLayoutService.layoutProgress            // 0–100
 */

import type cytoscape from "cytoscape";
import type { WorkerRequest, WorkerResponse } from "$lib/workers/graph-layout.worker";

// ---------------------------------------------------------------------------
// Service class
// ---------------------------------------------------------------------------

class GraphLayoutService {
    /** Computed node positions from the most recent layout run. */
    positions = $state<Array<{ id: string; x: number; y: number }>>([]);

    /** True while the worker is computing a layout. */
    layoutRunning = $state(false);

    /** Rough layout progress, 0–100. */
    layoutProgress = $state(0);

    private worker: Worker;

    /**
     * Element count from the last `requestLayout` call that triggered a run.
     * Used for cache invalidation — if the count is the same and we already
     * have positions, skip the expensive layout.
     */
    private lastElementCount = 0;

    constructor() {
        // Vite handles Worker bundling when using `new URL(..., import.meta.url)`.
        this.worker = new Worker(
            new URL("../workers/graph-layout.worker.ts", import.meta.url),
            { type: "module" },
        );

        this.worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
            const msg = event.data;

            if (msg.type === "positions") {
                console.log(`[graph-layout-service] Received ${msg.positions.length} positions from worker`);
                this.positions = msg.positions;
                this.layoutRunning = false;
                this.layoutProgress = 100;
            } else if (msg.type === "progress") {
                this.layoutProgress = msg.percent;
            } else if (msg.type === "error") {
                console.error("[graph-layout-service] worker error:", msg.message);
                this.layoutRunning = false;
                this.layoutProgress = 0;
            }
        };

        this.worker.onerror = (err) => {
            console.error("[graph-layout-service] worker uncaught error:", err);
            this.layoutRunning = false;
            this.layoutProgress = 0;
        };
    }

    /**
     * Request a layout computation for the given element set.
     *
     * If the node count matches the previous run AND we already have cached
     * positions, the request is a no-op (positions are still valid).
     *
     * @param elements Cytoscape element definitions (nodes + edges).
     */
    requestLayout(elements: cytoscape.ElementDefinition[]): void {
        const nodeCount = elements.filter((e) => e.group === "nodes").length;

        // Cache hit — skip if node count unchanged and positions exist.
        if (nodeCount === this.lastElementCount && this.positions.length > 0) {
            return;
        }

        this.lastElementCount = nodeCount;
        this.layoutRunning = true;
        this.layoutProgress = 0;

        // Strip reactive proxies before transferring to the worker — plain
        // objects cross the structured-clone boundary cleanly.
        const plainElements = elements.map((el) => ({
            group: el.group ?? "nodes",
            data: { ...(el.data ?? {}) },
        }));

        const nodeElements = plainElements.filter(e => e.group === "nodes");
        const edgeElements = plainElements.filter(e => e.group === "edges");
        console.log(`[graph-layout-service] Sending ${nodeElements.length} nodes, ${edgeElements.length} edges to worker`);

        const request: WorkerRequest = { type: "layout", elements: plainElements };
        this.worker.postMessage(request);
    }

    /**
     * Invalidate the position cache so the next `requestLayout` call always
     * triggers a fresh layout, even if the node count is the same.
     *
     * Called by `artifactGraphSDK._fetchAll()` when the graph refreshes.
     */
    invalidate(): void {
        this.lastElementCount = 0;
        this.positions = [];
    }
}

// ---------------------------------------------------------------------------
// Singleton export
// ---------------------------------------------------------------------------

export const graphLayoutService = new GraphLayoutService();
