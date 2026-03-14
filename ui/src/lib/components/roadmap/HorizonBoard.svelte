<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import type { ArtifactNode } from "$lib/types/artifact-graph";
	import MilestoneCard from "./MilestoneCard.svelte";
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

</script>

<div class="flex h-full gap-4 overflow-x-auto pb-4">
	{#each columns as col (col.key)}
		{@const isCollapsed = col.isDone === true && collapsedCols.has(col.key)}
		{@const isDrop = dropTargetKey === col.key}

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
						class="text-xs font-medium text-muted-foreground select-none"
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
						<div>
							<h3 class="text-sm font-semibold">{col.label}</h3>
							<p class="text-xs text-muted-foreground">{col.description}</p>
						</div>
						<div class="flex items-center gap-2">
							<span class="rounded-full bg-muted px-2 py-0.5 text-[10px] font-semibold tabular-nums text-muted-foreground">
								{col.milestones.length}
							</span>
							{#if col.isDone}
								<button
									class="rounded p-0.5 text-muted-foreground hover:text-foreground transition-colors text-xs"
									onclick={() => toggleCollapsed(col.key)}
									aria-label="Collapse {col.label}"
								>
									&#8594;
								</button>
							{/if}
						</div>
					</div>
				</div>

				<!-- Milestone cards -->
				<div class="flex flex-1 flex-col gap-3 overflow-y-auto p-3">
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
								draggable={onHorizonChange !== undefined}
								ondragstart={(e) => handleDragStart(e, ms)}
								class={cn(onHorizonChange && "cursor-grab active:cursor-grabbing")}
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
			</div>
		{/if}
	{/each}
</div>
