<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardAction } from "@orqastudio/svelte-components/pure";

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
		Object.fromEntries((projectStore.projectSettings?.statuses ?? []).map((s, i) => [s.key, i]))
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
			if (
				status !== "active" &&
				status !== "ready" &&
				status !== "prioritised"
			) continue;

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
				<div class="flex items-center gap-1">
					<Icon name="compass" size="md" />
					Purpose
				</div>
			</CardTitle>
			<CardDescription>What's Next</CardDescription>
			<!-- Tab buttons in Card.Action -->
			<CardAction>
				<div class="flex items-center gap-0">
					<button
						class="rounded-none border-b-2 px-2 py-1 text-xs {activeTab === 'actions' ? 'border-foreground font-medium text-foreground' : 'border-transparent text-muted-foreground'} hover:bg-accent"
						onclick={() => (activeTab = "actions")}
					>
						Actions
						{#if pendingActions.length > 0}
							<span class="ml-1 text-[10px] tabular-nums {activeTab === 'actions' ? 'text-foreground' : 'text-muted-foreground'}">
								{pendingActions.length}
							</span>
						{/if}
					</button>
					<button
						class="rounded-none border-b-2 px-2 py-1 text-xs {activeTab === 'epics' ? 'border-foreground font-medium text-foreground' : 'border-transparent text-muted-foreground'} hover:bg-accent"
						onclick={() => (activeTab = "epics")}
					>
						Epics
					</button>
				</div>
			</CardAction>
		</CardHeader>
		<CardContent>
			<div class="h-[280px] overflow-y-auto px-3 pb-3">
			{#if activeTab === "actions"}
				<!-- ---------------------------------------------------------- -->
				<!-- Actions tab: all artifacts needing attention               -->
				<!-- ---------------------------------------------------------- -->
				{#if pendingActions.length === 0}
					<div class="flex items-center gap-2 py-4 text-sm text-muted-foreground">
						<Icon name="check-circle-2" size="md" />
						<span>No pending actions — everything is moving</span>
					</div>
				{:else}
					<div class="space-y-1">
						{#each pendingActions as action (action.id)}
							<div class="flex w-full items-center justify-between gap-2 rounded-md px-2 py-1.5 transition-colors hover:bg-accent/50">
								<div class="min-w-0 flex-1">
									<p class="truncate text-xs font-medium">{action.action}</p>
									<p class="truncate text-[10px] text-muted-foreground">{action.title}</p>
								</div>
								<div class="shrink-0">
									<ArtifactLink id={action.id} displayLabel={action.id} />
								</div>
							</div>
						{/each}
					</div>
				{/if}
			{:else}
				<!-- ---------------------------------------------------------- -->
				<!-- Epics tab: in-progress + next ready epics                  -->
				<!-- ---------------------------------------------------------- -->
				{#if epicEntries.length === 0}
					<div class="flex items-center gap-2 py-4 text-sm text-muted-foreground">
						<Icon name="map" size="md" />
						<span>No active or ready epics</span>
					</div>
				{:else}
					<div class="space-y-1">
						{#each epicEntries as epic (epic.id)}
							<div class="flex w-full items-center justify-between gap-2 rounded-md px-2 py-1.5 transition-colors hover:bg-accent/50">
								<div class="min-w-0 flex-1">
									<p class="truncate text-xs font-medium">{epic.title}</p>
									{#if epic.description}
										<p class="truncate text-[10px] text-muted-foreground">{epic.description}</p>
									{/if}
									{#if epic.taskProgress !== null}
										<div class="mt-0.5 flex items-center gap-1">
											<div class="h-1 flex-1 rounded-full bg-muted overflow-hidden">
												<div
													class="h-full rounded-full bg-success transition-all"
													style:width="{Math.round(epic.taskProgress * 100)}%"
												></div>
											</div>
											<span class="text-[10px] text-muted-foreground tabular-nums shrink-0">
												{epic.taskDone}/{epic.taskTotal}
											</span>
										</div>
									{/if}
								</div>
								<div class="shrink-0">
									<ArtifactLink id={epic.id} displayLabel={epic.id} />
								</div>
							</div>
						{/each}
					</div>

					<button
						class="mt-2 w-full rounded px-2 py-1 text-center text-xs text-muted-foreground underline underline-offset-2 hover:bg-accent"
						onclick={openRoadmap}
					>
						View roadmap
					</button>
				{/if}
			{/if}
			</div>
		</CardContent>
	</CardRoot>
{/if}
