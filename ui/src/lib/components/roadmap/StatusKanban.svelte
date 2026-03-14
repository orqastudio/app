<script lang="ts">
	import type { ArtifactNode } from "$lib/types/artifact-graph";
	import CollapsibleColumn from "./CollapsibleColumn.svelte";
	import KanbanCard from "./KanbanCard.svelte";
	import SelectMenu from "$lib/components/shared/SelectMenu.svelte";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import * as ScrollArea from "$lib/components/ui/scroll-area";

	type ColumnDef = {
		key: string;
		label: string;
		isDone?: boolean;
	};

	let {
		nodes,
		columns,
		onCardClick,
		onFieldChange,
		getTaskCount,
	}: {
		nodes: ArtifactNode[];
		columns: ColumnDef[];
		onCardClick?: (node: ArtifactNode) => void;
		onFieldChange?: (node: ArtifactNode, newValue: string) => Promise<void>;
		getTaskCount?: (nodeId: string) => { done: number; total: number } | undefined;
	} = $props();

	// Grouping options
	const GROUP_OPTIONS = [
		{ value: "status", label: "Group by Status" },
		{ value: "priority", label: "Group by Priority" },
	];
	let groupBy = $state("status");
	const groupByLabel = $derived(
		GROUP_OPTIONS.find((o) => o.value === groupBy)?.label ?? "Group by Status",
	);

	// Drag state
	let dragNodeId = $state<string | null>(null);

	function nodesForColumn(colKey: string): ArtifactNode[] {
		if (groupBy === "priority") {
			// Remap priority columns to P1/P2/P3/none
			return nodes.filter((n) => (n.priority ?? "none") === colKey);
		}
		// Default: status-based column
		return nodes.filter(
			(n) => (n.status ?? "").toLowerCase() === colKey.toLowerCase(),
		);
	}

	function handleDragStart(e: DragEvent, node: ArtifactNode) {
		dragNodeId = node.id;
		e.dataTransfer?.setData("text/plain", node.id);
	}

	function handleDrop(e: DragEvent, colKey: string) {
		e.preventDefault();
		const nodeId = e.dataTransfer?.getData("text/plain") ?? dragNodeId;
		if (!nodeId) return;
		const node = nodes.find((n) => n.id === nodeId);
		if (!node) return;

		const currentValue =
			groupBy === "priority"
				? (node.priority ?? "")
				: (node.status ?? "");

		if (currentValue === colKey) return;

		onFieldChange?.(node, colKey);
		dragNodeId = null;
	}

	// Priority columns (used when groupBy === "priority")
	const PRIORITY_COLUMNS: ColumnDef[] = [
		{ key: "P1", label: "P1 — Critical" },
		{ key: "P2", label: "P2 — High" },
		{ key: "P3", label: "P3 — Normal" },
		{ key: "none", label: "Unranked", isDone: true },
	];

	const activeColumns = $derived(groupBy === "priority" ? PRIORITY_COLUMNS : columns);

	const totalNodes = $derived(nodes.length);
</script>

<div class="flex h-full flex-col gap-3">
	<!-- Toolbar -->
	<div class="flex items-center justify-between">
		<span class="text-xs text-muted-foreground">
			{totalNodes} item{totalNodes === 1 ? "" : "s"}
		</span>
		<SelectMenu
			items={GROUP_OPTIONS}
			selected={groupBy}
			onSelect={(v) => { groupBy = v; }}
			triggerLabel={groupByLabel}
			triggerSize="sm"
		/>
	</div>

	<!-- Kanban columns -->
	<ScrollArea.Root class="min-h-0 flex-1" orientation="horizontal">
	<div class="flex h-full min-w-max gap-3 pb-2">
		{#if totalNodes === 0}
			<div class="flex flex-1 items-center justify-center">
				<EmptyState
					icon={LayersIcon}
					title="No items"
					description="Nothing to show here yet."
				/>
			</div>
		{:else}
			{#each activeColumns as col (col.key)}
				{@const colNodes = nodesForColumn(col.key)}
				<CollapsibleColumn
					title={col.label}
					count={colNodes.length}
					collapsed={col.isDone === true && colNodes.length > 0}
					isDone={col.isDone}
					onDrop={(e) => handleDrop(e, col.key)}
				>
					{#if colNodes.length === 0}
						<div class="rounded border border-dashed border-border p-3 text-center text-xs text-muted-foreground">
							No items
						</div>
					{:else}
						{#each colNodes as node (node.id)}
							<KanbanCard
								{node}
								taskCount={getTaskCount?.(node.id)}
								onClick={onCardClick ? () => onCardClick(node) : undefined}
								onDragStart={onFieldChange
									? (e) => handleDragStart(e, node)
									: undefined}
							/>
						{/each}
					{/if}
				</CollapsibleColumn>
			{/each}
		{/if}
	</div>
	</ScrollArea.Root>
</div>
