<script lang="ts">
	import { onDestroy } from "svelte";
	import { SvelteSet } from "svelte/reactivity";
	import { Network } from "vis-network";
	import type { Node, Edge, Options } from "vis-network";
	import { DataSet } from "vis-data";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";

	/** Container div bound by the template. */
	let container = $state<HTMLDivElement | undefined>(undefined);

	/** The vis-network instance, cleaned up on destroy. */
	let network: Network | null = null;

	/** Whether the graph is still stabilizing. */
	let stabilizing = $state(false);

	/** Stabilization progress 0-100. */
	let stabilizationProgress = $state(0);

	/** Cached graph size to avoid re-stabilizing when nothing changed. */
	let lastGraphSize = 0;

	/** Map Tailwind dot class to a hex color for vis-network. */
	function hexFromDotClass(dotClass: string): string {
		if (dotClass.includes("blue-500")) return "#3b82f6";
		if (dotClass.includes("emerald-500")) return "#10b981";
		if (dotClass.includes("amber-500")) return "#f59e0b";
		if (dotClass.includes("purple-500")) return "#a855f7";
		if (dotClass.includes("destructive") || dotClass.includes("red")) return "#ef4444";
		return "#6b7280";
	}

	/** Resolve node color by status. */
	function resolveNodeColor(status: string | null): string {
		if (!status) return "#6b7280";
		return hexFromDotClass(statusColor(status));
	}

	/** Per-type fallback colors when status is not meaningful. */
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

	function typeColor(artifactType: string): string {
		return TYPE_COLORS[artifactType] ?? "#6b7280";
	}

	/** Build the vis-network from the current graph state. */
	function buildNetwork(el: HTMLDivElement): void {
		if (network) {
			network.destroy();
			network = null;
		}

		const graphNodes = [...artifactGraphSDK.graph.values()];

		if (graphNodes.length === 0) return;

		stabilizing = true;

		const nodeDataset = new DataSet<Node>(
			graphNodes.map((node): Node => {
				const color = node.status
					? resolveNodeColor(node.status)
					: typeColor(node.artifact_type);
				return {
					id: node.id,
					label: node.id,
					title: `${node.title}\n${node.artifact_type}${node.status ? ` · ${node.status}` : ""}`,
					color: {
						background: color,
						border: color,
						highlight: { background: color, border: "#ffffff" },
						hover: { background: color, border: "#ffffff" },
					},
					font: { color: "#ffffff", size: 10, face: "monospace" },
					size: 12,
					borderWidth: 1,
					shape: "dot",
				};
			}),
		);

		// Deduplicate edges: use "source->target" as key to avoid multi-edges
		const edgeKeys = new SvelteSet<string>();
		const edgeList: Edge[] = [];

		for (const node of graphNodes) {
			for (const ref of node.references_out) {
				// Only add edges where the target also exists in the graph
				if (!artifactGraphSDK.graph.has(ref.target_id)) continue;
				const key = `${ref.source_id}->${ref.target_id}`;
				if (edgeKeys.has(key)) continue;
				edgeKeys.add(key);
				edgeList.push({
					from: ref.source_id,
					to: ref.target_id,
					arrows: "to",
					color: { color: "#4b5563", opacity: 0.5 },
				});
			}
		}

		const edgeDataset = new DataSet<Edge>(edgeList);

		const options: Options = {
			physics: {
				enabled: true,
				stabilization: {
					enabled: true,
					iterations: 150,
					updateInterval: 25,
					fit: true,
				},
				barnesHut: {
					gravitationalConstant: -4000,
					centralGravity: 0.3,
					springLength: 100,
					springConstant: 0.04,
					damping: 0.09,
				},
			},
			layout: {
				improvedLayout: graphNodes.length < 100,
			},
			interaction: {
				hover: true,
				tooltipDelay: 200,
				navigationButtons: true,
				keyboard: true,
				zoomView: true,
			},
			nodes: {
				shape: "dot",
				scaling: { min: 8, max: 20 },
			},
			edges: {
				smooth: { enabled: true, type: "dynamic", roundness: 0.5 },
				width: 1,
				arrows: { to: { enabled: true, scaleFactor: 0.5 } },
			},
		};

		network = new Network(el, { nodes: nodeDataset, edges: edgeDataset }, options);

		network.on("click", (params) => {
			if (params.nodes.length > 0) {
				const clickedId = String(params.nodes[0]);
				navigationStore.navigateToArtifact(clickedId);
			}
		});

		network.on("stabilizationProgress", (params) => {
			stabilizing = true;
			stabilizationProgress = Math.round((params.iterations / params.total) * 100);
		});

		network.on("stabilizationIterationsDone", () => {
			stabilizing = false;
			stabilizationProgress = 100;
			network?.setOptions({ physics: { enabled: false } });
			network?.fit({ animation: { duration: 400, easingFunction: "easeInOutQuad" } });
		});

		network.on("stabilized", () => {
			stabilizing = false;
		});
	}

	// Rebuild only when the container mounts or the graph data actually changes
	$effect(() => {
		const el = container;
		const currentSize = artifactGraphSDK.graph.size;

		if (!el) return;

		// Skip rebuild if graph size hasn't changed (cache the render)
		if (network && currentSize === lastGraphSize) return;

		lastGraphSize = currentSize;
		buildNetwork(el);
	});

	onDestroy(() => {
		if (network) {
			network.destroy();
			network = null;
		}
	});
</script>

<div class="relative flex h-full flex-col">
	<!-- Header -->
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

	<!-- Graph container -->
	{#if artifactGraphSDK.loading}
		<div class="flex flex-1 items-center justify-center text-sm text-muted-foreground">
			Loading graph…
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
				<div class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-background/80 backdrop-blur-sm">
					<p class="text-sm font-medium text-muted-foreground">Laying out {artifactGraphSDK.graph.size} nodes…</p>
					<div class="h-1.5 w-48 overflow-hidden rounded-full bg-muted">
						<div
							class="h-full rounded-full bg-primary transition-[width] duration-200"
							style="width: {stabilizationProgress}%"
						></div>
					</div>
					<p class="text-xs text-muted-foreground tabular-nums">{stabilizationProgress}%</p>
				</div>
			{/if}
		</div>
	{/if}
</div>
