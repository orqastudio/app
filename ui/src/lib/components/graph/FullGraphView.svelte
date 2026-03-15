<script lang="ts">
	import { onDestroy } from "svelte";
	import cytoscape from "cytoscape";
	// @ts-expect-error — no type declarations for cytoscape-cose-bilkent
	import coseBilkent from "cytoscape-cose-bilkent";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";

	// Register layout extension once
	try {
		cytoscape.use(coseBilkent);
	} catch {
		// Already registered — safe to ignore
	}

	let container = $state<HTMLDivElement | undefined>(undefined);
	let cy: cytoscape.Core | null = null;

	let stabilizing = $state(false);

	/** Track element count so we only rebuild when the graph actually changes. */
	let lastElementCount = 0;

	let resizeObserver: ResizeObserver | null = null;
	let resizeTimer: ReturnType<typeof setTimeout> | null = null;

	function buildGraph(el: HTMLDivElement): void {
		// Destroy existing instance
		if (cy) {
			try {
				artifactGraphSDK.cachedPositions = cy.nodes().map((n) => ({
					id: n.id(), x: n.position().x, y: n.position().y,
				}));
			} catch { /* ignore */ }
			cy.destroy();
			cy = null;
		}

		const elements = artifactGraphSDK.graphElements;
		if (elements.filter((e) => e.group === "nodes").length === 0) return;

		const positions = artifactGraphSDK.cachedPositions;
		const hasCachedPositions = positions.length > 0;

		if (!hasCachedPositions) {
			stabilizing = true;
		}

		// Apply cached positions to node element definitions
		let elementsWithPositions: cytoscape.ElementDefinition[];
		if (hasCachedPositions) {
			const positionMap = new Map(positions.map((p) => [p.id, { x: p.x, y: p.y }]));
			elementsWithPositions = elements.map((el) => {
				if (el.group === "nodes" && el.data?.id) {
					const pos = positionMap.get(el.data.id as string);
					if (pos) return { ...el, position: pos };
				}
				return el;
			});
		} else {
			elementsWithPositions = elements;
		}

		cy = cytoscape({
			container: el,
			elements: elementsWithPositions,
			style: [
				{
					selector: "node",
					style: {
						label: "data(label)",
						"background-color": "data(color)",
						"background-opacity": 0.1,
						"border-width": 1,
						"border-color": "data(color)",
						"border-opacity": 0.3,
						color: "data(color)",
						"text-valign": "center",
						"text-halign": "center",
						"font-size": "11px",
						"font-family": "'JetBrains Mono', monospace",
						"font-weight": "500",
						shape: "round-rectangle",
						width: "label",
						height: 20,
						"padding-left": "6px",
						"padding-right": "6px",
						"text-margin-y": 0,
					},
				},
				{
					selector: "node:selected",
					style: {
						"border-width": 2,
						"border-color": "#ffffff",
					},
				},
				{
					selector: "edge",
					style: {
						width: 1,
						"line-color": "#4b5563",
						"target-arrow-color": "#4b5563",
						"target-arrow-shape": "triangle",
						"curve-style": "bezier",
						opacity: 0.5,
					},
				},
			],
			layout: hasCachedPositions
				? { name: "preset" }
				: ({
						name: "cose-bilkent",
						animate: "end",
						animationDuration: 500,
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
					} as cytoscape.LayoutOptions),
			minZoom: 0.1,
			maxZoom: 4,
			wheelSensitivity: 0.3,
		});

		// Click handler — navigate to clicked artifact
		cy.on("tap", "node", (evt) => {
			const nodeId = evt.target.id() as string;
			navigationStore.navigateToArtifact(nodeId);
		});

		if (hasCachedPositions) {
			stabilizing = false;
			cy.fit(undefined, 40);
		} else {
			// cose-bilkent fires layoutstop when done
			cy.one("layoutstop", () => {
				stabilizing = false;
				cy?.fit(undefined, 40);
				// Store positions back to the SDK so subsequent renders skip layout
				if (cy) {
					artifactGraphSDK.cachedPositions = cy.nodes().map((n) => ({
						id: n.id(),
						x: n.position().x,
						y: n.position().y,
					}));
				}
			});
		}

		// Debounced resize observer
		if (resizeObserver) resizeObserver.disconnect();
		resizeObserver = new ResizeObserver(() => {
			if (resizeTimer) clearTimeout(resizeTimer);
			resizeTimer = setTimeout(() => {
				cy?.resize();
			}, 150);
		});
		resizeObserver.observe(el);
	}

	$effect(() => {
		const el = container;
		const elements = artifactGraphSDK.graphElements;
		const nodeCount = elements.filter((e) => e.group === "nodes").length;

		if (!el) return;
		if (cy && nodeCount === lastElementCount) return;

		lastElementCount = nodeCount;
		stabilizing = true;
		// Defer to next frame so loading overlay renders before heavy work
		requestAnimationFrame(() => {
			try {
				buildGraph(el);
			} catch (err) {
				console.error("Graph build failed:", err);
				stabilizing = false;
			}
		});
	});

	onDestroy(() => {
		if (resizeObserver) {
			resizeObserver.disconnect();
			resizeObserver = null;
		}
		if (resizeTimer) clearTimeout(resizeTimer);
		if (cy) {
			// Save final positions back to SDK before destroy
			artifactGraphSDK.cachedPositions = cy.nodes().map((n) => ({
				id: n.id(),
				x: n.position().x,
				y: n.position().y,
			}));
			cy.destroy();
			cy = null;
		}
	});
</script>

<div class="relative flex h-full flex-col">
	<div class="flex items-center justify-between border-b border-border px-4 py-2">
		<div class="flex items-center gap-2">
			<span class="text-sm font-medium">Artifact Graph</span>
			{#if artifactGraphSDK.stats}
				<span class="text-xs text-muted-foreground">
					{artifactGraphSDK.stats.node_count} nodes · {artifactGraphSDK.stats.edge_count} edges
				</span>
			{/if}
		</div>
	</div>

	{#if artifactGraphSDK.loading}
		<div class="flex flex-1 items-center justify-center">
			<LoadingSpinner size="lg" />
		</div>
	{:else if artifactGraphSDK.graph.size === 0}
		<div class="flex flex-1 items-center justify-center text-sm text-muted-foreground">
			No artifacts found. Open a project to explore its graph.
		</div>
	{:else}
		<div class="relative flex-1">
			<div
				bind:this={container}
				class="h-full w-full"
				role="img"
				aria-label="Full artifact relationship graph"
			></div>
			{#if stabilizing}
				<div class="absolute inset-0 flex flex-col items-center justify-center gap-4 bg-background/60 backdrop-blur-[2px]">
					<LoadingSpinner size="lg" />
					<p class="text-sm font-medium text-muted-foreground">
						Laying out {artifactGraphSDK.graph.size} nodes…
					</p>
				</div>
			{/if}
		</div>
	{/if}
</div>
