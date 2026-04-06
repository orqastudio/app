<script lang="ts">
	import { onDestroy } from "svelte";
	import cytoscape from "cytoscape";
	import { getStores, logger } from "@orqastudio/sdk";
	import { getGraphViz } from "$lib/graph-viz.svelte";
	import { graphLayoutService } from "$lib/services/graph-layout.svelte";
	import {
		LoadingSpinner,
		Toolbar,
		Caption,
		Button,
		Icon,
		HStack,
		Stack,
		Center,
		Text,
	} from "@orqastudio/svelte-components/pure";
	import GraphHealthPanel from "./GraphHealthPanel.svelte";
	import type { GraphHealthData, HealthSnapshot } from "@orqastudio/types";

	const log = logger("graph-view");
	const { artifactGraphSDK, navigationStore, toast } = getStores();
	const graphViz = getGraphViz();

	// cose-bilkent is no longer needed here — layout runs in the worker.

	let container = $state<HTMLDivElement | undefined>(undefined);
	let cy: cytoscape.Core | null = null;

	// Health panel state
	let healthPanelOpen = $state(true);
	let graphHealth = $state<GraphHealthData | null>(null);
	let healthSnapshots = $state<HealthSnapshot[]>([]);
	let healthLoading = $state(false);

	/**
	 *
	 */
	async function loadHealth(): Promise<void> {
		healthLoading = true;
		try {
			const [health, snapshots] = await Promise.all([
				artifactGraphSDK.getGraphHealth(),
				artifactGraphSDK.getHealthSnapshots(10),
			]);
			graphHealth = health;
			healthSnapshots = snapshots;
		} catch (err: unknown) {
			toast.error(err instanceof Error ? err.message : String(err));
		} finally {
			healthLoading = false;
		}
	}

	// Load health when the graph is available
	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !graphHealth && !healthLoading) {
			void loadHealth();
		}
	});

	/**
	 * Track the positions snapshot we last rendered so we only rebuild when
	 *  positions actually change (not on every reactive read).
	 */
	let lastRenderedPositionCount = 0;
	let lastRenderedNodeCount = 0;

	let resizeObserver: ResizeObserver | null = null;
	let resizeTimer: ReturnType<typeof setTimeout> | null = null;

	/**
	 *
	 * @param el
	 */
	function buildGraph(el: HTMLDivElement): void {
		if (cy) {
			cy.destroy();
			cy = null;
		}

		const elements = graphViz.graphElements;
		const positions = graphLayoutService.positions;

		if (elements.filter((e: cytoscape.ElementDefinition) => e.group === "nodes").length === 0)
			return;
		if (positions.length === 0) return;

		// Apply worker-computed positions to node element definitions.
		const positionMap = new Map(positions.map((p) => [p.id, { x: p.x, y: p.y }]));
		const elementsWithPositions: cytoscape.ElementDefinition[] = elements.map(
			(el: cytoscape.ElementDefinition) => {
				if (el.group === "nodes" && el.data?.id) {
					const pos = positionMap.get(el.data.id as string);
					if (pos) return { ...el, position: pos };
				}
				return el;
			},
		);

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
						"font-size": "9px",
						"font-family": "monospace",
						"font-weight": "normal",
						shape: "round-rectangle",
						width: 80,
						height: 22,
						"text-max-width": "70px",
						"text-wrap": "ellipsis" as unknown as undefined,
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
			// Positions are already pre-computed — use preset layout for instant render.
			layout: { name: "preset" },
			minZoom: 0.1,
			maxZoom: 4,
		});

		cy.fit(undefined, 40);

		// Click handler — navigate to clicked artifact
		cy.on("tap", "node", (evt) => {
			const nodeId = evt.target.id() as string;
			navigationStore.navigateToArtifact(nodeId);
		});

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
		const elements = graphViz.graphElements;
		const positions = graphLayoutService.positions;
		const running = graphLayoutService.layoutRunning;

		const nodeCount = elements.filter(
			(e: cytoscape.ElementDefinition) => e.group === "nodes",
		).length;
		const posCount = positions.length;

		if (!el) return;

		// Request layout computation when we have elements but no positions
		if (nodeCount > 0 && posCount === 0 && !running) {
			graphLayoutService.requestLayout(elements);
			return;
		}

		if (running) return; // Still computing — spinner shown instead
		if (posCount === 0) return; // No positions yet

		// Rebuild if the graph or positions changed since last render.
		if (cy && nodeCount === lastRenderedNodeCount && posCount === lastRenderedPositionCount) return;

		lastRenderedNodeCount = nodeCount;
		lastRenderedPositionCount = posCount;

		requestAnimationFrame(() => {
			try {
				buildGraph(el);
			} catch (err) {
				log.error("Graph build failed", err);
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
			cy.destroy();
			cy = null;
		}
	});
</script>

<Stack height="full" gap={0}>
	<!-- Toolbar -->
	<Toolbar>
		{#snippet left()}
			<Text variant="body-strong">Artifact Graph</Text>
			{#if artifactGraphSDK.stats}
				<Caption>
					{artifactGraphSDK.stats.node_count} nodes · {artifactGraphSDK.stats.edge_count} edges
				</Caption>
			{/if}
		{/snippet}
		{#snippet right()}
			<Button
				variant="ghost"
				size="sm"
				onclick={() => {
					healthPanelOpen = !healthPanelOpen;
				}}
				aria-label={healthPanelOpen ? "Hide health panel" : "Show health panel"}
			>
				<Icon name="activity" size="sm" />
				{healthPanelOpen ? "Hide Health" : "Health"}
			</Button>
		{/snippet}
	</Toolbar>

	<!-- Main content area: graph + health panel side by side -->
	<HStack flex={1} gap={0} align="stretch">
		<!-- Graph area: the bind:this container for Cytoscape is a legitimate exception -->
		<div style="position: relative; flex: 1; overflow: hidden;">
			{#if artifactGraphSDK.loading}
				<Center full>
					<LoadingSpinner size="lg" />
				</Center>
			{:else if artifactGraphSDK.graph.size === 0}
				<Center full>
					<Caption>No artifacts found. Open a project to explore its graph.</Caption>
				</Center>
			{:else}
				<div
					bind:this={container}
					style="height: 100%; width: 100%;"
					role="img"
					aria-label="Full artifact relationship graph"
				></div>
				{#if graphLayoutService.layoutRunning}
					<div
						style="position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 1rem; background: hsl(var(--background) / 0.6); backdrop-filter: blur(2px);"
					>
						<LoadingSpinner size="lg" />
						<Caption>Laying out {artifactGraphSDK.graph.size} nodes…</Caption>
					</div>
				{/if}
			{/if}
		</div>

		<!-- Health panel sidebar -->
		{#if healthPanelOpen}
			<div style="width: 13rem; flex-shrink: 0; overflow: hidden;">
				<GraphHealthPanel
					health={graphHealth}
					snapshots={healthSnapshots}
					loading={healthLoading}
					onRefresh={loadHealth}
				/>
			</div>
		{/if}
	</HStack>
</Stack>
