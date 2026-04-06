<script lang="ts">
	import {
		Icon,
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		CardAction,
		Stack,
		HStack,
		Caption,
		Text,
		Button,
		Box,
		ProgressBar,
		ScrollArea,
	} from "@orqastudio/svelte-components/pure";
	import { Panel } from "@orqastudio/svelte-components/pure";

	import { ArtifactLink } from "@orqastudio/svelte-components/connected";
	import { SvelteMap } from "svelte/reactivity";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, navigationStore, pluginRegistry, projectStore } = getStores();
	import type { ArtifactNode } from "@orqastudio/types";

	// -------------------------------------------------------------------------
	// Tab state
	// -------------------------------------------------------------------------

	type TabKey = "actions" | "epics";
	let activeTab = $state<TabKey>("actions");

	// -------------------------------------------------------------------------
	// Pending actions — all artifacts with status: review
	// -------------------------------------------------------------------------

	interface PendingAction {
		id: string;
		title: string;
		artifactType: string;
		action: string;
		path: string;
		priority: string | null;
	}

	/**
	 * Returns the review action label for an artifact type from its schema.
	 * Falls back to "Review required" when the schema has no reviewAction.
	 * @param type - The artifact type key.
	 * @returns The action label string for the type.
	 */
	function actionLabel(type: string): string {
		return pluginRegistry.getSchema(type)?.reviewAction ?? "Review required";
	}

	const pendingActions = $derived.by((): PendingAction[] => {
		return artifactGraphSDK.byStatus("review").map((node) => ({
			id: node.id,
			title: node.title,
			artifactType: node.artifact_type,
			action: actionLabel(node.artifact_type),
			path: node.path,
			priority: node.priority,
		}));
	});

	// -------------------------------------------------------------------------
	// Epics tab — in-progress and next-priority
	// -------------------------------------------------------------------------

	interface EpicEntry {
		id: string;
		title: string;
		description: string | null;
		status: string;
		priority: string | null;
		path: string;
		taskProgress: number | null;
		taskDone: number;
		taskTotal: number;
	}

	/**
	 * Maps a priority band to a numeric sort rank where lower is higher priority.
	 * @param p - The priority string (e.g. "P1", "P2") or null for unset.
	 * @returns A numeric rank; unset priorities rank last.
	 */
	function priorityRank(p: string | null): number {
		if (p === "P1") return 0;
		if (p === "P2") return 1;
		if (p === "P3") return 2;
		return 3;
	}

	/** Status sort index derived from the project status order. Lower index = higher priority. */
	const statusOrder = $derived(
		Object.fromEntries((projectStore.projectSettings?.statuses ?? []).map((s, i) => [s.key, i])),
	);

	const epicEntries = $derived.by((): EpicEntry[] => {
		const entries: EpicEntry[] = [];

		// Pre-index tasks by epic reference (frontmatter `epic` field)
		const tasksByEpic = new SvelteMap<string, ArtifactNode[]>();
		for (const task of artifactGraphSDK.byType("task")) {
			const fm = task.frontmatter as Record<string, unknown>;
			const epicId = typeof fm.epic === "string" ? fm.epic : null;
			if (!epicId) continue;
			const existing = tasksByEpic.get(epicId) ?? [];
			existing.push(task);
			tasksByEpic.set(epicId, existing);
		}

		for (const node of artifactGraphSDK.byType("epic")) {
			const status = node.status ?? "";
			if (status !== "active" && status !== "ready" && status !== "prioritised") continue;

			const tasks = tasksByEpic.get(node.id) ?? [];
			const taskTotal = tasks.length;
			const taskDone = tasks.filter((t) => t.status === "completed").length;
			const taskProgress = taskTotal > 0 ? taskDone / taskTotal : null;

			entries.push({
				id: node.id,
				title: node.title,
				description: node.description ?? null,
				status,
				priority: node.priority,
				path: node.path,
				taskProgress,
				taskDone,
				taskTotal,
			});
		}

		// active first, then ready; within each group sort by priority
		return entries.sort((a, b) => {
			const sa = statusOrder[a.status] ?? 99;
			const sb = statusOrder[b.status] ?? 99;
			if (sa !== sb) return sa - sb;
			return priorityRank(a.priority) - priorityRank(b.priority);
		});
	});

	// -------------------------------------------------------------------------
	// General state
	// -------------------------------------------------------------------------

	const hasData = $derived(artifactGraphSDK.graph.size > 0);

	// -------------------------------------------------------------------------
	// Navigation
	// -------------------------------------------------------------------------

	/** Navigate to the roadmap activity view. */
	function openRoadmap() {
		navigationStore.setActivity("roadmap");
	}
</script>

{#if hasData}
	<CardRoot>
		<CardHeader compact>
			<CardTitle>
				<HStack gap={1}>
					<Icon name="compass" size="md" />
					Purpose
				</HStack>
			</CardTitle>
			<CardDescription>What's Next</CardDescription>
			<!-- Tab buttons in Card.Action — ghost variant, secondary when active -->
			<CardAction>
				<HStack gap={0}>
					<Button
						variant={activeTab === "actions" ? "secondary" : "ghost"}
						size="sm"
						onclick={() => (activeTab = "actions")}
					>
						Actions
						{#if pendingActions.length > 0}
							<Caption variant="caption-tabular">{pendingActions.length}</Caption>
						{/if}
					</Button>
					<Button
						variant={activeTab === "epics" ? "secondary" : "ghost"}
						size="sm"
						onclick={() => (activeTab = "epics")}
					>
						Epics
					</Button>
				</HStack>
			</CardAction>
		</CardHeader>
		<CardContent>
			<!-- Fixed-height scroll area for the queue content — 280px matches card layout budget -->
			<ScrollArea heightPx={280}>
				{#if activeTab === "actions"}
					<!-- ---------------------------------------------------------- -->
					<!-- Actions tab: all artifacts needing attention               -->
					<!-- ---------------------------------------------------------- -->
					{#if pendingActions.length === 0}
						<Panel padding="normal">
							<HStack gap={2}>
								<Icon name="check-circle-2" size="md" />
								<Text variant="body-muted">No pending actions — everything is moving</Text>
							</HStack>
						</Panel>
					{:else}
						<Stack gap={1}>
							{#each pendingActions as action (action.id)}
								<Panel padding="tight">
									<HStack gap={2}>
										<Stack gap={0} flex={1}>
											<Text variant="caption-strong" truncate>{action.action}</Text>
											<Caption truncate>{action.title}</Caption>
										</Stack>
										<Box flex={0}>
											<ArtifactLink id={action.id} displayLabel={action.id} />
										</Box>
									</HStack>
								</Panel>
							{/each}
						</Stack>
					{/if}
				{:else}
					<!-- ---------------------------------------------------------- -->
					<!-- Epics tab: in-progress + next ready epics                  -->
					<!-- ---------------------------------------------------------- -->
					{#if epicEntries.length === 0}
						<Panel padding="normal">
							<HStack gap={2}>
								<Icon name="map" size="md" />
								<Text variant="body-muted">No active or ready epics</Text>
							</HStack>
						</Panel>
					{:else}
						<Stack gap={1}>
							{#each epicEntries as epic (epic.id)}
								<Panel padding="tight">
									<HStack gap={2}>
										<Stack gap={1} flex={1}>
											<Text variant="caption-strong" truncate>{epic.title}</Text>
											{#if epic.description}
												<Caption truncate>{epic.description}</Caption>
											{/if}
											{#if epic.taskProgress !== null}
												<HStack gap={1} align="center">
													<ProgressBar mini ratio={epic.taskProgress} />
													<Caption variant="caption-tabular"
														>{epic.taskDone}/{epic.taskTotal}</Caption
													>
												</HStack>
											{/if}
										</Stack>
										<Box flex={0}>
											<ArtifactLink id={epic.id} displayLabel={epic.id} />
										</Box>
									</HStack>
								</Panel>
							{/each}
						</Stack>

						<Button variant="ghost" size="sm" onclick={openRoadmap}>View roadmap</Button>
					{/if}
				{/if}
			</ScrollArea>
		</CardContent>
	</CardRoot>
{/if}
