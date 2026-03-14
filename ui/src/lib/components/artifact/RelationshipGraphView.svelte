<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
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

	/** Layout constants */
	const WIDTH = 600;
	const HEIGHT = 400;
	const CX = WIDTH / 2;
	const CY = HEIGHT / 2;
	const INNER_RADIUS = 120;
	const OUTER_RADIUS = 170;
	const NODE_RADIUS = 6;
	const CENTER_RADIUS = 10;

	interface GraphNode {
		id: string;
		label: string;
		title: string;
		x: number;
		y: number;
		fillColor: string;
		direction: "in" | "out";
		edgeType: string;
	}

	interface GraphGroup {
		label: string;
		direction: "in" | "out";
		midAngle: number;
		labelX: number;
		labelY: number;
	}

	/** Humanize a relationship type. */
	function humanizeLabel(value: string): string {
		return value
			.replace(/-/g, " ")
			.replace(/_/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/** Map Tailwind bg class to an SVG fill color. */
	function svgFillFromClass(dotClass: string): string {
		if (dotClass.includes("blue-500")) return "#3b82f6";
		if (dotClass.includes("emerald-500")) return "#10b981";
		if (dotClass.includes("amber-500")) return "#f59e0b";
		if (dotClass.includes("purple-500")) return "#a855f7";
		if (dotClass.includes("destructive")) return "#ef4444";
		return "#6b7280";
	}

	/** Resolve SVG fill color for an artifact ID based on its status. */
	function resolveNodeColor(targetId: string): string {
		const node = artifactGraphSDK.resolve(targetId);
		if (!node?.status) return "#6b7280";
		return svgFillFromClass(statusColor(node.status));
	}

	/** Resolve display title for an artifact ID. */
	function resolveTitle(targetId: string): string {
		const node = artifactGraphSDK.resolve(targetId);
		return node?.title ?? targetId;
	}

	/** Collect unique related IDs grouped by (direction, edgeType). */
	function buildGroups(): {
		key: string;
		direction: "in" | "out";
		edgeType: string;
		ids: string[];
	}[] {
		const map = new SvelteMap<string, { direction: "in" | "out"; edgeType: string; ids: string[] }>();

		for (const ref of incomingRefs) {
			const edgeType = ref.relationship_type ?? ref.field;
			const key = `in:${edgeType}`;
			const group = map.get(key);
			if (group) {
				if (!group.ids.includes(ref.source_id)) group.ids.push(ref.source_id);
			} else {
				map.set(key, { direction: "in", edgeType, ids: [ref.source_id] });
			}
		}

		for (const ref of outgoingRefs) {
			const edgeType = ref.relationship_type ?? ref.field;
			const key = `out:${edgeType}`;
			const group = map.get(key);
			if (group) {
				if (!group.ids.includes(ref.target_id)) group.ids.push(ref.target_id);
			} else {
				map.set(key, { direction: "out", edgeType, ids: [ref.target_id] });
			}
		}

		return [...map.entries()].map(([key, val]) => ({ key, ...val }));
	}

	/** Compute node positions in a radial layout. */
	const layout = $derived.by(() => {
		const groups = buildGroups();
		const totalGroups = groups.length;
		if (totalGroups === 0) return { nodes: [] as GraphNode[], groups: [] as GraphGroup[] };

		const sectorAngle = (2 * Math.PI) / totalGroups;
		const nodes: GraphNode[] = [];
		const groupLabels: GraphGroup[] = [];

		groups.forEach((group, gi) => {
			const startAngle = gi * sectorAngle - Math.PI / 2;
			const midAngle = startAngle + sectorAngle / 2;

			// Group label position
			const labelR = OUTER_RADIUS + 22;
			groupLabels.push({
				label: humanizeLabel(group.edgeType),
				direction: group.direction,
				midAngle,
				labelX: CX + labelR * Math.cos(midAngle),
				labelY: CY + labelR * Math.sin(midAngle),
			});

			// Place nodes along the sector arc
			const count = group.ids.length;
			for (let ni = 0; ni < count; ni++) {
				const t = count === 1 ? 0.5 : ni / (count - 1);
				const angle = startAngle + sectorAngle * 0.15 + t * sectorAngle * 0.7;
				const r = count === 1 ? INNER_RADIUS : INNER_RADIUS + (ni % 2 === 0 ? 0 : 20);

				const targetId = group.ids[ni];

				nodes.push({
					id: targetId,
					label: targetId,
					title: resolveTitle(targetId),
					x: CX + r * Math.cos(angle),
					y: CY + r * Math.sin(angle),
					fillColor: resolveNodeColor(targetId),
					direction: group.direction,
					edgeType: group.edgeType,
				});
			}
		});

		return { nodes, groups: groupLabels };
	});

	/** Center node fill color. */
	const centerFill = $derived(resolveNodeColor(artifactId));

	function handleNodeClick(id: string): void {
		navigationStore.navigateToArtifact(id);
	}

	/** Compute text-anchor for a label based on its angle. */
	function textAnchor(angle: number): string {
		const deg = ((angle * 180) / Math.PI + 360) % 360;
		if (deg > 100 && deg < 260) return "end";
		if (deg >= 80 && deg <= 100) return "middle";
		if (deg >= 260 && deg <= 280) return "middle";
		return "start";
	}
</script>

<svg
	viewBox="0 0 {WIDTH} {HEIGHT}"
	class="h-full w-full"
	role="img"
	aria-label="Relationship graph for {artifactId}"
>
	<!-- Edges from center to each node -->
	{#each layout.nodes as node (node.id + node.direction + node.edgeType)}
		<line
			x1={CX}
			y1={CY}
			x2={node.x}
			y2={node.y}
			stroke={node.direction === "out" ? "#3b82f650" : "#a855f750"}
			stroke-width="1"
			stroke-dasharray={node.direction === "in" ? "4 3" : "none"}
		/>
	{/each}

	<!-- Group type labels -->
	{#each layout.groups as group (group.label + group.direction)}
		<text
			x={group.labelX}
			y={group.labelY}
			text-anchor={textAnchor(group.midAngle)}
			dominant-baseline="central"
			class="fill-muted-foreground"
			font-size="9"
			font-weight="500"
			style="text-transform: uppercase"
		>
			{group.direction === "in" ? "\u2190 " : ""}{group.label}{group.direction === "out" ? " \u2192" : ""}
		</text>
	{/each}

	<!-- Related artifact nodes -->
	{#each layout.nodes as node (node.id + node.direction + node.edgeType)}
		<g
			class="cursor-pointer"
			role="button"
			tabindex="0"
			onclick={() => handleNodeClick(node.id)}
			onkeydown={(e) => {
				if (e.key === "Enter" || e.key === " ") handleNodeClick(node.id);
			}}
		>
			<title>{node.title} ({node.id})</title>
			<circle
				cx={node.x}
				cy={node.y}
				r={NODE_RADIUS}
				fill={node.fillColor}
				stroke="var(--border)"
				stroke-width="0.5"
				opacity="0.9"
			/>
			<!-- Invisible larger hit area for easier clicking -->
			<circle
				cx={node.x}
				cy={node.y}
				r={NODE_RADIUS + 4}
				fill="transparent"
			/>
			<text
				x={node.x}
				y={node.y + NODE_RADIUS + 11}
				text-anchor="middle"
				class="fill-foreground"
				font-size="9"
				font-family="monospace"
			>
				{node.label}
			</text>
		</g>
	{/each}

	<!-- Center node (focused artifact) -->
	<circle
		cx={CX}
		cy={CY}
		r={CENTER_RADIUS}
		fill={centerFill}
		stroke="var(--foreground)"
		stroke-width="1.5"
		opacity="0.95"
	/>
	<text
		x={CX}
		y={CY + CENTER_RADIUS + 14}
		text-anchor="middle"
		class="fill-foreground"
		font-size="11"
		font-family="monospace"
		font-weight="600"
	>
		{artifactId}
	</text>
</svg>
