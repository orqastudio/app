/**
 * Graph Layout Web Worker
 *
 * Runs cose-bilkent layout in a background thread so the main thread is never
 * blocked by the O(n²) force-directed algorithm.  The worker is kept alive
 * after each layout so subsequent requests reuse the same thread.
 *
 * Message protocol
 * ----------------
 * Main → Worker:  WorkerRequest  (type: 'layout')
 * Worker → Main:  WorkerResponse (type: 'positions' | 'progress' | 'error')
 */

import cytoscape from "cytoscape";
// @ts-expect-error — no type declarations for cytoscape-cose-bilkent
import coseBilkent from "cytoscape-cose-bilkent";

// Register the layout extension once for this worker's cytoscape scope.
try {
	cytoscape.use(coseBilkent);
} catch (err) {
	// Already registered — safe to ignore (shouldn't happen in a fresh worker).
	// Cannot import logger in a web worker; log via postMessage if needed.
	console.debug(
		"[graph-layout-worker] cose-bilkent registration skipped (already registered)",
		err,
	);
}

// ---------------------------------------------------------------------------
// Message types
// ---------------------------------------------------------------------------

export type WorkerRequest = {
	readonly type: "layout";
	readonly elements: ReadonlyArray<{
		readonly group: string;
		readonly data: Readonly<Record<string, unknown>>;
	}>;
};

export type WorkerResponse =
	| {
			readonly type: "positions";
			readonly positions: ReadonlyArray<{
				readonly id: string;
				readonly x: number;
				readonly y: number;
			}>;
	  }
	| { readonly type: "progress"; readonly percent: number }
	| { readonly type: "error"; readonly message: string };

// ---------------------------------------------------------------------------
// Layout handler
// ---------------------------------------------------------------------------

function runLayout(elements: WorkerRequest["elements"]): void {
	try {
		const cy = cytoscape({
			headless: true,
			elements: elements as cytoscape.ElementDefinition[],
		});

		const nodeCount = cy.nodes().length;

		if (nodeCount === 0) {
			const response: WorkerResponse = { type: "positions", positions: [] };
			self.postMessage(response);
			cy.destroy();
			return;
		}

		// Emit an initial progress tick so the UI can show the spinner
		// immediately rather than waiting for the layout to finish.
		const startResponse: WorkerResponse = { type: "progress", percent: 5 };
		self.postMessage(startResponse);

		// Run cose-bilkent synchronously (no animation in headless mode).
		const layout = cy.layout({
			name: "cose-bilkent",
			animate: false,
			randomize: true,
			nodeRepulsion: 4500,
			idealEdgeLength: 100,
			edgeElasticity: 0.45,
			nestingFactor: 0.1,
			gravity: 0.25,
			numIter: 2500,
			tile: true,
			tilingPaddingVertical: 10,
			tilingPaddingHorizontal: 10,
		} as cytoscape.LayoutOptions);

		layout.run();

		const midResponse: WorkerResponse = { type: "progress", percent: 90 };
		self.postMessage(midResponse);

		// Collect final positions.
		const positions: Array<{ id: string; x: number; y: number }> = [];
		cy.nodes().forEach((node) => {
			const pos = node.position();
			positions.push({ id: node.id(), x: pos.x, y: pos.y });
		});

		cy.destroy();

		const doneResponse: WorkerResponse = { type: "positions", positions };
		self.postMessage(doneResponse);
	} catch (err: unknown) {
		const message = err instanceof Error ? err.message : String(err);
		console.error("[graph-layout-worker] Layout computation failed", {
			nodeCount: elements.length,
			err,
		});
		const errorResponse: WorkerResponse = {
			type: "error",
			message: `Graph layout failed (${elements.length} elements): ${message}`,
		};
		self.postMessage(errorResponse);
	}
}

// ---------------------------------------------------------------------------
// Message dispatch
// ---------------------------------------------------------------------------

self.onmessage = (event: MessageEvent<WorkerRequest>) => {
	const req = event.data;
	if (req.type === "layout") {
		runLayout(req.elements);
	}
};
