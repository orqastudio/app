<!-- ProcessGraphView — renders the dependency graph as a visual node layout
     with SVG edges. Each node is a card showing build status, error/warning
     count, and process status. Click-through navigates to the Stream tab
     pre-filtered for that node's events.

     Layout: topological tiers from left to right. Nodes with no dependencies
     are in tier 0 (leftmost). Each tier is a vertical column. -->
<script lang="ts">
	import { onMount } from "svelte";
	import {
		Stack,
		HStack,
		ScrollArea,
		Text,
		Caption,
		Badge,
		Icon,
		Panel,
	} from "@orqastudio/svelte-components/pure";
	import {
		topology,
		topologyLoaded,
		nodeCounts,
		categoryFilterForNode,
		initTopology,
		type GraphNode,
	} from "../../stores/graph-topology.svelte.js";
	import { filters } from "../../stores/log-store.svelte.js";
	import { navigation } from "../../stores/devtools-navigation.svelte.js";
	import { SvelteMap, SvelteSet } from "svelte/reactivity";

	onMount(() => {
		initTopology();
	});

	// Compute topological tiers for layout.
	const tiers = $derived.by(() => {
		const nodes = topology.nodes;
		if (nodes.length === 0) return [];

		// Build depth map: longest path from a root to each node.
		const depthMap = new SvelteMap<string, number>();
		const nodeMap = new SvelteMap<string, GraphNode>();
		for (const node of nodes) {
			nodeMap.set(node.id, node);
		}

		/**
		 * Recursively compute the depth of a node.
		 * @param id - Node ID to compute depth for.
		 * @returns The depth (longest dependency chain length).
		 */
		function getDepth(id: string): number {
			if (depthMap.has(id)) return depthMap.get(id)!;
			const node = nodeMap.get(id);
			if (!node || node.dependsOn.length === 0) {
				depthMap.set(id, 0);
				return 0;
			}
			const maxParent = Math.max(...node.dependsOn.map((d) => getDepth(d)));
			const depth = maxParent + 1;
			depthMap.set(id, depth);
			return depth;
		}

		for (const node of nodes) getDepth(node.id);

		// Group by tier.
		const maxDepth = Math.max(...depthMap.values(), 0);
		const tierArr: GraphNode[][] = Array.from({ length: maxDepth + 1 }, () => []);
		for (const node of nodes) {
			const depth = depthMap.get(node.id) ?? 0;
			tierArr[depth].push(node);
		}

		return tierArr;
	});

	const counts = $derived(nodeCounts());

	// Node kind display config.
	const KIND_LABEL: Record<string, string> = {
		"ts-library": "Library",
		"svelte-library": "Library",
		"rust-workspace": "Workspace",
		"tauri-app": "App",
		service: "Service",
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
{:else if tiers.length === 0}
	<Panel padding="normal">
		<Text variant="body-muted">No process nodes detected.</Text>
	</Panel>
{:else}
	<ScrollArea full>
		<Panel padding="normal">
			<HStack gap={6} align="start">
				{#each tiers as tier, tierIndex (tierIndex)}
					<Stack gap={2} minWidth={0}>
						<Caption tone="muted">Tier {tierIndex}</Caption>
						{#each tier as node (node.id)}
							{@const c = counts[node.id] ?? { errors: 0, warnings: 0 }}
							{@const isService = node.kind === "service" || node.kind === "tauri-app"}
							<button
								type="button"
								class="border-border bg-card hover:bg-accent/30 flex w-44 flex-col gap-1 rounded-lg border p-2 text-left transition-colors"
								onclick={() => handleNodeClick(node.id)}
							>
								<HStack gap={1} justify="between">
									<Text variant="label" truncate>{node.name}</Text>
									{#if isService}
										<Icon
											name={node.status === "running" ? "circle-check" : "circle-x"}
											size="sm"
										/>
									{/if}
								</HStack>

								<HStack gap={1}>
									<Badge variant="outline" size="xs">{KIND_LABEL[node.kind] ?? node.kind}</Badge>
									<Badge variant={STATUS_VARIANT[node.status] ?? "outline"} size="xs">
										{node.status}
									</Badge>
								</HStack>

								{#if c.errors > 0 || c.warnings > 0}
									<HStack gap={2}>
										{#if c.errors > 0}
											<Caption tone="destructive">{c.errors} errors</Caption>
										{/if}
										{#if c.warnings > 0}
											<Caption tone="warning">{c.warnings} warnings</Caption>
										{/if}
									</HStack>
								{/if}
							</button>
						{/each}
					</Stack>
				{/each}
			</HStack>
		</Panel>
	</ScrollArea>
{/if}
