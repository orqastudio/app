<script lang="ts">
	import { onDestroy } from "svelte";
	import cytoscape from "cytoscape";
	// @ts-expect-error — no type declarations for cytoscape-cose-bilkent
	import coseBilkent from "cytoscape-cose-bilkent";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";

	// Register layout extension once (safe to call multiple times — cytoscape deduplicates)
	cytoscape.use(coseBilkent);

	let container = $state<HTMLDivElement | undefined>(undefined);
	let cy: cytoscape.Core | null = null;

	let stabilizing = $state(false);

	/** Cached graph size — only rebuild when this changes. */
	let lastGraphSize = 0;

	/** Cached node positions from previous layout run. */
	let cachedPositions: Array<{ id: string; x: number; y: number }> = [];

	let resizeObserver: ResizeObserver | null = null;
	let resizeTimer: ReturnType<typeof setTimeout> | null = null;

	const TYPE_COLORS: Record<string, string> = {
		epic: "#3b82f6",
		task: "#10b981",
		milestone: "#f59e0b",
		idea: "#a855f7",
		decision: "#ec4899",
		research: "#06b6d4",
		lesson: "#f97316",
		rule: "#ef4444",
		agent: "#8b5cf6",
		skill: "#14b8a6",
		hook: "#6366f1",
		pillar: "#d97706",
		doc: "#9ca3af",
	};

	function hexFromDotClass(dotClass: string): string {
		if (dotClass.includes("blue-500")) return "#3b82f6";
		if (dotClass.includes("emerald-500")) return "#10b981";
		if (dotClass.includes("amber-500")) return "#f59e0b";
		if (dotClass.includes("purple-500")) return "#a855f7";
		if (dotClass.includes("destructive") || dotClass.includes("red")) return "#ef4444";
		return "#6b7280";
	}

	function resolveNodeColor(status: string | null, artifactType: string): string {
		if (status) return hexFromDotClass(statusColor(status));
		return TYPE_COLORS[artifactType] ?? "#6b7280";
	}

	function buildGraph(el: HTMLDivElement): void {
		// Save positions and destroy existing instance
		if (cy) {
			cachedPositions = cy.nodes().map((n) => ({
				id: n.id(),
				x: n.position().x,
				y: n.position().y,
			}));
			cy.destroy();
			cy = null;
		}

		const graphNodes = [...artifactGraphSDK.graph.values()];
		if (graphNodes.length === 0) return;

		const positionMap = new Map(cachedPositions.map((p) => [p.id, { x: p.x, y: p.y }]));
		const hasCachedPositions = cachedPositions.length > 0;

		if (!hasCachedPositions) {
			stabilizing = true;
		}

		// Build elements
		const edgeKeys = new Set<string>();
		const elements: cytoscape.ElementDefinition[] = [];

		for (const node of graphNodes) {
			const color = resolveNodeColor(node.status ?? null, node.artifact_type);
			const cached = positionMap.get(node.id);
			const elementDef: cytoscape.ElementDefinition = {
				group: "nodes",
				data: {
					id: node.id,
					label: node.id,
					color,
					tooltip: `${node.title}\n${node.artifact_type}${node.status ? ` · ${node.status}` : ""}`,
				},
			};
			if (cached) {
				elementDef.position = { x: cached.x, y: cached.y };
			}
			elements.push(elementDef);
		}

		for (const node of graphNodes) {
			for (const ref of node.references_out) {
				if (!artifactGraphSDK.graph.has(ref.target_id)) continue;
				const key = `${ref.source_id}->${ref.target_id}`;
				if (edgeKeys.has(key)) continue;
				edgeKeys.add(key);
				elements.push({
					group: "edges",
					data: {
						id: key,
						source: ref.source_id,
						target: ref.target_id,
					},
				});
			}
		}

		cy = cytoscape({
			container: el,
			elements,
			style: [
				{
					selector: "node",
					style: {
						label: "data(label)",
						"background-color": "data(color)",
						color: "#fff",
						"text-valign": "center",
						"text-halign": "center",
						"font-size": "10px",
						"font-family": "monospace",
						width: 24,
						height: 24,
						"text-outline-width": 2,
						"text-outline-color": "data(color)",
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
				// Cache positions after layout
				if (cy) {
					cachedPositions = cy.nodes().map((n) => ({
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
		const currentSize = artifactGraphSDK.graph.size;

		if (!el) return;
		if (cy && currentSize === lastGraphSize) return;

		lastGraphSize = currentSize;
		buildGraph(el);
	});

	onDestroy(() => {
		if (resizeObserver) {
			resizeObserver.disconnect();
			resizeObserver = null;
		}
		if (resizeTimer) clearTimeout(resizeTimer);
		if (cy) {
			// Save final positions before destroy
			cachedPositions = cy.nodes().map((n) => ({
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
