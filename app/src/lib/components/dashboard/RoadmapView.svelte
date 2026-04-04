<!-- Roadmap view — kanban board of milestones grouped by status.
     Clicking a milestone card drills into a kanban of its epics grouped by status.
     Breadcrumb navigation and a back button return to the milestone board. -->
<script lang="ts">
	import { Icon, Heading, Badge } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import type { ArtifactNode } from "@orqastudio/types";
	import type { PipelineStageConfig } from "@orqastudio/types";

	const { artifactGraphSDK, navigationStore, pluginRegistry } = getStores();

	// -----------------------------------------------------------------------
	// Drill-down state — null means top-level milestone board
	// -----------------------------------------------------------------------

	/** The milestone the user clicked into, or null when at the top level. */
	let selectedMilestone = $state<ArtifactNode | null>(null);

	// -----------------------------------------------------------------------
	// Pipeline stage helpers
	// -----------------------------------------------------------------------

	/**
	 * Return pipeline stages for an artifact type from the plugin registry.
	 * Falls back to a single "unknown" column when no stages are registered.
	 * @param artifactType - The artifact type key (e.g. "milestone", "epic").
	 * @returns Array of stage configs with key, label, and color.
	 */
	function stagesFor(artifactType: string): PipelineStageConfig[] {
		const stages = pluginRegistry.getPipelineStages(artifactType);
		if (stages.length > 0) return stages;
		return [{ key: "unknown", label: "Unknown", color: "#94a3b8" }];
	}

	/**
	 * Group an array of artifact nodes into a map keyed by status.
	 * Nodes without a status are placed under the first stage key.
	 * @param nodes - Artifact nodes to group.
	 * @param stages - Ordered stage configs defining the columns.
	 * @returns Map from stage key to the nodes in that column.
	 */
	function groupByStatus(
		nodes: ArtifactNode[],
		stages: PipelineStageConfig[],
	): Map<string, ArtifactNode[]> {
		const fallbackKey = stages[0]?.key ?? "unknown";
		const map = new Map<string, ArtifactNode[]>();
		for (const stage of stages) {
			map.set(stage.key, []);
		}
		for (const node of nodes) {
			const key = node.status ?? fallbackKey;
			if (!map.has(key)) {
				map.set(key, []);
			}
			map.get(key)!.push(node);
		}
		return map;
	}

	// -----------------------------------------------------------------------
	// Milestone board data
	// -----------------------------------------------------------------------

	/** All milestones sorted by title. */
	const milestones = $derived.by(() => {
		return artifactGraphSDK.byType("milestone").sort((a, b) =>
			a.title.localeCompare(b.title),
		);
	});

	/** Pipeline stages for milestones. */
	const milestoneStages = $derived(stagesFor("milestone"));

	/** Milestones grouped into kanban columns. */
	const milestoneColumns = $derived(groupByStatus(milestones, milestoneStages));

	// -----------------------------------------------------------------------
	// Epic board data (populated when a milestone is selected)
	// -----------------------------------------------------------------------

	/**
	 * Return epics for the given milestone.
	 * Epics carry an outgoing "fulfils" edge pointing at their parent milestone.
	 * traverseIncoming follows those edges in reverse — milestone → its epics.
	 * @param milestone - The selected milestone node.
	 * @returns Epics sorted by title.
	 */
	function epicsForMilestone(milestone: ArtifactNode): ArtifactNode[] {
		return artifactGraphSDK
			.traverseIncoming(milestone.id, "fulfils")
			.sort((a, b) => a.title.localeCompare(b.title));
	}

	/** Pipeline stages for epics. */
	const epicStages = $derived(stagesFor("epic"));

	/** Epics for the selected milestone, grouped into kanban columns. */
	const epicColumns = $derived.by(() => {
		if (!selectedMilestone) return new Map<string, ArtifactNode[]>();
		return groupByStatus(epicsForMilestone(selectedMilestone), epicStages);
	});

	// -----------------------------------------------------------------------
	// Navigation actions
	// -----------------------------------------------------------------------

	/**
	 * Open the milestone artifact in the editor panel.
	 * @param ms - The milestone node to open.
	 */
	function openMilestone(ms: ArtifactNode) {
		navigationStore.openArtifact(ms.path, []);
	}

	/**
	 * Drill into a milestone's epic kanban board.
	 * @param ms - The milestone to drill into.
	 */
	function drillIntoMilestone(ms: ArtifactNode) {
		selectedMilestone = ms;
	}

	/**
	 * Navigate to an epic artifact by its ID.
	 * @param epic - The epic node to navigate to.
	 */
	function openEpic(epic: ArtifactNode) {
		navigationStore.navigateToArtifact(epic.id);
	}

	/** Return to the top-level milestone kanban. */
	function backToMilestones() {
		selectedMilestone = null;
	}

	// -----------------------------------------------------------------------
	// Style helpers
	// -----------------------------------------------------------------------

	/**
	 * CSS inline style string for a stage column header accent.
	 * Uses the stage color as a left border.
	 * @param color - Hex color string from the stage config.
	 * @returns CSS style attribute value.
	 */
	function columnHeaderStyle(color: string): string {
		return `border-left: 3px solid ${color};`;
	}

	/**
	 * CSS inline style string for a stage column header text color.
	 * @param color - Hex color string from the stage config.
	 * @returns CSS style attribute value.
	 */
	function columnLabelStyle(color: string): string {
		return `color: ${color};`;
	}

	/**
	 * CSS inline style string for a card's status dot.
	 * @param color - Hex color string.
	 * @returns CSS style attribute value.
	 */
	function dotStyle(color: string): string {
		return `background-color: ${color};`;
	}

	/**
	 * Return the color for a given status key within a stage list.
	 * Falls back to a neutral grey when the status is not found.
	 * @param stages - The stage list to search.
	 * @param statusKey - The status key to look up.
	 * @returns Hex color string.
	 */
	function colorForStatus(stages: PipelineStageConfig[], statusKey: string | null): string {
		if (!statusKey) return "#94a3b8";
		return stages.find((s) => s.key === statusKey)?.color ?? "#94a3b8";
	}
</script>

<div class="h-full overflow-y-auto">
	<div class="flex flex-col gap-6 p-6">

		<!-- Breadcrumb / back navigation -->
		<div class="flex items-center gap-2">
			{#if selectedMilestone}
				<button
					class="flex items-center gap-1 rounded px-2 py-1 text-sm text-muted-foreground hover:bg-accent"
					onclick={backToMilestones}
				>
					<Icon name="chevron-left" size="sm" />
					Roadmap
				</button>
				<span class="text-muted-foreground">/</span>
				<span class="text-sm font-medium truncate max-w-xs">{selectedMilestone.title}</span>
			{:else}
				<Heading level={1}>Roadmap</Heading>
			{/if}
		</div>

		{#if !selectedMilestone}
			<!-- ================================================================
			     TOP LEVEL: Milestone kanban board
			     ================================================================ -->
			{#if milestones.length === 0}
				<div class="rounded-md border border-dashed p-8 text-center text-sm text-muted-foreground">
					<Icon name="map" size="xl" />
					<p class="mt-2">No milestones yet.</p>
					<p class="mt-1">Create milestone artifacts in your delivery tree to build a roadmap.</p>
				</div>
			{:else}
				<div class="grid gap-4" style="grid-template-columns: repeat({milestoneStages.length}, minmax(200px, 1fr));">
					{#each milestoneStages as stage (stage.key)}
						{@const cards = milestoneColumns.get(stage.key) ?? []}
						<div class="flex flex-col gap-2">
							<!-- Column header -->
							<div
								class="rounded px-3 py-1.5"
								style={columnHeaderStyle(stage.color)}
							>
								<span class="text-xs font-semibold uppercase tracking-wide" style={columnLabelStyle(stage.color)}>
									{stage.label}
								</span>
								<span class="ml-2 text-xs text-muted-foreground">({cards.length})</span>
							</div>

							<!-- Cards -->
							<div class="flex flex-col gap-2">
								{#each cards as ms (ms.id)}
									{@const statusColor = colorForStatus(milestoneStages, ms.status)}
									{@const deadline = (ms.frontmatter as Record<string, unknown>)?.deadline as string | undefined}
									<div class="flex flex-col gap-1 rounded-md border border-border bg-card p-3">
										<!-- Title row -->
										<div class="flex items-start gap-2">
											<div class="mt-1 h-2 w-2 shrink-0 rounded-full" style={dotStyle(statusColor)}></div>
											<button
												class="h-auto flex-1 text-left text-sm font-medium leading-snug hover:underline"
												onclick={() => drillIntoMilestone(ms)}
											>
												{ms.title}
											</button>
										</div>

										{#if ms.description}
											<p class="text-xs text-muted-foreground line-clamp-2 pl-4">{ms.description}</p>
										{/if}

										<!-- Actions row -->
										<div class="flex items-center justify-between pl-4 pt-0.5">
											{#if deadline}
												<div class="flex items-center gap-1 text-xs text-muted-foreground">
													<Icon name="calendar" size="sm" />
													{deadline}
												</div>
											{:else}
												<span></span>
											{/if}
											<button
												class="flex h-auto items-center p-0 text-xs text-muted-foreground hover:text-foreground"
												onclick={() => openMilestone(ms)}
												title="Open artifact"
											>
												<Icon name="external-link" size="sm" />
											</button>
										</div>
									</div>
								{/each}

								{#if cards.length === 0}
									<div class="rounded-md border border-dashed border-border p-3 text-center text-xs text-muted-foreground">
										No milestones
									</div>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			{/if}

		{:else}
			<!-- ================================================================
			     DRILL-DOWN: Epic kanban board for selected milestone
			     ================================================================ -->
			{@const allEpics = epicsForMilestone(selectedMilestone)}

			{#if allEpics.length === 0}
				<div class="rounded-md border border-dashed p-8 text-center text-sm text-muted-foreground">
					<Icon name="layers" size="xl" />
					<p class="mt-2">No epics linked to this milestone yet.</p>
					<p class="mt-1">Create epic artifacts with a "fulfils" relationship to <strong>{selectedMilestone.title}</strong>.</p>
				</div>
			{:else}
				<div class="grid gap-4" style="grid-template-columns: repeat({epicStages.length}, minmax(200px, 1fr));">
					{#each epicStages as stage (stage.key)}
						{@const cards = epicColumns.get(stage.key) ?? []}
						<div class="flex flex-col gap-2">
							<!-- Column header -->
							<div
								class="rounded px-3 py-1.5"
								style={columnHeaderStyle(stage.color)}
							>
								<span class="text-xs font-semibold uppercase tracking-wide" style={columnLabelStyle(stage.color)}>
									{stage.label}
								</span>
								<span class="ml-2 text-xs text-muted-foreground">({cards.length})</span>
							</div>

							<!-- Cards -->
							<div class="flex flex-col gap-2">
								{#each cards as epic (epic.id)}
									{@const statusColor = colorForStatus(epicStages, epic.status)}
									<div class="flex flex-col gap-1 rounded-md border border-border bg-card p-3">
										<!-- Title row -->
										<div class="flex items-start gap-2">
											<div class="mt-1 h-2 w-2 shrink-0 rounded-full" style={dotStyle(statusColor)}></div>
											<button
												class="h-auto flex-1 text-left text-sm font-medium leading-snug hover:underline"
												onclick={() => openEpic(epic)}
											>
												{epic.title}
											</button>
										</div>

										{#if epic.description}
											<p class="text-xs text-muted-foreground line-clamp-2 pl-4">{epic.description}</p>
										{/if}

										<!-- Priority badge -->
										{#if epic.priority}
											<div class="pl-4">
												<Badge variant="secondary">{epic.priority}</Badge>
											</div>
										{/if}
									</div>
								{/each}

								{#if cards.length === 0}
									<div class="rounded-md border border-dashed border-border p-3 text-center text-xs text-muted-foreground">
										No epics
									</div>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}

	</div>
</div>
