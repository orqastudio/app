<script lang="ts">
	import { onDestroy } from "svelte";
	import { SvelteSet } from "svelte/reactivity";
	import { Network } from "vis-network";
	import type { Node, Edge, Options } from "vis-network";
	import { DataSet } from "vis-data";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";

	let container = $state<HTMLDivElement | undefined>(undefined);
	let network: Network | null = null;

	// Suppress ResizeObserver loop error — vis-network triggers resize during
	// layout which fires before the browser can deliver all notifications.
	// This is harmless and expected with any physics-based graph renderer.
	if (typeof window !== "undefined") {
		const origError = window.onerror;
		window.onerror = (msg, ...args) => {
			if (typeof msg === "string" && msg.includes("ResizeObserver")) return true;
			if (origError) return origError(msg, ...args) as boolean;
			return false;
		};
	}
	let stabilizing = $state(false);
	let stabilizationProgress = $state(0);

	/** Cached graph size — only rebuild when this changes. */
	let lastGraphSize = 0;

	/** Debounced resize handler to avoid ResizeObserver loop. */
	let resizeTimer: ReturnType<typeof setTimeout> | null = null;
	let resizeObserver: ResizeObserver | null = null;

	/** Cached node positions from previous stabilization. */
	let cachedPositions: Record<string, { x: number; y: number }> = {};

	function hexFromDotClass(dotClass: string): string {
		if (dotClass.includes("blue-500")) return "#3b82f6";
		if (dotClass.includes("emerald-500")) return "#10b981";
		if (dotClass.includes("amber-500")) return "#f59e0b";
		if (dotClass.includes("purple-500")) return "#a855f7";
		if (dotClass.includes("destructive") || dotClass.includes("red")) return "#ef4444";
		return "#6b7280";
	}

	function resolveNodeColor(status: string | null): string {
		if (!status) return "#6b7280";
		return hexFromDotClass(statusColor(status));
	}

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

	function buildNetwork(el: HTMLDivElement): void {
		if (network) {
			// Save positions before destroying
			try {
				const positions = network.getPositions();
				cachedPositions = positions;
			} catch {
				// Network may not be ready
			}
			network.destroy();
			network = null;
		}

		const graphNodes = [...artifactGraphSDK.graph.values()];
		if (graphNodes.length === 0) return;

		const hasCachedPositions = Object.keys(cachedPositions).length > 0;

		// Only show stabilizing overlay if we don't have cached positions
		if (!hasCachedPositions) {
			stabilizing = true;
			stabilizationProgress = 0;
		}

		const nodeDataset = new DataSet<Node>(
			graphNodes.map((node): Node => {
				const color = node.status
					? resolveNodeColor(node.status)
					: typeColor(node.artifact_type);
				const cached = cachedPositions[node.id];
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
					// Use cached position if available — skips physics
					...(cached ? { x: cached.x, y: cached.y, fixed: false } : {}),
				};
			}),
		);

		const edgeKeys = new SvelteSet<string>();
		const edgeList: Edge[] = [];

		for (const node of graphNodes) {
			for (const ref of node.references_out) {
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
			autoResize: false,
			physics: {
				enabled: !hasCachedPositions, // Disable physics if positions are cached
				stabilization: {
					enabled: !hasCachedPositions,
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
				improvedLayout: !hasCachedPositions && graphNodes.length < 100,
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
			stabilizationProgress = Math.round((params.iterations / params.total) * 100);
		});

		network.on("stabilizationIterationsDone", () => {
			stabilizing = false;
			stabilizationProgress = 100;
			network?.setOptions({ physics: { enabled: false } });
			network?.fit({ animation: { duration: 400, easingFunction: "easeInOutQuad" } });
			// Cache positions for next render
			try {
				if (network) cachedPositions = network.getPositions();
			} catch {
				// Ignore
			}
		});

		// If we used cached positions, fit immediately
		if (hasCachedPositions && network) {
			stabilizing = false;
			network.fit({ animation: { duration: 200, easingFunction: "easeInOutQuad" } });
		}

		// Manual debounced resize (autoResize disabled to prevent loop)
		if (resizeObserver) resizeObserver.disconnect();
		resizeObserver = new ResizeObserver(() => {
			if (resizeTimer) clearTimeout(resizeTimer);
			resizeTimer = setTimeout(() => {
				network?.redraw();
				network?.fit();
			}, 150);
		});
		resizeObserver.observe(el);
	}

	$effect(() => {
		const el = container;
		const currentSize = artifactGraphSDK.graph.size;

		if (!el) return;
		if (network && currentSize === lastGraphSize) return;

		lastGraphSize = currentSize;
		buildNetwork(el);
	});

	onDestroy(() => {
		if (resizeObserver) {
			resizeObserver.disconnect();
			resizeObserver = null;
		}
		if (resizeTimer) clearTimeout(resizeTimer);
		if (network) {
			try {
				cachedPositions = network.getPositions();
			} catch {
				// Ignore
			}
			network.destroy();
			network = null;
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
				<div class="absolute inset-0 flex flex-col items-center justify-center gap-4 bg-background/80 backdrop-blur-sm">
					<LoadingSpinner size="lg" />
					<p class="text-sm font-medium text-muted-foreground">
						Laying out {artifactGraphSDK.graph.size} nodes…
					</p>
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
