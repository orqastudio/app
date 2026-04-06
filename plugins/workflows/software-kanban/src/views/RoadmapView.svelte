<!-- RoadmapView: top-level plugin view. Accesses stores and delegates all data to child components.
     Provides drill-down from milestone horizon board → epic kanban → task kanban. -->
<script lang="ts">
	import { Icon, EmptyState } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import {
		Heading,
		Panel,
		SectionHeader,
		Text,
		Caption,
		Stack,
		HStack,
		Box,
		Center,
	} from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	// Store access boundary: only RoadmapView (the plugin root component) accesses
	// stores directly. All child components receive data via props (props-in, events-out).
	const { artifactGraphSDK, navigationStore, projectStore } = getStores();
	import type { ArtifactNode } from "@orqastudio/types";

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

	/** Relationship type on level-1 artifacts that connects to root (e.g. "delivers"). */
	const level1ParentRel = $derived(level1Type?.parent?.relationship ?? "delivers");
	/** Relationship type on level-2 artifacts that connects to level-1 (e.g. "delivers"). */
	const level2ParentRel = $derived(level2Type?.parent?.relationship ?? "delivers");

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

	const drillLevel = $derived(selectedEpic ? 2 : selectedMilestone ? 1 : 0);

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
	 * @param ms - The milestone artifact node to determine a horizon for.
	 * @returns The horizon key string ("now", "next", "later", or "done").
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
		readonly key: string;
		readonly label: string;
		readonly description: string;
		readonly milestones: readonly ArtifactNode[];
		readonly isDone?: boolean;
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
		return epics.filter((e) =>
			e.references_out.some(
				(r) => r.relationship_type === level1ParentRel && r.target_id === ms.id,
			),
		);
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
		return tasks.filter((t) =>
			t.references_out.some(
				(r) => r.relationship_type === level2ParentRel && r.target_id === ep.id,
			),
		);
	});

	// ---------------------------------------------------------------------------
	// Task count helper (used for epic cards)
	// ---------------------------------------------------------------------------

	/**
	 * Returns the done and total task counts for a given epic, used to render progress on epic cards.
	 * @param epicId - The epic artifact ID to count tasks for.
	 * @returns An object with `done` and `total` task counts for the epic.
	 */
	function taskCountForEpic(epicId: string): { done: number; total: number } {
		const epicTaskList = tasks.filter((t) =>
			t.references_out.some(
				(r) => r.relationship_type === level2ParentRel && r.target_id === epicId,
			),
		);
		const done = epicTaskList.filter((t) => t.status === "completed").length;
		return { done, total: epicTaskList.length };
	}

	// ---------------------------------------------------------------------------
	// Field update via backend (drag and drop persists)
	// ---------------------------------------------------------------------------

	/**
	 * Persists a frontmatter field update for an artifact via the graph SDK.
	 * @param node - The artifact node whose field is being updated.
	 * @param field - The frontmatter field name to update (e.g. "horizon", "status").
	 * @param value - The new value to write for the field.
	 */
	async function updateField(node: ArtifactNode, field: string, value: string): Promise<void> {
		try {
			await artifactGraphSDK.updateField(node.path, field, value);
		} catch (err) {
			console.error("[RoadmapView] updateField failed:", err);
		}
	}

	// ---------------------------------------------------------------------------
	// Navigation handlers
	// ---------------------------------------------------------------------------

	/**
	 * Handles milestone click events, drilling down to the milestone's epic kanban.
	 * @param ms - The milestone artifact node that was clicked.
	 */
	function handleMilestoneClick(ms: ArtifactNode) {
		selectedMilestone = ms;
		selectedEpic = null;
	}

	/**
	 * Handles epic click events, drilling down to the epic's task kanban or navigating directly.
	 * @param epic - The epic artifact node that was clicked.
	 */
	function handleEpicClick(epic: ArtifactNode) {
		if (drillLevel === 1) {
			selectedEpic = epic;
		} else {
			// Level 0 shouldn't directly show epics, but handle gracefully
			navigationStore.navigateToArtifact(epic.id);
		}
	}

	/**
	 * Handles task click events, navigating the app to the task's artifact detail view.
	 * @param task - The task artifact node that was clicked.
	 */
	function handleTaskClick(task: ArtifactNode) {
		navigationStore.navigateToArtifact(task.id);
	}
</script>

<Stack gap={0} height="full">
	<!-- Breadcrumb bar -->
	{#if drillLevel > 0}
		<SectionHeader>
			<DrilldownBreadcrumbs items={breadcrumbItems} />
		</SectionHeader>
	{/if}

	<!-- Main content -->
	<Stack flex={1} minHeight={0}>
		{#if graphLoading && !hasData}
			<Center flex={1}>
				<LoadingSpinner />
			</Center>
		{:else if graphError && !hasData}
			<Panel padding="loose">
				<ErrorDisplay message={graphError} onRetry={() => artifactGraphSDK.refresh()} />
			</Panel>
		{:else if !hasData}
			<Center flex={1}>
				<EmptyState
					icon="kanban"
					title="No {rootLabel.toLowerCase()}s found"
					description="Create {rootLabel.toLowerCase()}s to see them here."
				/>
			</Center>
		{:else if drillLevel === 0}
			<!-- Level 0: Horizon board -->
			<Panel padding="loose" full>
				<Stack gap={4} height="full">
					<HStack gap={3} align="center">
						<Icon name="kanban" size="xl" />
						<Stack gap={0}>
							<Heading level={2}>Roadmap</Heading>
							<Caption>
								Click a {rootLabel.toLowerCase()} to drill into its {level1Label.toLowerCase()}s.
							</Caption>
						</Stack>
					</HStack>
					<Box minHeight={0} flex={1}>
						<HorizonBoard
							columns={horizonColumns}
							{epics}
							epicParentRel={level1ParentRel}
							epicLabel={level1Label}
							{rootLabel}
							onMilestoneClick={handleMilestoneClick}
							onHorizonChange={async (ms, horizon) => updateField(ms, "horizon", horizon)}
						/>
					</Box>
				</Stack>
			</Panel>
		{:else if drillLevel === 1 && selectedMilestone}
			<!-- Level 1: Milestone → Epics kanban -->
			<Panel padding="loose" full>
				<Stack gap={4} height="full">
					<!-- Milestone detail header -->
					<Stack gap={1}>
						<Stack gap={0}>
							<Caption variant="caption-mono">{selectedMilestone.id}</Caption>
							<Heading level={2}>{selectedMilestone.title}</Heading>
						</Stack>
						{#if selectedMilestone.description}
							<Text variant="body-muted">{selectedMilestone.description}</Text>
						{/if}
						{#if milestoneEpics.length > 0}
							{@const doneCount = milestoneEpics.filter((e) => e.status === "completed").length}
							<Caption>
								{doneCount}/{milestoneEpics.length}
								{level1Label.toLowerCase()}s done
							</Caption>
						{/if}
					</Stack>

					<!-- Epics kanban -->
					<Box minHeight={0} flex={1}>
						<StatusKanban
							nodes={milestoneEpics}
							columns={epicColumns}
							onCardClick={handleEpicClick}
							onFieldChange={async (epic, newStatus) => updateField(epic, "status", newStatus)}
							getTaskCount={(epicId) => taskCountForEpic(epicId)}
						/>
					</Box>
				</Stack>
			</Panel>
		{:else if drillLevel === 2 && selectedEpic}
			<!-- Level 2: Epic → Tasks kanban -->
			<Panel padding="loose" full>
				<Stack gap={4} height="full">
					<!-- Epic detail header -->
					<Stack gap={1}>
						<Stack gap={0}>
							<Caption variant="caption-mono">{selectedEpic.id}</Caption>
							<Heading level={2}>{selectedEpic.title}</Heading>
						</Stack>
						{#if selectedEpic.description}
							<Text variant="body-muted">{selectedEpic.description}</Text>
						{/if}
						{#if epicTasks.length > 0}
							{@const doneCount = epicTasks.filter((t) => t.status === "completed").length}
							<Caption>
								{doneCount}/{epicTasks.length}
								{level2Label.toLowerCase()}s done
							</Caption>
						{/if}
					</Stack>

					<!-- Tasks kanban -->
					<Box minHeight={0} flex={1}>
						<StatusKanban
							nodes={epicTasks}
							columns={TASK_COLUMNS}
							onCardClick={handleTaskClick}
							onFieldChange={async (task, newStatus) => updateField(task, "status", newStatus)}
						/>
					</Box>
				</Stack>
			</Panel>
		{/if}
	</Stack>
</Stack>
