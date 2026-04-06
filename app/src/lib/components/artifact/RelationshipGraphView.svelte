<script lang="ts">
	import { onDestroy } from "svelte";
	import { SvelteSet } from "svelte/reactivity";
	import cytoscape from "cytoscape";
	// @ts-expect-error — no type declarations for cytoscape-cose-bilkent
	import coseBilkent from "cytoscape-cose-bilkent";
	import { getStores } from "@orqastudio/sdk";
	import { ARTIFACT_TYPE_COLORS } from "@orqastudio/graph-visualiser";

	const { artifactGraphSDK, navigationStore } = getStores();
	import type { ArtifactRef } from "@orqastudio/types";
	import { Box } from "@orqastudio/svelte-components/pure";

	// Register layout extension once (safe to call multiple times — cytoscape deduplicates)
	cytoscape.use(coseBilkent);

	let {
		artifactId,
		incomingRefs,
		outgoingRefs,
	}: {
		artifactId: string;
		incomingRefs: readonly ArtifactRef[];
		outgoingRefs: readonly ArtifactRef[];
	} = $props();

	/** Container div bound by the template via Box ref. */
	let container = $state<HTMLDivElement | undefined>(undefined);

	/** The cytoscape instance, cleaned up on destroy. */
	let cy: cytoscape.Core | null = null;

	/**
	 * Resolve a node color from its artifact type.
	 * @param id - The artifact ID to look up.
	 * @returns A hex color string for the artifact type, or gray as fallback.
	 */
	function resolveNodeColor(id: string): string {
		const node = artifactGraphSDK.resolve(id);
		if (!node) return "#6b7280";
		return ARTIFACT_TYPE_COLORS[node.artifact_type] ?? "#6b7280";
	}

	/**
	 * Resolve the display title for an artifact.
	 * @param id - The artifact ID to look up.
	 * @returns The artifact title, or the ID itself if the artifact is not found.
	 */
	function resolveTitle(id: string): string {
		const node = artifactGraphSDK.resolve(id);
		return node?.title ?? id;
	}

	/**
	 * Humanize a relationship type label.
	 * @param value - The raw relationship type string to humanize.
	 * @returns A title-cased human-readable label.
	 */
	function humanizeLabel(value: string): string {
		return value
			.replace(/-/g, " ")
			.replace(/_/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/**
	 * Build and (re)initialize the cytoscape instance.
	 * @param el - The container div to mount cytoscape into.
	 */
	function buildGraph(el: HTMLDivElement): void {
		if (cy) {
			cy.destroy();
			cy = null;
		}

		// Collect unique node IDs from incoming and outgoing refs
		const nodeIds = new SvelteSet<string>();
		nodeIds.add(artifactId);
		for (const ref of incomingRefs) nodeIds.add(ref.source_id);
		for (const ref of outgoingRefs) nodeIds.add(ref.target_id);

		const elements: cytoscape.ElementDefinition[] = [];

		// Nodes
		for (const id of nodeIds) {
			const isCenter = id === artifactId;
			const color = resolveNodeColor(id);
			elements.push({
				group: "nodes",
				data: {
					id,
					label: id,
					color,
					isCenter: isCenter ? 1 : 0,
					tooltip: resolveTitle(id),
				},
			});
		}

		// Outgoing edges (solid, blue)
		for (const ref of outgoingRefs) {
			const edgeType = ref.relationship_type ?? ref.field;
			elements.push({
				group: "edges",
				data: {
					id: `out-${artifactId}-${ref.target_id}-${edgeType}`,
					source: artifactId,
					target: ref.target_id,
					label: humanizeLabel(edgeType),
					edgeDir: "out",
				},
			});
		}

		// Incoming edges (dashed, purple)
		for (const ref of incomingRefs) {
			const edgeType = ref.relationship_type ?? ref.field;
			elements.push({
				group: "edges",
				data: {
					id: `in-${ref.source_id}-${artifactId}-${edgeType}`,
					source: ref.source_id,
					target: artifactId,
					label: humanizeLabel(edgeType),
					edgeDir: "in",
				},
			});
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
						width: 20,
						height: 20,
						"text-outline-width": 2,
						"text-outline-color": "data(color)",
					},
				},
				{
					// Center node is larger with a white border ring
					selector: "node[isCenter = 1]",
					style: {
						width: 32,
						height: 32,
						"font-size": "12px",
						"border-width": 3,
						"border-color": "#ffffff",
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
						width: 1.5,
						"line-color": "#4b5563",
						"target-arrow-color": "#4b5563",
						"target-arrow-shape": "triangle",
						"curve-style": "bezier",
						opacity: 0.7,
						label: "data(label)",
						"font-size": "9px",
						color: "#9ca3af",
						"text-rotation": "autorotate",
						"text-margin-y": -8,
					},
				},
				{
					// Outgoing edges: solid, blue
					selector: "edge[edgeDir = 'out']",
					style: {
						"line-color": "#3b82f6",
						"target-arrow-color": "#3b82f6",
					},
				},
				{
					// Incoming edges: dashed, purple
					selector: "edge[edgeDir = 'in']",
					style: {
						"line-style": "dashed",
						"line-color": "#a855f7",
						"target-arrow-color": "#a855f7",
					},
				},
			],
			layout: {
				name: "cose",
				animate: false,
				randomize: false,
				nodeRepulsion: () => 3000,
				idealEdgeLength: () => 120,
				edgeElasticity: () => 100,
				gravity: 80,
				numIter: 1000,
				fit: true,
				padding: 30,
			},
			minZoom: 0.3,
			maxZoom: 3,
			wheelSensitivity: 0.3,
		});

		// Click non-center nodes to navigate
		cy.on("tap", "node", (evt) => {
			const nodeId = evt.target.id() as string;
			if (nodeId !== artifactId) {
				navigationStore.navigateToArtifact(nodeId);
			}
		});
	}

	// Re-initialize whenever container or data changes
	$effect(() => {
		const el = container;
		// Access reactive dependencies so the effect re-runs on data changes
		void artifactId;
		void incomingRefs;
		void outgoingRefs;

		if (!el) return;

		buildGraph(el);
	});

	onDestroy(() => {
		if (cy) {
			cy.destroy();
			cy = null;
		}
	});
</script>

<Box
	bind:ref={container}
	height="full"
	width="full"
	role="img"
	aria-label="Relationship graph for {artifactId}"
/>
