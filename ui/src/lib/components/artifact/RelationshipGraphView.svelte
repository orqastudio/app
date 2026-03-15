<script lang="ts">
	import { onDestroy } from "svelte";
	import { Network } from "vis-network";
	import type { Node, Edge, Options } from "vis-network";
	import { DataSet } from "vis-data";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";
	import type { ArtifactRef } from "$lib/types/artifact-graph";

	let {
		artifactId,
		incomingRefs,
		outgoingRefs,
	}: {
		artifactId: string;
		incomingRefs: ArtifactRef[];
		outgoingRefs: ArtifactRef[];
	} = $props();

	/** Container div bound by the template. */
	let container = $state<HTMLDivElement | undefined>(undefined);

	/** The vis-network instance, cleaned up on destroy. */
	let network: Network | null = null;

	/** Map Tailwind dot class to a hex color for vis-network. */
	function hexFromDotClass(dotClass: string): string {
		if (dotClass.includes("blue-500")) return "#3b82f6";
		if (dotClass.includes("emerald-500")) return "#10b981";
		if (dotClass.includes("amber-500")) return "#f59e0b";
		if (dotClass.includes("purple-500")) return "#a855f7";
		if (dotClass.includes("destructive") || dotClass.includes("red")) return "#ef4444";
		return "#6b7280";
	}

	/** Resolve a node color from its status. */
	function resolveNodeColor(id: string): string {
		const node = artifactGraphSDK.resolve(id);
		if (!node?.status) return "#6b7280";
		return hexFromDotClass(statusColor(node.status));
	}

	/** Resolve the display title for an artifact. */
	function resolveTitle(id: string): string {
		const node = artifactGraphSDK.resolve(id);
		return node?.title ?? id;
	}

	/** Humanize a relationship type label. */
	function humanizeLabel(value: string): string {
		return value
			.replace(/-/g, " ")
			.replace(/_/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/** Build and (re)initialize the vis-network instance. */
	function buildNetwork(el: HTMLDivElement): void {
		// Tear down any existing instance before rebuilding
		if (network) {
			network.destroy();
			network = null;
		}

		// Collect unique node IDs from incoming and outgoing refs
		const nodeIds = new Set<string>();
		nodeIds.add(artifactId);
		for (const ref of incomingRefs) nodeIds.add(ref.source_id);
		for (const ref of outgoingRefs) nodeIds.add(ref.target_id);

		const nodes = new DataSet<Node>(
			[...nodeIds].map((id): Node => {
				const isCenter = id === artifactId;
				const color = resolveNodeColor(id);
				return {
					id,
					label: id,
					title: resolveTitle(id),
					color: {
						background: color,
						border: isCenter ? "#ffffff" : color,
						highlight: {
							background: color,
							border: "#ffffff",
						},
					},
					font: {
						color: "#ffffff",
						size: isCenter ? 14 : 11,
						face: "monospace",
					},
					size: isCenter ? 22 : 14,
					borderWidth: isCenter ? 3 : 1,
					shape: "dot",
				};
			}),
		);

		const edgeList: Edge[] = [];

		for (const ref of outgoingRefs) {
			const edgeType = ref.relationship_type ?? ref.field;
			edgeList.push({
				from: artifactId,
				to: ref.target_id,
				label: humanizeLabel(edgeType),
				dashes: false,
				color: { color: "#3b82f6", opacity: 0.7 },
				arrows: "to",
			});
		}

		for (const ref of incomingRefs) {
			const edgeType = ref.relationship_type ?? ref.field;
			edgeList.push({
				from: ref.source_id,
				to: artifactId,
				label: humanizeLabel(edgeType),
				dashes: true,
				color: { color: "#a855f7", opacity: 0.7 },
				arrows: "to",
			});
		}

		const edges = new DataSet<Edge>(edgeList);

		const options: Options = {
			physics: {
				enabled: true,
				stabilization: { iterations: 150, fit: true },
				barnesHut: {
					gravitationalConstant: -3000,
					springLength: 120,
					damping: 0.4,
				},
			},
			layout: {
				improvedLayout: true,
			},
			interaction: {
				hover: true,
				tooltipDelay: 200,
				navigationButtons: false,
				keyboard: false,
			},
			nodes: {
				shape: "dot",
				scaling: { min: 10, max: 28 },
			},
			edges: {
				font: {
					size: 9,
					color: "#9ca3af",
					align: "middle",
					strokeWidth: 0,
				},
				smooth: { enabled: true, type: "dynamic", roundness: 0.5 },
				width: 1.5,
			},
		};

		network = new Network(el, { nodes, edges }, options);

		network.on("click", (params) => {
			if (params.nodes.length > 0) {
				const clickedId = String(params.nodes[0]);
				if (clickedId !== artifactId) {
					navigationStore.navigateToArtifact(clickedId);
				}
			}
		});

		network.on("stabilizationIterationsDone", () => {
			network?.setOptions({ physics: { enabled: false } });
		});
	}

	// Re-initialize the network whenever container or data changes
	$effect(() => {
		const el = container;
		// Access reactive dependencies so the effect re-runs on data changes
		const _id = artifactId;
		const _in = incomingRefs;
		const _out = outgoingRefs;

		if (!el) return;

		buildNetwork(el);
	});

	onDestroy(() => {
		if (network) {
			network.destroy();
			network = null;
		}
	});
</script>

<div
	bind:this={container}
	class="h-full w-full"
	role="img"
	aria-label="Relationship graph for {artifactId}"
></div>
