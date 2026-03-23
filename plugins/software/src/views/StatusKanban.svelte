<script lang="ts">
	import type { ArtifactNode } from "@orqastudio/types";
	import CollapsibleColumn from "./CollapsibleColumn.svelte";
	import KanbanCard from "./KanbanCard.svelte";
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { EmptyState } from "@orqastudio/svelte-components/pure";

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

	// All-done view: whether the user has clicked "View board" to override the all-done state
	let showBoardOverride = $state(false);

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
			groupBy === "priority" ? (node.priority ?? "") : (node.status ?? "");

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

	// Count nodes that are NOT in a done column (status mode only)
	const nonDoneCount = $derived(
		nodes.filter((n) => {
			if (groupBy === "priority") return false; // priority mode has no "done" semantics
			const doneKeys = activeColumns
				.filter((c) => c.isDone)
				.map((c) => c.key.toLowerCase());
			return !doneKeys.includes((n.status ?? "").toLowerCase());
		}).length,
	);

	const doneNodes = $derived(totalNodes - nonDoneCount);

	// All-done: items exist, none are in non-done columns, status grouping, user hasn't overridden
	const isAllDone = $derived(
		totalNodes > 0 && nonDoneCount === 0 && groupBy === "status" && !showBoardOverride,
	);

	// Done column is collapsed only when there are non-done items present
	function doneColumnCollapsed(col: ColumnDef): boolean {
		if (!col.isDone) return false;
		return nonDoneCount > 0;
	}
</script>

<div class="flex h-full flex-col gap-3">
	<!-- Toolbar -->
	<div class="flex items-center justify-between">
		<span class="text-xs text-muted-foreground">
			{doneNodes}/{totalNodes} Done
		</span>
		<SelectMenu
			items={GROUP_OPTIONS}
			selected={groupBy}
			onSelect={(v) => {
				groupBy = v;
				showBoardOverride = false;
			}}
			triggerLabel={groupByLabel}
			triggerSize="sm"
		/>
	</div>

	<!-- All-done state -->
	{#if isAllDone}
		<div class="flex flex-1 items-center justify-center">
			<EmptyState
				icon="circle-check-big"
				title="All completed"
				description="Every item at this level is done."
				action={{
					label: "View board",
					onclick: () => {
						showBoardOverride = true;
					},
				}}
			/>
		</div>
	{:else}
		<!-- Kanban columns -->
		<div class="min-h-0 flex-1">
			<div class="flex h-full gap-3 pb-2">
				{#if totalNodes === 0}
					<div class="flex flex-1 items-center justify-center">
						<EmptyState
							icon="layers"
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
							doneCount={col.isDone && totalNodes > 0 ? colNodes.length : undefined}
							totalCount={col.isDone && totalNodes > 0 ? totalNodes : undefined}
							collapsed={doneColumnCollapsed(col)}
							isDone={col.isDone}
							onDrop={(e) => handleDrop(e, col.key)}
						>
							{#if colNodes.length === 0}
								<div
									class="rounded border border-dashed border-border p-3 text-center text-xs text-muted-foreground"
								>
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
		</div>
	{/if}
</div>
