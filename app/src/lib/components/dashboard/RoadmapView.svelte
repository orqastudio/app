<!-- Roadmap view — kanban board of milestones grouped by status.
     Clicking a milestone card drills into a kanban of its epics grouped by status.
     Breadcrumb navigation and a back button return to the milestone board. -->
<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import {
		Icon,
		Heading,
		Badge,
		Button,
		HStack,
		Stack,
		Text,
		Caption,
	} from "@orqastudio/svelte-components/pure";
	import { Panel, ScrollArea } from "@orqastudio/svelte-components/pure";
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
		const map = new SvelteMap<string, ArtifactNode[]>();
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
		return artifactGraphSDK.byType("milestone").sort((a, b) => a.title.localeCompare(b.title));
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

<ScrollArea full>
	<Panel padding="loose">
		<Stack gap={6}>
			<!-- Breadcrumb / back navigation -->
			<HStack gap={2}>
				{#if selectedMilestone}
					<Button variant="ghost" size="sm" onclick={backToMilestones}>
						<Icon name="chevron-left" size="sm" />
						Roadmap
					</Button>
					<Caption>/</Caption>
					<Text variant="body-strong" truncate>{selectedMilestone.title}</Text>
				{:else}
					<Heading level={1}>Roadmap</Heading>
				{/if}
			</HStack>

			{#if !selectedMilestone}
				<!-- ================================================================
			     TOP LEVEL: Milestone kanban board
			     ================================================================ -->
				{#if milestones.length === 0}
					<Panel padding="loose">
						<Stack gap={2} align="center">
							<Icon name="map" size="xl" />
							<Text variant="body-muted" block>No milestones yet.</Text>
							<Text variant="body-muted" block
								>Create milestone artifacts in your delivery tree to build a roadmap.</Text
							>
						</Stack>
					</Panel>
				{:else}
					<!-- Dynamic-column kanban grid — column count from plugin registry; inline style required -->
					<div
						class="grid gap-4"
						style="grid-template-columns: repeat({milestoneStages.length}, minmax(200px, 1fr));"
					>
						{#each milestoneStages as stage (stage.key)}
							{@const cards = milestoneColumns.get(stage.key) ?? []}
							<Stack gap={2}>
								<!-- Column header — stage color border and text color are dynamic from plugin registry; inline style required on the span wrapper, not on HStack -->
								<!-- FOLLOW-UP: HStack style prop removed; border-left dynamic color moved to outer span wrapper; see findings -->
								<span style={columnHeaderStyle(stage.color)}>
									<HStack>
										<!-- Stage label with dynamic color — Text has no style prop; span wrapper used -->
										<span style={columnLabelStyle(stage.color)}
											><Text variant="overline">{stage.label}</Text></span
										>
										<Caption>({cards.length})</Caption>
									</HStack>
								</span>

								<!-- Cards -->
								<Stack gap={2}>
									{#each cards as ms (ms.id)}
										{@const statusColor = colorForStatus(milestoneStages, ms.status)}
										{@const deadline = (ms.frontmatter as Record<string, unknown>)?.deadline as
											| string
											| undefined}
										<Panel padding="normal" border="all" rounded="md" background="card">
											<Stack gap={1}>
												<!-- Title row -->
												<HStack gap={2} align="start">
													<!-- Status dot with inline color from plugin manifest -->
													<span
														aria-hidden="true"
														style="display: inline-block; margin-top: 0.25rem; height: 0.5rem; width: 0.5rem; flex-shrink: 0; border-radius: 9999px; {dotStyle(
															statusColor,
														)}"
													></span>
													<Button variant="ghost" size="sm" onclick={() => drillIntoMilestone(ms)}>
														{ms.title}
													</Button>
												</HStack>

												{#if ms.description}
													<Caption lineClamp={2}>{ms.description}</Caption>
												{/if}

												<!-- Actions row -->
												<HStack justify="between">
													{#if deadline}
														<HStack gap={1}>
															<Icon name="calendar" size="sm" />
															<Caption>{deadline}</Caption>
														</HStack>
													{:else}
														<span></span>
													{/if}
													<Button variant="ghost" size="sm" onclick={() => openMilestone(ms)}>
														<Icon name="external-link" size="sm" />
													</Button>
												</HStack>
											</Stack>
										</Panel>
									{/each}

									{#if cards.length === 0}
										<Panel padding="normal" border="all" rounded="md">
											<Caption>No milestones</Caption>
										</Panel>
									{/if}
								</Stack>
							</Stack>
						{/each}
					</div>
				{/if}
			{:else}
				<!-- ================================================================
			     DRILL-DOWN: Epic kanban board for selected milestone
			     ================================================================ -->
				{@const allEpics = epicsForMilestone(selectedMilestone)}

				{#if allEpics.length === 0}
					<Panel padding="loose">
						<Stack gap={2} align="center">
							<Icon name="layers" size="xl" />
							<Text variant="body-muted" block>No epics linked to this milestone yet.</Text>
							<Text variant="body-muted" block
								>Create epic artifacts with a "fulfils" relationship to <strong
									>{selectedMilestone.title}</strong
								>.</Text
							>
						</Stack>
					</Panel>
				{:else}
					<!-- Dynamic-column kanban grid — column count from plugin registry; inline style required -->
					<div
						class="grid gap-4"
						style="grid-template-columns: repeat({epicStages.length}, minmax(200px, 1fr));"
					>
						{#each epicStages as stage (stage.key)}
							{@const cards = epicColumns.get(stage.key) ?? []}
							<Stack gap={2}>
								<!-- Column header — stage color border and text color are dynamic from plugin registry; inline style required on the span wrapper, not on HStack -->
								<!-- FOLLOW-UP: HStack style prop removed; border-left dynamic color moved to outer span wrapper; see findings -->
								<span style={columnHeaderStyle(stage.color)}>
									<HStack>
										<!-- Stage label with dynamic color — Text has no style prop; span wrapper used -->
										<span style={columnLabelStyle(stage.color)}
											><Text variant="overline">{stage.label}</Text></span
										>
										<Caption>({cards.length})</Caption>
									</HStack>
								</span>

								<!-- Cards -->
								<Stack gap={2}>
									{#each cards as epic (epic.id)}
										{@const statusColor = colorForStatus(epicStages, epic.status)}
										<Panel padding="normal" border="all" rounded="md" background="card">
											<Stack gap={1}>
												<!-- Title row -->
												<HStack gap={2} align="start">
													<!-- Status dot with inline color from plugin manifest -->
													<span
														aria-hidden="true"
														style="display: inline-block; margin-top: 0.25rem; height: 0.5rem; width: 0.5rem; flex-shrink: 0; border-radius: 9999px; {dotStyle(
															statusColor,
														)}"
													></span>
													<Button variant="ghost" size="sm" onclick={() => openEpic(epic)}>
														{epic.title}
													</Button>
												</HStack>

												{#if epic.description}
													<Caption lineClamp={2}>{epic.description}</Caption>
												{/if}

												<!-- Priority badge -->
												{#if epic.priority}
													<Badge variant="secondary">{epic.priority}</Badge>
												{/if}
											</Stack>
										</Panel>
									{/each}

									{#if cards.length === 0}
										<Panel padding="normal" border="all" rounded="md">
											<Caption>No epics</Caption>
										</Panel>
									{/if}
								</Stack>
							</Stack>
						{/each}
					</div>
				{/if}
			{/if}
		</Stack>
	</Panel>
</ScrollArea>
