<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import type { ArtifactNode } from "$lib/types/artifact-graph";
	import MilestoneCard from "./MilestoneCard.svelte";
	import SelectMenu from "$lib/components/shared/SelectMenu.svelte";
	import { Badge } from "$lib/components/ui/badge";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { cn } from "$lib/utils";

	type HorizonColumn = {
		key: string;
		label: string;
		description: string;
		milestones: ArtifactNode[];
		isDone?: boolean;
	};

	let {
		columns,
		epics,
		onMilestoneClick,
		onHorizonChange,
	}: {
		columns: HorizonColumn[];
		epics: ArtifactNode[];
		onMilestoneClick: (milestone: ArtifactNode) => void;
		onHorizonChange?: (milestone: ArtifactNode, newHorizon: string) => Promise<void>;
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

	function handleDragLeave() {
		dropTargetKey = null;
	}

	function handleDrop(e: DragEvent, colKey: string) {
		e.preventDefault();
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
		const s = ms.status ?? "planning";
		if (s === "active") return "now";
		if (s === "complete") return "done";
		return "next";
	}

	function epicsForMilestone(msId: string): ArtifactNode[] {
		return epics.filter((e) => e.frontmatter["milestone"] === msId);
	}

	// Sort/group options for the horizon board
	const SORT_OPTIONS = [
		{ value: "horizon", label: "Group by Horizon" },
		{ value: "status", label: "Group by Status" },
		{ value: "priority", label: "Group by Priority" },
	];
	let sortBy = $state("horizon");
	const sortByLabel = $derived(
		SORT_OPTIONS.find((o) => o.value === sortBy)?.label ?? "Group by Horizon",
	);

	// When grouping by status, derive columns from milestone statuses
	const STATUS_COLUMNS = [
		{ key: "planning", label: "Planning", isDone: false },
		{ key: "active", label: "Active", isDone: false },
		{ key: "complete", label: "Complete", isDone: true },
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
		key: string;
		label: string;
		description: string;
		milestones: ArtifactNode[];
		isDone?: boolean;
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
		// Default: use the horizon columns passed as props
		return columns;
	});

</script>

<div class="flex h-full flex-col gap-3">
	<!-- Toolbar -->
	<div class="flex items-center justify-between">
		<span class="text-xs text-muted-foreground">
			{allMilestones.length} milestone{allMilestones.length === 1 ? '' : 's'}
		</span>
		<SelectMenu
			items={SORT_OPTIONS}
			selected={sortBy}
			onSelect={(v) => { sortBy = v; }}
			triggerLabel={sortByLabel}
			triggerSize="sm"
		/>
	</div>

	<!-- Horizon columns -->
	<ScrollArea.Root class="min-h-0 flex-1" orientation="horizontal">
		<div class="flex h-full min-w-max gap-4 pb-4">
			{#each activeColumns as col (col.key)}
				{@const isCollapsed = col.isDone === true && collapsedCols.has(col.key)}
				{@const isDrop = dropTargetKey === col.key}
				{@const totalMilestones = allMilestones.length}

				{#if isCollapsed}
					<!-- Thin collapsed bar for done column -->
					<div
						class={cn(
							"flex w-10 shrink-0 cursor-pointer flex-col items-center rounded-xl border border-dashed border-border bg-muted/30 transition-colors hover:bg-muted/50",
							isDrop && "border-primary bg-primary/10",
						)}
						onclick={() => toggleCollapsed(col.key)}
						ondragover={(e) => handleDragOver(e, col.key)}
						ondragleave={handleDragLeave}
						ondrop={(e) => handleDrop(e, col.key)}
						role="button"
						tabindex="0"
						onkeydown={(e) => e.key === "Enter" && toggleCollapsed(col.key)}
						aria-label="Expand {col.label} column"
					>
						<div class="flex flex-1 flex-col items-center justify-center gap-2 py-4">
							<span
								class="text-xs font-medium text-muted-foreground select-none capitalize"
								style="writing-mode: vertical-rl; transform: rotate(180deg);"
							>
								{col.label}
							</span>
							{#if col.milestones.length > 0}
								<span class="flex h-5 w-5 items-center justify-center rounded-full bg-muted text-[10px] font-semibold tabular-nums text-muted-foreground">
									{col.milestones.length}
								</span>
							{/if}
						</div>
					</div>
				{:else}
					<!-- Expanded column -->
					<div
						class={cn(
							"flex min-w-60 flex-1 flex-col rounded-xl border border-border bg-muted/5 transition-colors",
							isDrop && "border-primary bg-primary/5",
						)}
						ondragover={(e) => handleDragOver(e, col.key)}
						ondragleave={handleDragLeave}
						ondrop={(e) => handleDrop(e, col.key)}
						role="region"
						aria-label="{col.label} horizon column"
					>
						<!-- Column header -->
						<div class="border-b border-border px-4 py-3">
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									<Badge variant="outline" class="text-xs font-semibold capitalize">
										{col.label}
									</Badge>
									{#if col.isDone && totalMilestones > 0}
										<span class="text-[10px] tabular-nums text-muted-foreground">
											{col.milestones.length}/{totalMilestones} Done
										</span>
									{/if}
								</div>
								{#if col.isDone}
									<button
										class="rounded p-0.5 text-muted-foreground hover:text-foreground transition-colors text-xs"
										onclick={() => toggleCollapsed(col.key)}
										aria-label="Collapse {col.label}"
									>
										<span class="text-xs">&rarr;</span>
									</button>
								{/if}
							</div>
							{#if col.description}
								<p class="mt-1 text-xs text-muted-foreground">{col.description}</p>
							{/if}
						</div>

						<!-- Milestone cards -->
						<ScrollArea.Root class="min-h-0 flex-1" orientation="vertical">
							<div
								class="flex flex-col gap-3 p-3"
								role="list"
								ondragover={(e) => handleDragOver(e, col.key)}
								ondragleave={handleDragLeave}
								ondrop={(e) => handleDrop(e, col.key)}
							>
								{#if col.milestones.length === 0}
									<div class="flex flex-1 items-center justify-center rounded-lg border border-dashed border-border p-6 text-center text-xs text-muted-foreground">
										No milestones
									</div>
								{:else}
									{#each col.milestones as ms (ms.id)}
										{@const msEpics = epicsForMilestone(ms.id)}
										{@const doneCount = msEpics.filter((e) => e.status === "done").length}
										{@const inProgress = msEpics.filter((e) => e.status === "in-progress")}
										{@const critical = msEpics.filter(
											(e) => e.priority === "P1" && e.status !== "done",
										)}
										<div
											draggable={onHorizonChange !== undefined && sortBy === "horizon"}
											ondragstart={(e) => handleDragStart(e, ms)}
											class={cn(onHorizonChange && sortBy === "horizon" && "cursor-grab active:cursor-grabbing")}
											role="listitem"
										>
											<MilestoneCard
												milestone={ms}
												epicCount={msEpics.length}
												doneEpicCount={doneCount}
												inProgressEpics={inProgress}
												criticalEpics={critical}
												onClick={() => onMilestoneClick(ms)}
											/>
										</div>
									{/each}
								{/if}
							</div>
						</ScrollArea.Root>
					</div>
				{/if}
			{/each}
		</div>
	</ScrollArea.Root>
</div>
