<!-- HorizonBoard: displays milestones grouped into horizon columns (Now/Next/Later/Done).
     Supports drag-and-drop to change a milestone's horizon, and sort/group by status or priority. -->
<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import type { ArtifactNode } from "@orqastudio/types";
	import { assertNever } from "@orqastudio/types";
	import MilestoneCard from "./MilestoneCard.svelte";
	import {
		SelectMenu,
		Badge,
		ScrollArea,
		Button,
		Caption,
		Text,
		Icon,
		EmptyState,
		CollapsibleRoot,
		CollapsibleTrigger,
		CollapsibleContent,
		Stack,
		HStack,
		Box,
		Center,
		CountBadge,
		VerticalText,
	} from "@orqastudio/svelte-components/pure";

	type HorizonColumn = {
		readonly key: string;
		readonly label: string;
		readonly description: string;
		readonly milestones: readonly ArtifactNode[];
		readonly isDone?: boolean;
	};

	/** Discriminated union of valid sort/group modes for the horizon board. */
	type SortBy = "horizon" | "status" | "priority";

	let {
		columns,
		epics,
		epicParentRel = "delivers",
		epicLabel = "Epic",
		rootLabel = "Milestone",
		onMilestoneClick,
		onHorizonChange,
	}: {
		readonly columns: readonly HorizonColumn[];
		readonly epics: readonly ArtifactNode[];
		/** The relationship type on epics that connects to the parent milestone. Defaults to "delivers". */
		readonly epicParentRel?: string;
		/** Display label for the level-1 type (e.g. "Epic"). Used in card counts. */
		readonly epicLabel?: string;
		/** Display label for the root type (e.g. "Milestone"). Used in empty state text. */
		readonly rootLabel?: string;
		readonly onMilestoneClick: (milestone: ArtifactNode) => void;
		readonly onHorizonChange?: (milestone: ArtifactNode, newHorizon: string) => Promise<void>;
	} = $props();

	// Collapsed state for the "done" column — SvelteSet is inherently reactive
	const collapsedCols = new SvelteSet<string>(["done"]);

	function toggleCollapsed(key: string) {
		if (collapsedCols.has(key)) {
			collapsedCols.delete(key);
		} else {
			collapsedCols.add(key);
		}
	}

	// Drag and drop
	let dragMilestoneId = $state<string | null>(null);
	let dropTargetKey = $state<string | null>(null);

	function handleDragStart(e: DragEvent, milestone: ArtifactNode) {
		dragMilestoneId = milestone.id;
		e.dataTransfer?.setData("text/plain", milestone.id);
	}

	function handleDragOver(e: DragEvent, colKey: string) {
		e.preventDefault();
		dropTargetKey = colKey;
	}

	function handleDragLeave(e: DragEvent) {
		// Only reset the drop target when the cursor actually leaves the column,
		// not when it moves between child elements.
		const related = e.relatedTarget as Node | null;
		if (related && (e.currentTarget as HTMLElement).contains(related)) return;
		dropTargetKey = null;
	}

	function handleDrop(e: DragEvent, colKey: string) {
		e.preventDefault();
		e.stopPropagation();
		dropTargetKey = null;
		const msId = e.dataTransfer?.getData("text/plain") ?? dragMilestoneId;
		if (!msId) return;

		// Find the milestone across all columns
		let milestone: ArtifactNode | undefined;
		for (const col of columns) {
			milestone = col.milestones.find((m) => m.id === msId);
			if (milestone) break;
		}
		if (!milestone) return;

		const currentHorizon = (milestone.frontmatter["horizon"] as string | undefined) ?? inferHorizon(milestone);
		if (currentHorizon === colKey) return;

		onHorizonChange?.(milestone, colKey);
		dragMilestoneId = null;
	}

	function inferHorizon(ms: ArtifactNode): string {
		const s = ms.status ?? "captured";
		if (s === "active") return "now";
		if (s === "completed" || s === "surpassed") return "done";
		if (s === "captured") return "later";
		if (s === "exploring") return "next";
		return "next";
	}

	function epicsForMilestone(msId: string): ArtifactNode[] {
		return epics.filter((e) =>
			e.references_out.some(
				(r) => r.relationship_type === epicParentRel && r.target_id === msId,
			),
		);
	}

	// Sort/group options for the horizon board
	const SORT_OPTIONS: ReadonlyArray<{ readonly value: SortBy; readonly label: string }> = [
		{ value: "horizon", label: "Group by Horizon" },
		{ value: "status", label: "Group by Status" },
		{ value: "priority", label: "Group by Priority" },
	];
	let sortBy = $state<SortBy>("horizon");
	const sortByLabel = $derived(
		SORT_OPTIONS.find((o) => o.value === sortBy)?.label ?? "Group by Horizon",
	);

	// When grouping by status, derive columns from milestone statuses
	const STATUS_COLUMNS = [
		{ key: "captured", label: "Captured", isDone: false },
		{ key: "exploring", label: "Exploring", isDone: false },
		{ key: "ready", label: "Ready", isDone: false },
		{ key: "active", label: "Active", isDone: false },
		{ key: "review", label: "Review", isDone: false },
		{ key: "completed", label: "Completed", isDone: true },
	];

	// When grouping by priority, derive columns from milestone priorities
	const PRIORITY_COLUMNS = [
		{ key: "P1", label: "P1 — Critical", isDone: false },
		{ key: "P2", label: "P2 — High", isDone: false },
		{ key: "P3", label: "P3 — Normal", isDone: false },
		{ key: "none", label: "Unranked", isDone: false },
	];

	// Flatten all milestones across columns
	const allMilestones = $derived(columns.flatMap((c) => c.milestones));

	type FlatColumn = {
		readonly key: string;
		readonly label: string;
		readonly description: string;
		readonly milestones: readonly ArtifactNode[];
		readonly isDone?: boolean;
	};

	const activeColumns = $derived.by((): FlatColumn[] => {
		if (sortBy === "status") {
			return STATUS_COLUMNS.map((col) => ({
				...col,
				description: "",
				milestones: allMilestones.filter(
					(ms) => (ms.status ?? "planning").toLowerCase() === col.key,
				),
			}));
		}
		if (sortBy === "priority") {
			return PRIORITY_COLUMNS.map((col) => ({
				...col,
				description: "",
				milestones: allMilestones.filter(
					(ms) => (ms.frontmatter["priority"] as string | undefined ?? "none") === col.key,
				),
			}));
		}
		if (sortBy === "horizon") {
			// Use the horizon columns passed as props
			return columns as FlatColumn[];
		}
		return assertNever(sortBy);
	});

</script>

<Stack gap={3} height="full">
	<!-- Toolbar -->
	<HStack justify="between" align="center">
		<Caption variant="caption-tabular">
			{allMilestones.filter(m => m.status === 'complete').length}/{allMilestones.length} Done
		</Caption>
		<SelectMenu
			items={SORT_OPTIONS as Array<{ value: string; label: string }>}
			selected={sortBy}
			onSelect={(v) => { sortBy = v as SortBy; }}
			triggerLabel={sortByLabel}
			triggerSize="sm"
		/>
	</HStack>

	<!-- Horizon columns -->
	<Box minHeight={0} flex={1}>
		<HStack gap={4} height="full" paddingBottom={4}>
			{#each activeColumns as col (col.key)}
				{@const isCollapsed = col.isDone === true && collapsedCols.has(col.key)}
				{@const isDrop = dropTargetKey === col.key}
				{@const totalMilestones = allMilestones.length}

				{@const isColOpen = !isCollapsed}

				<!-- horizon-col class provides h-full flex layout for CollapsibleRoot. -->
				<div class="horizon-col">
					<CollapsibleRoot
						open={isColOpen}
						onOpenChange={(open) => { if (!open) collapsedCols.add(col.key); else collapsedCols.delete(col.key); }}
					>
						{#if isCollapsed}
							<!-- Thin collapsed bar for done column. collapsed-bar class applied via scoped CSS. -->
							<div
								class="collapsed-bar"
								class:drag-over={isDrop}
								ondragover={(e) => handleDragOver(e, col.key)}
								ondragleave={handleDragLeave}
								ondrop={(e) => handleDrop(e, col.key)}
								role="region"
								aria-label="{col.label} column (collapsed)"
							>
								<CollapsibleTrigger aria-label="Expand {col.label} column">
									<Center flex={1} gap={2}>
										<VerticalText variant="caption" tone="muted">{col.label}</VerticalText>
										{#if col.milestones.length > 0}
											<CountBadge count={col.milestones.length} variant="muted" />
										{/if}
									</Center>
								</CollapsibleTrigger>
							</div>
						{:else}
							<!-- Expanded column. column-expanded provides border/bg/min-width via scoped CSS. -->
							<div
								class="column-expanded"
								class:drag-over={isDrop}
								ondragover={(e) => handleDragOver(e, col.key)}
								ondragleave={handleDragLeave}
								ondrop={(e) => handleDrop(e, col.key)}
								role="region"
								aria-label="{col.label} horizon column"
							>
								<!-- Column header -->
								<Box borderBottom paddingX={4} paddingY={3}>
									<HStack justify="between" align="center">
										<HStack gap={2} align="center">
											<Badge variant="outline" size="sm" capitalize>
												{col.label}
											</Badge>
											{#if col.isDone && totalMilestones > 0}
												<Caption variant="caption-tabular">
													{col.milestones.length}/{totalMilestones} Done
												</Caption>
											{/if}
										</HStack>
										{#if col.isDone}
											<CollapsibleTrigger>
												<Button
													variant="ghost"
													size="icon-sm"
													aria-label="Collapse {col.label}"
												>
													<Icon name="chevron-right" size="sm" />
												</Button>
											</CollapsibleTrigger>
										{/if}
									</HStack>
									{#if col.description}
										<Box marginTop={1}>
											<Caption>{col.description}</Caption>
										</Box>
									{/if}
								</Box>

								<!-- Milestone cards -->
								<CollapsibleContent>
									<Box minHeight={0} flex={1}>
										<ScrollArea full orientation="vertical">
											<Stack gap={3} padding={3} role="list">
												{#if col.milestones.length === 0}
													<EmptyState
														title="No {rootLabel.toLowerCase()}s"
														description="Drop a {rootLabel.toLowerCase()} here."
													/>
												{:else}
													{#each col.milestones as ms (ms.id)}
														{@const msEpics = epicsForMilestone(ms.id)}
														{@const doneCount = msEpics.filter((e) => e.status === "completed").length}
														{@const inProgress = msEpics.filter((e) => e.status === "active")}
														{@const critical = msEpics.filter(
															(e) => e.priority === "P1" && e.status !== "completed",
														)}
														<!-- Raw div required: HTML5 drag API needs a native draggable element.
														     Cursor classes moved to scoped CSS. -->
														<div
															draggable={onHorizonChange !== undefined && sortBy === "horizon"}
															ondragstart={(e) => handleDragStart(e, ms)}
															class:draggable-item={onHorizonChange && sortBy === "horizon"}
															role="listitem"
														>
															<MilestoneCard
																milestone={ms}
																epicCount={msEpics.length}
																doneEpicCount={doneCount}
																inProgressEpics={inProgress}
																criticalEpics={critical}
																{epicLabel}
																onClick={() => onMilestoneClick(ms)}
															/>
														</div>
													{/each}
												{/if}
											</Stack>
										</ScrollArea>
									</Box>
								</CollapsibleContent>
							</div>
						{/if}
					</CollapsibleRoot>
				</div>
			{/each}
		</HStack>
	</Box>
</Stack>

<style>
	/* Provides h-full flex layout for the CollapsibleRoot wrapper. */
	.horizon-col {
		display: flex;
		height: 100%;
	}

	/* Thin vertical bar appearance for collapsed column state. */
	.collapsed-bar {
		display: flex;
		width: 2.5rem;
		flex-shrink: 0;
		flex-direction: column;
		align-items: center;
		border-radius: 0.75rem;
		border: 1px dashed var(--color-border);
		background-color: color-mix(in srgb, var(--color-muted) 30%, transparent);
		transition: background-color 0.15s;
		cursor: pointer;
	}

	.collapsed-bar:hover {
		background-color: color-mix(in srgb, var(--color-muted) 50%, transparent);
	}

	.collapsed-bar.drag-over {
		border-color: var(--color-primary);
		background-color: color-mix(in srgb, var(--color-primary) 10%, transparent);
	}

	/* Full column container: sets min-width, flex layout, rounded border, and bg. */
	.column-expanded {
		display: flex;
		min-width: 12rem;
		flex: 1;
		flex-direction: column;
		border-radius: 0.75rem;
		border: 1px solid var(--color-border);
		background-color: color-mix(in srgb, var(--color-muted) 5%, transparent);
		transition: border-color 0.15s, background-color 0.15s;
	}

	.column-expanded.drag-over {
		border-color: var(--color-primary);
		background-color: color-mix(in srgb, var(--color-primary) 5%, transparent);
	}

	/* Cursor for draggable milestone list items. The draggable div cannot use typed
	   props because HTML5 drag API requires a native element. */
	.draggable-item {
		cursor: grab;
	}

	.draggable-item:active {
		cursor: grabbing;
	}
</style>
