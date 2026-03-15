<script lang="ts">
	import KanbanIcon from "@lucide/svelte/icons/kanban";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import type { ArtifactNode } from "$lib/types/artifact-graph";
	import HorizonBoard from "./HorizonBoard.svelte";
	import StatusKanban from "./StatusKanban.svelte";
	import DrilldownBreadcrumbs from "./DrilldownBreadcrumbs.svelte";

	// ---------------------------------------------------------------------------
	// Delivery type hierarchy from project config (with fallback to hardcoded keys)
	// ---------------------------------------------------------------------------

	const deliveryTypes = $derived(projectStore.projectSettings?.delivery?.types ?? []);

	/** Level 0: root type — no parent (e.g. milestone). */
	const rootType = $derived(deliveryTypes.find((t) => !t.parent) ?? null);
	/** Level 1: child of root (e.g. epic). */
	const level1Type = $derived(
		deliveryTypes.find((t) => t.parent?.type === (rootType?.key ?? "milestone")) ?? null,
	);
	/** Level 2: child of level1 (e.g. task). */
	const level2Type = $derived(
		deliveryTypes.find((t) => t.parent?.type === (level1Type?.key ?? "epic")) ?? null,
	);

	/** Keys used for byType() lookups — fall back to well-known names. */
	const rootKey = $derived(rootType?.key ?? "milestone");
	const level1Key = $derived(level1Type?.key ?? "epic");
	const level2Key = $derived(level2Type?.key ?? "task");

	/** Field on level-1 artifacts that points to the root (e.g. "milestone"). */
	const level1ParentField = $derived(level1Type?.parent?.field ?? rootKey);
	/** Field on level-2 artifacts that points to level-1 (e.g. "epic"). */
	const level2ParentField = $derived(level2Type?.parent?.field ?? level1Key);

	/** Labels for UI display. */
	const rootLabel = $derived(rootType?.label ?? "Milestone");
	const level1Label = $derived(level1Type?.label ?? "Epic");
	const level2Label = $derived(level2Type?.label ?? "Task");

	// ---------------------------------------------------------------------------
	// Data from graph SDK
	// ---------------------------------------------------------------------------

	const milestones = $derived(artifactGraphSDK.byType(rootKey));
	const epics = $derived(artifactGraphSDK.byType(level1Key));
	const tasks = $derived(artifactGraphSDK.byType(level2Key));
	const graphLoading = $derived(artifactGraphSDK.loading);
	const graphError = $derived(artifactGraphSDK.error);
	const hasData = $derived(milestones.length > 0 || epics.length > 0);

	// ---------------------------------------------------------------------------
	// Drill-down state
	// level 0 = horizon board (all milestones)
	// level 1 = milestone detail (epics kanban)
	// level 2 = epic detail (tasks kanban)
	// ---------------------------------------------------------------------------

	let selectedMilestone = $state<ArtifactNode | null>(null);
	let selectedEpic = $state<ArtifactNode | null>(null);

	const drillLevel = $derived(
		selectedEpic ? 2 : selectedMilestone ? 1 : 0,
	);

	// ---------------------------------------------------------------------------
	// Breadcrumb items derived from drill level
	// ---------------------------------------------------------------------------

	const breadcrumbItems = $derived.by(() => {
		const items: Array<{ label: string; onClick: () => void }> = [
			{
				label: "Roadmap",
				onClick: () => {
					selectedMilestone = null;
					selectedEpic = null;
				},
			},
		];
		if (selectedMilestone) {
			items.push({
				label: `${selectedMilestone.id}: ${selectedMilestone.title}`,
				onClick: () => {
					selectedEpic = null;
				},
			});
		}
		if (selectedEpic) {
			items.push({
				label: `${selectedEpic.id}: ${selectedEpic.title}`,
				onClick: () => {
					// already at level 2, no-op
				},
			});
		}
		return items;
	});

	// ---------------------------------------------------------------------------
	// Horizon columns for milestones
	// ---------------------------------------------------------------------------

	/**
	 * Determine a milestone's horizon bucket.
	 * Uses the `horizon` frontmatter field if present, otherwise infers from status.
	 */
	function milestoneHorizon(ms: ArtifactNode): string {
		const fm = ms.frontmatter;
		if (typeof fm["horizon"] === "string") return fm["horizon"];
		const s = ms.status ?? "captured";
		if (s === "active") return "now";
		if (s === "completed" || s === "surpassed") return "done";
		if (s === "captured" || s === "exploring") return "later";
		return "next";
	}

	type HorizonCol = {
		key: string;
		label: string;
		description: string;
		milestones: ArtifactNode[];
		isDone?: boolean;
	};

	const horizonColumns = $derived.by((): HorizonCol[] => {
		const now: ArtifactNode[] = [];
		const next: ArtifactNode[] = [];
		const later: ArtifactNode[] = [];
		const done: ArtifactNode[] = [];

		for (const ms of milestones) {
			const h = milestoneHorizon(ms);
			if (h === "now") now.push(ms);
			else if (h === "next") next.push(ms);
			else if (h === "later") later.push(ms);
			else if (h === "done") done.push(ms);
			else next.push(ms); // default bucket
		}

		return [
			{ key: "now", label: "Now", description: "Active milestones", milestones: now },
			{ key: "next", label: "Next", description: "Planned — not started", milestones: next },
			{ key: "later", label: "Later", description: "Future milestones", milestones: later },
			{
				key: "done",
				label: "Completed",
				description: "Finished milestones",
				milestones: done,
				isDone: true,
			},
		];
	});

	// ---------------------------------------------------------------------------
	// Epic columns (for milestone drilldown)
	// ---------------------------------------------------------------------------

	const EPIC_COLUMNS = [
		{ key: "captured", label: "Captured" },
		{ key: "ready", label: "Ready" },
		{ key: "active", label: "Active" },
		{ key: "review", label: "Review" },
		{ key: "completed", label: "Completed", isDone: true },
	];

	const epicColumns = $derived.by(() => {
		return EPIC_COLUMNS;
	});

	/** Level-1 items (epics) that belong to the selected root item (milestone). */
	const milestoneEpics = $derived.by((): ArtifactNode[] => {
		const ms = selectedMilestone;
		if (!ms) return [];
		return epics.filter((e) => e.frontmatter[level1ParentField] === ms.id);
	});

	// ---------------------------------------------------------------------------
	// Task columns (for epic drilldown)
	// ---------------------------------------------------------------------------

	const TASK_COLUMNS = [
		{ key: "captured", label: "Captured" },
		{ key: "ready", label: "Ready" },
		{ key: "active", label: "Active" },
		{ key: "review", label: "Review" },
		{ key: "completed", label: "Completed", isDone: true },
	];

	/** Level-2 items (tasks) that belong to the selected level-1 item (epic). */
	const epicTasks = $derived.by((): ArtifactNode[] => {
		const ep = selectedEpic;
		if (!ep) return [];
		return tasks.filter((t) => t.frontmatter[level2ParentField] === ep.id);
	});

	// ---------------------------------------------------------------------------
	// Task count helper (used for epic cards)
	// ---------------------------------------------------------------------------

	function taskCountForEpic(epicId: string): { done: number; total: number } {
		const epicTaskList = tasks.filter(
			(t) => t.frontmatter[level2ParentField] === epicId,
		);
		const done = epicTaskList.filter((t) => t.status === "completed").length;
		return { done, total: epicTaskList.length };
	}

	// ---------------------------------------------------------------------------
	// Field update via backend (drag and drop persists)
	// ---------------------------------------------------------------------------

	async function updateField(
		node: ArtifactNode,
		field: string,
		value: string,
	): Promise<void> {
		try {
			await artifactGraphSDK.updateField(node.path, field, value);
		} catch (err) {
			console.error("[RoadmapView] updateField failed:", err);
		}
	}

	// ---------------------------------------------------------------------------
	// Navigation handlers
	// ---------------------------------------------------------------------------

	function handleMilestoneClick(ms: ArtifactNode) {
		selectedMilestone = ms;
		selectedEpic = null;
	}

	function handleEpicClick(epic: ArtifactNode) {
		if (drillLevel === 1) {
			selectedEpic = epic;
		} else {
			// Level 0 shouldn't directly show epics, but handle gracefully
			navigationStore.navigateToArtifact(epic.id);
		}
	}

	function handleTaskClick(task: ArtifactNode) {
		navigationStore.navigateToArtifact(task.id);
	}
</script>

<div class="flex h-full flex-col">
	<!-- Breadcrumb bar -->
	{#if drillLevel > 0}
		<div class="flex items-center border-b border-border px-6 py-2">
			<DrilldownBreadcrumbs items={breadcrumbItems} />
		</div>
	{/if}

	<!-- Main content -->
	<div class="flex min-h-0 flex-1 flex-col">
		{#if graphLoading && !hasData}
			<div class="flex flex-1 items-center justify-center">
				<LoadingSpinner />
			</div>
		{:else if graphError && !hasData}
			<div class="p-6">
				<ErrorDisplay
					message={graphError}
					onRetry={() => artifactGraphSDK.refresh()}
				/>
			</div>
		{:else if !hasData}
			<div class="flex flex-1 items-center justify-center">
				<EmptyState
					icon={KanbanIcon}
					title="No {rootLabel.toLowerCase()}s found"
					description="Create {rootLabel.toLowerCase()}s to see them here."
				/>
			</div>
		{:else if drillLevel === 0}
			<!-- Level 0: Horizon board -->
			<div class="flex h-full flex-col px-6 py-4">
				<div class="mb-4">
					<div class="flex items-center gap-3">
						<KanbanIcon class="h-6 w-6 text-muted-foreground" />
						<div>
							<h1 class="text-xl font-bold">Roadmap</h1>
							<p class="text-xs text-muted-foreground">
								Click a {rootLabel.toLowerCase()} to drill into its {level1Label.toLowerCase()}s.
							</p>
						</div>
					</div>
				</div>
				<div class="min-h-0 flex-1 overflow-hidden">
					<HorizonBoard
						columns={horizonColumns}
						{epics}
						epicParentField={level1ParentField}
						epicLabel={level1Label}
						{rootLabel}
						onMilestoneClick={handleMilestoneClick}
						onHorizonChange={async (ms, horizon) =>
							updateField(ms, "horizon", horizon)}
					/>
				</div>
			</div>
		{:else if drillLevel === 1 && selectedMilestone}
			<!-- Level 1: Milestone → Epics kanban -->
			<div class="flex h-full flex-col px-6 py-4">
				<!-- Milestone detail header -->
				<div class="mb-4">
					<div class="flex items-start gap-2">
						<div>
							<p class="text-[10px] font-mono text-muted-foreground/60">
								{selectedMilestone.id}
							</p>
							<h1 class="text-xl font-bold">{selectedMilestone.title}</h1>
							{#if selectedMilestone.description}
								<p class="mt-0.5 text-sm text-muted-foreground">
									{selectedMilestone.description}
								</p>
							{/if}
							{#if milestoneEpics.length > 0}
								{@const doneCount = milestoneEpics.filter(
									(e) => e.status === "completed",
								).length}
								<p class="mt-1 text-xs text-muted-foreground">
									{doneCount}/{milestoneEpics.length} {level1Label.toLowerCase()}s done
								</p>
							{/if}
						</div>
					</div>
				</div>

				<!-- Epics kanban -->
				<div class="min-h-0 flex-1 overflow-hidden">
					<StatusKanban
						nodes={milestoneEpics}
						columns={epicColumns}
						onCardClick={handleEpicClick}
						onFieldChange={async (epic, newStatus) =>
							updateField(epic, "status", newStatus)}
						getTaskCount={(epicId) => taskCountForEpic(epicId)}
					/>
				</div>
			</div>
		{:else if drillLevel === 2 && selectedEpic}
			<!-- Level 2: Epic → Tasks kanban -->
			<div class="flex h-full flex-col px-6 py-4">
				<!-- Epic detail header -->
				<div class="mb-4">
					<div>
						<p class="text-[10px] font-mono text-muted-foreground/60">
							{selectedEpic.id}
						</p>
						<h1 class="text-xl font-bold">{selectedEpic.title}</h1>
						{#if selectedEpic.description}
							<p class="mt-0.5 text-sm text-muted-foreground">
								{selectedEpic.description}
							</p>
						{/if}
						{#if epicTasks.length > 0}
							{@const doneCount = epicTasks.filter(
								(t) => t.status === "completed",
							).length}
							<p class="mt-1 text-xs text-muted-foreground">
								{doneCount}/{epicTasks.length} {level2Label.toLowerCase()}s done
							</p>
						{/if}
					</div>
				</div>

				<!-- Tasks kanban -->
				<div class="min-h-0 flex-1 overflow-hidden">
					<StatusKanban
						nodes={epicTasks}
						columns={TASK_COLUMNS}
						onCardClick={handleTaskClick}
						onFieldChange={async (task, newStatus) =>
							updateField(task, "status", newStatus)}
					/>
				</div>
			</div>
		{/if}
	</div>
</div>
