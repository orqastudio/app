<!-- ProcessGraphView — renders the dependency graph as a visual node layout
     with SVG edges. Each node is a card showing build status, error/warning
     count, and process status. Click-through navigates to the Stream tab
     pre-filtered for that node's events.

     Layout: topological tiers from left to right. Nodes with no dependencies
     are in tier 0 (leftmost). Each tier is a vertical column. SVG edges
     connect nodes to their dependencies. -->
<script lang="ts">
	import { HStack, ScrollArea, Text, Badge, Icon, Panel } from "@orqastudio/svelte-components/pure";
	import {
		topology,
		topologyLoaded,
		nodeCounts,
		categoryFilterForNode,
		type GraphNode,
	} from "../../stores/graph-topology.svelte.js";
	import { filters } from "../../stores/log-store.svelte.js";
	import { navigation } from "../../stores/devtools-navigation.svelte.js";
	import { SvelteMap, SvelteSet } from "svelte/reactivity";

	// Layout constants.
	const CARD_W = 160;
	const CARD_H = 64;
	const TIER_GAP = 80;
	const NODE_GAP = 16;
	const PAD = 24;

	/** Positioned node for rendering. */
	interface PositionedNode {
		node: GraphNode;
		x: number;
		y: number;
		tier: number;
	}

	/** Edge between two positioned nodes. */
	interface Edge {
		from: PositionedNode;
		to: PositionedNode;
	}

	// Compute positioned nodes and edges.
	const layout = $derived.by(() => {
		const nodes = topology.nodes;
		if (nodes.length === 0) return { positioned: [], edges: [], width: 0, height: 0 };

		const nodeMap = new SvelteMap<string, GraphNode>();
		const depthMap = new SvelteMap<string, number>();
		for (const node of nodes) nodeMap.set(node.id, node);

		/**
		 * Compute depth of a node (longest dependency chain).
		 * @param id - Node ID.
		 * @returns Depth value.
		 */
		function getDepth(id: string): number {
			if (depthMap.has(id)) return depthMap.get(id)!;
			const node = nodeMap.get(id);
			if (!node || node.dependsOn.length === 0) {
				depthMap.set(id, 0);
				return 0;
			}
			const d = Math.max(...node.dependsOn.map((dep) => getDepth(dep))) + 1;
			depthMap.set(id, d);
			return d;
		}

		for (const node of nodes) getDepth(node.id);

		// Group by tier.
		const maxDepth = Math.max(...depthMap.values(), 0);
		const tiers: GraphNode[][] = Array.from({ length: maxDepth + 1 }, () => []);
		for (const node of nodes) tiers[depthMap.get(node.id) ?? 0].push(node);

		// Position nodes.
		const posMap = new SvelteMap<string, PositionedNode>();
		const positioned: PositionedNode[] = [];
		let maxY = 0;

		for (let t = 0; t <= maxDepth; t++) {
			const tier = tiers[t];
			for (let i = 0; i < tier.length; i++) {
				const x = PAD + t * (CARD_W + TIER_GAP);
				const y = PAD + i * (CARD_H + NODE_GAP);
				const pn: PositionedNode = { node: tier[i], x, y, tier: t };
				positioned.push(pn);
				posMap.set(tier[i].id, pn);
				if (y + CARD_H > maxY) maxY = y + CARD_H;
			}
		}

		// Build edges.
		const edges: Edge[] = [];
		for (const pn of positioned) {
			for (const depId of pn.node.dependsOn) {
				const dep = posMap.get(depId);
				if (dep) edges.push({ from: dep, to: pn });
			}
		}

		const width = PAD * 2 + (maxDepth + 1) * CARD_W + maxDepth * TIER_GAP;
		const height = maxY + PAD;

		return { positioned, edges, width, height };
	});

	const counts = $derived(nodeCounts());

	/**
	 * Shorten a package name for display by stripping the scope prefix.
	 * @param node - The graph node to get a short name for.
	 * @returns The shortened display name.
	 */
	function shortName(node: GraphNode): string {
		return node.name.replace("@orqastudio/", "");
	}

	const KIND_LABEL: Record<string, string> = {
		"ts-library": "Lib",
		"svelte-library": "Lib",
		"rust-workspace": "Rust",
		"tauri-app": "App",
		service: "Svc",
		plugin: "Plugin",
	};

	const STATUS_VARIANT: Record<
		string,
		"default" | "secondary" | "destructive" | "warning" | "outline"
	> = {
		running: "default",
		built: "secondary",
		watching: "secondary",
		building: "outline",
		rebuilding: "outline",
		starting: "outline",
		pending: "outline",
		stopped: "warning",
		crashed: "destructive",
		"build-failed": "destructive",
	};

	const STATUS_COLOR: Record<string, string> = {
		running: "text-emerald-400",
		built: "text-muted-foreground",
		watching: "text-muted-foreground",
		crashed: "text-destructive",
		"build-failed": "text-destructive",
		stopped: "text-warning",
	};

	/**
	 * Navigate to Stream tab with the category filter set for this node.
	 * @param nodeId - The graph node ID to filter by.
	 */
	function handleNodeClick(nodeId: string): void {
		const cat = categoryFilterForNode(nodeId);
		filters.categories = new SvelteSet([cat]);
		navigation.activeTab = "stream";
	}
</script>

{#if !topologyLoaded.value}
	<Panel padding="normal">
		<Text variant="body-muted">Waiting for process graph topology...</Text>
	</Panel>
{:else if layout.positioned.length === 0}
	<Panel padding="normal">
		<Text variant="body-muted">No process nodes detected.</Text>
	</Panel>
{:else}
	<ScrollArea full>
		<div class="relative" style="width: {layout.width}px; min-height: {layout.height}px;">
			<!-- SVG edges layer -->
			<svg class="pointer-events-none absolute inset-0" width={layout.width} height={layout.height}>
				{#each layout.edges as edge (edge.from.node.id + "->" + edge.to.node.id)}
					{@const x1 = edge.from.x + CARD_W}
					{@const y1 = edge.from.y + CARD_H / 2}
					{@const x2 = edge.to.x}
					{@const y2 = edge.to.y + CARD_H / 2}
					{@const mx = (x1 + x2) / 2}
					<path
						d="M {x1} {y1} C {mx} {y1}, {mx} {y2}, {x2} {y2}"
						fill="none"
						stroke="currentColor"
						stroke-width="1"
						class="text-border opacity-40"
					/>
				{/each}
			</svg>

			<!-- Node cards layer -->
			{#each layout.positioned as pn (pn.node.id)}
				{@const c = counts[pn.node.id] ?? { errors: 0, warnings: 0 }}
				{@const isService = pn.node.kind === "service" || pn.node.kind === "tauri-app"}
				<button
					type="button"
					class="border-border bg-card hover:bg-accent/30 absolute flex flex-col gap-0.5 rounded-lg border p-2 text-left transition-colors"
					style="left: {pn.x}px; top: {pn.y}px; width: {CARD_W}px; height: {CARD_H}px;"
					onclick={() => handleNodeClick(pn.node.id)}
				>
					<HStack gap={1} justify="between">
						<Text variant="label" truncate>{shortName(pn.node)}</Text>
						{#if isService}
							<span class={STATUS_COLOR[pn.node.status] ?? "text-muted-foreground"}>
								<Icon name={pn.node.status === "running" ? "circle-check" : "circle-x"} size="sm" />
							</span>
						{/if}
					</HStack>

					<HStack gap={1}>
						<Badge variant="outline" size="xs">{KIND_LABEL[pn.node.kind] ?? pn.node.kind}</Badge>
						<Badge variant={STATUS_VARIANT[pn.node.status] ?? "outline"} size="xs">
							{pn.node.status}
						</Badge>
						{#if c.errors > 0}
							<Badge variant="destructive" size="xs">{c.errors}</Badge>
						{/if}
						{#if c.warnings > 0}
							<Badge variant="warning" size="xs">{c.warnings}</Badge>
						{/if}
					</HStack>
				</button>
			{/each}
		</div>
	</ScrollArea>
{/if}
