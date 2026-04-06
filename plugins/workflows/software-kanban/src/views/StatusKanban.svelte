<!-- StatusKanban: displays artifact nodes as a kanban board grouped by status or priority.
     Supports drag-and-drop to change status/priority and an all-done celebration state. -->
<script lang="ts">
	import type { ArtifactNode } from "@orqastudio/types";
	import { assertNever } from "@orqastudio/types";
	import CollapsibleColumn from "./CollapsibleColumn.svelte";
	import KanbanCard from "./KanbanCard.svelte";
	import {
		SelectMenu,
		EmptyState,
		Caption,
		HStack,
		Stack,
		Box,
		Center,
	} from "@orqastudio/svelte-components/pure";

	type ColumnDef = {
		readonly key: string;
		readonly label: string;
		readonly isDone?: boolean;
	};

	/** Discriminated union of valid grouping modes for the status kanban. */
	type GroupBy = "status" | "priority";

	let {
		nodes,
		columns,
		onCardClick,
		onFieldChange,
		getTaskCount,
	}: {
		readonly nodes: readonly ArtifactNode[];
		readonly columns: readonly ColumnDef[];
		readonly onCardClick?: (node: ArtifactNode) => void;
		readonly onFieldChange?: (node: ArtifactNode, newValue: string) => Promise<void>;
		readonly getTaskCount?: (
			nodeId: string,
		) => { readonly done: number; readonly total: number } | undefined;
	} = $props();

	// Grouping options
	const GROUP_OPTIONS: ReadonlyArray<{ readonly value: GroupBy; readonly label: string }> = [
		{ value: "status", label: "Group by Status" },
		{ value: "priority", label: "Group by Priority" },
	];
	let groupBy = $state<GroupBy>("status");
	const groupByLabel = $derived(
		GROUP_OPTIONS.find((o) => o.value === groupBy)?.label ?? "Group by Status",
	);

	// Drag state
	let dragNodeId = $state<string | null>(null);

	// All-done view: whether the user has clicked "View board" to override the all-done state
	let showBoardOverride = $state(false);

	/**
	 * Returns the list of artifact nodes that belong to the given column key.
	 * @param colKey - The column key to filter nodes by (e.g. status or priority value).
	 * @returns The artifact nodes assigned to that column.
	 */
	function nodesForColumn(colKey: string): ArtifactNode[] {
		if (groupBy === "priority") {
			// Remap priority columns to P1/P2/P3/none
			return nodes.filter((n) => (n.priority ?? "none") === colKey) as ArtifactNode[];
		}
		if (groupBy === "status") {
			return nodes.filter(
				(n) => (n.status ?? "").toLowerCase() === colKey.toLowerCase(),
			) as ArtifactNode[];
		}
		return assertNever(groupBy);
	}

	/**
	 * Initiates a drag operation for the given artifact node.
	 * @param e - The drag event from the browser.
	 * @param node - The artifact node being dragged.
	 */
	function handleDragStart(e: DragEvent, node: ArtifactNode) {
		dragNodeId = node.id;
		e.dataTransfer?.setData("text/plain", node.id);
	}

	/**
	 * Handles dropping a dragged artifact node into the target column.
	 * @param e - The drop event from the browser.
	 * @param colKey - The column key where the node was dropped.
	 */
	function handleDrop(e: DragEvent, colKey: string) {
		e.preventDefault();
		const nodeId = e.dataTransfer?.getData("text/plain") ?? dragNodeId;
		if (!nodeId) return;
		const node = nodes.find((n) => n.id === nodeId);
		if (!node) return;

		const currentValue = groupBy === "priority" ? (node.priority ?? "") : (node.status ?? "");

		if (currentValue === colKey) return;

		onFieldChange?.(node, colKey);
		dragNodeId = null;
	}

	// Priority columns (used when groupBy === "priority")
	const PRIORITY_COLUMNS: readonly ColumnDef[] = [
		{ key: "P1", label: "P1 — Critical" },
		{ key: "P2", label: "P2 — High" },
		{ key: "P3", label: "P3 — Normal" },
		{ key: "none", label: "Unranked", isDone: true },
	];

	const activeColumns = $derived.by((): readonly ColumnDef[] => {
		if (groupBy === "priority") return PRIORITY_COLUMNS;
		if (groupBy === "status") return columns;
		return assertNever(groupBy);
	});

	const totalNodes = $derived(nodes.length);

	// Count nodes that are NOT in a done column (status mode only)
	const nonDoneCount = $derived(
		nodes.filter((n) => {
			if (groupBy === "priority") return false; // priority mode has no "done" semantics
			const doneKeys = activeColumns.filter((c) => c.isDone).map((c) => c.key.toLowerCase());
			return !doneKeys.includes((n.status ?? "").toLowerCase());
		}).length,
	);

	const doneNodes = $derived(totalNodes - nonDoneCount);

	// All-done: items exist, none are in non-done columns, status grouping, user hasn't overridden
	const isAllDone = $derived(
		totalNodes > 0 && nonDoneCount === 0 && groupBy === "status" && !showBoardOverride,
	);

	// Done column is collapsed only when there are non-done items present
	/**
	 * Determines whether the done column should be collapsed.
	 * @param col - The column definition to check.
	 * @returns True if the column is a done column and there are non-done items.
	 */
	function doneColumnCollapsed(col: ColumnDef): boolean {
		if (!col.isDone) return false;
		return nonDoneCount > 0;
	}
</script>

<Stack gap={3} height="full">
	<!-- Toolbar -->
	<HStack justify="between" align="center">
		<Caption variant="caption-tabular">{doneNodes}/{totalNodes} Done</Caption>
		<SelectMenu
			items={GROUP_OPTIONS as Array<{ value: string; label: string }>}
			selected={groupBy}
			onSelect={(v) => {
				groupBy = v as GroupBy;
				showBoardOverride = false;
			}}
			triggerLabel={groupByLabel}
			triggerSize="sm"
		/>
	</HStack>

	<!-- All-done state -->
	{#if isAllDone}
		<Center flex={1}>
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
		</Center>
	{:else}
		<!-- Kanban columns -->
		<Box minHeight={0} flex={1}>
			<HStack gap={3} height="full">
				{#if totalNodes === 0}
					<Center flex={1}>
						<EmptyState icon="layers" title="No items" description="Nothing to show here yet." />
					</Center>
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
								<Caption>No items</Caption>
							{:else}
								{#each colNodes as node (node.id)}
									<KanbanCard
										{node}
										taskCount={getTaskCount?.(node.id)}
										onClick={onCardClick ? () => onCardClick(node) : undefined}
										onDragStart={onFieldChange ? (e) => handleDragStart(e, node) : undefined}
									/>
								{/each}
							{/if}
						</CollapsibleColumn>
					{/each}
				{/if}
			</HStack>
		</Box>
	{/if}
</Stack>
