<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import KanbanIcon from "@lucide/svelte/icons/kanban";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import StatusIndicator from "$lib/components/shared/StatusIndicator.svelte";
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import type { ArtifactNode } from "$lib/types/artifact-graph";

	// ---------------------------------------------------------------------------
	// Data from graph
	// ---------------------------------------------------------------------------

	const milestones = $derived(artifactGraphSDK.byType("milestone"));
	const epics = $derived(artifactGraphSDK.byType("epic"));
	const tasks = $derived(artifactGraphSDK.byType("task"));
	const graphLoading = $derived(artifactGraphSDK.loading);
	const graphError = $derived(artifactGraphSDK.error);
	const hasData = $derived(milestones.length > 0);

	// ---------------------------------------------------------------------------
	// Milestone columns sorted by status: planning -> active -> complete
	// ---------------------------------------------------------------------------

	const STATUS_ORDER: Record<string, number> = {
		planning: 0,
		active: 1,
		complete: 2,
	};

	/** Epics not assigned to any known milestone. */
	const unassignedEpicsList = $derived.by(() => {
		return epics
			.filter((e) => {
				const ms = getEpicMilestone(e);
				return !ms || !milestones.some((m) => m.id === ms);
			})
			.sort((a, b) => (a.priority ?? "P3").localeCompare(b.priority ?? "P3"));
	});

	const sortedMilestones = $derived.by(() => {
		return [...milestones].sort((a, b) => {
			const orderA = STATUS_ORDER[a.status ?? "planning"] ?? 0;
			const orderB = STATUS_ORDER[b.status ?? "planning"] ?? 0;
			return orderA - orderB;
		});
	});

	// ---------------------------------------------------------------------------
	// Epics grouped by milestone
	// ---------------------------------------------------------------------------

	/** Resolve the milestone field from an epic's frontmatter. */
	function getEpicMilestone(epic: ArtifactNode): string | null {
		const fm = epic.frontmatter;
		if (typeof fm.milestone === "string") return fm.milestone;
		return null;
	}

	/** Get epics for a given milestone, sorted by priority (P1 first). */
	function epicsForMilestone(milestoneId: string): ArtifactNode[] {
		return epics
			.filter((e) => getEpicMilestone(e) === milestoneId)
			.sort((a, b) => {
				const pa = a.priority ?? "P3";
				const pb = b.priority ?? "P3";
				return pa.localeCompare(pb);
			});
	}

	/** Count tasks for an epic: { done, total }. */
	function taskCountForEpic(epicId: string): { done: number; total: number } {
		const epicTasks = tasks.filter((t) => {
			const fm = t.frontmatter;
			return fm.epic === epicId;
		});
		const done = epicTasks.filter((t) => t.status === "done").length;
		return { done, total: epicTasks.length };
	}

	/** Priority badge variant mapping. */
	function priorityVariant(priority: string | null): "default" | "secondary" | "destructive" {
		if (priority === "P1") return "destructive";
		if (priority === "P2") return "default";
		return "secondary";
	}

	function handleEpicClick(epicId: string) {
		navigationStore.navigateToArtifact(epicId);
	}
</script>

<ScrollArea.Root class="h-full">
	<div class="p-6">
		<!-- Header -->
		<div class="mb-6">
			<div class="flex items-center gap-3">
				<KanbanIcon class="h-8 w-8 text-muted-foreground" />
				<div>
					<h1 class="text-2xl font-bold">Roadmap</h1>
					<p class="text-sm text-muted-foreground">
						Milestones as columns, epics as cards. Click an epic to view details.
					</p>
				</div>
			</div>
		</div>

		<!-- States: loading, error, empty, loaded -->
		{#if graphLoading && !hasData}
			<div class="flex items-center justify-center py-12">
				<LoadingSpinner />
			</div>
		{:else if graphError && !hasData}
			<ErrorDisplay
				message={graphError}
				onRetry={() => artifactGraphSDK.refresh()}
			/>
		{:else if !hasData}
			<EmptyState
				icon={KanbanIcon}
				title="No milestones found"
				description="Create milestones in .orqa/delivery/milestones/ to see them here."
			/>
		{:else}
			<!-- Kanban board: columns expand to fill available space, scroll if too many -->
			<div class="flex gap-4 overflow-x-auto pb-4">
				{#each sortedMilestones as milestone (milestone.id)}
					{@const msEpics = epicsForMilestone(milestone.id)}
					<div class="flex min-w-64 flex-1 flex-col">
						<!-- Column header -->
						<Card.Root class="mb-3">
							<Card.Header class="pb-2">
								<div class="flex items-center justify-between">
									<Card.Title class="text-sm font-semibold">
										{milestone.title}
									</Card.Title>
									<StatusIndicator status={milestone.status ?? "planning"} mode="badge" />
								</div>
								{#if milestone.description}
									<p class="text-xs text-muted-foreground">{milestone.description}</p>
								{/if}
							</Card.Header>
							<Card.Content class="pt-0">
								<span class="text-xs text-muted-foreground">
									{msEpics.length} epic{msEpics.length === 1 ? "" : "s"}
								</span>
							</Card.Content>
						</Card.Root>

						<!-- Epic cards -->
						<div class="flex flex-col gap-2">
							{#each msEpics as epic (epic.id)}
								{@const tc = taskCountForEpic(epic.id)}
								<button
									class="w-full rounded-lg border border-border bg-card p-3 text-left transition-colors hover:bg-accent/50"
									onclick={() => handleEpicClick(epic.id)}
								>
									<div class="flex items-start justify-between gap-2">
										<div class="flex items-center gap-2 min-w-0">
											<StatusIndicator status={epic.status ?? "draft"} mode="dot" />
											<span class="truncate text-sm font-medium">{epic.title}</span>
										</div>
										{#if epic.priority}
											<SmallBadge variant={priorityVariant(epic.priority)}>
												{epic.priority}
											</SmallBadge>
										{/if}
									</div>
									{#if epic.description}
										<p class="mt-1.5 line-clamp-2 text-xs text-muted-foreground">
											{epic.description}
										</p>
									{/if}
									{#if tc.total > 0}
										<div class="mt-2 flex items-center gap-2">
											<div class="h-1.5 flex-1 rounded-full bg-muted">
												<div
													class="h-1.5 rounded-full bg-emerald-500 transition-all"
													style="width: {tc.total > 0 ? (tc.done / tc.total) * 100 : 0}%"
												></div>
											</div>
											<span class="shrink-0 text-[10px] tabular-nums text-muted-foreground">
												{tc.done}/{tc.total}
											</span>
										</div>
									{/if}
								</button>
							{:else}
								<div class="rounded-lg border border-dashed border-border p-3 text-center text-xs text-muted-foreground">
									No epics
								</div>
							{/each}
						</div>
					</div>
				{/each}

				<!-- Unassigned epics column -->
				{#if unassignedEpicsList.length > 0}
					<div class="flex min-w-64 flex-1 flex-col">
						<Card.Root class="mb-3">
							<Card.Header class="pb-2">
								<Card.Title class="text-sm font-semibold text-muted-foreground">
									Unassigned
								</Card.Title>
							</Card.Header>
							<Card.Content class="pt-0">
								<span class="text-xs text-muted-foreground">
									{unassignedEpicsList.length} epic{unassignedEpicsList.length === 1 ? "" : "s"}
								</span>
							</Card.Content>
						</Card.Root>

						<div class="flex flex-col gap-2">
							{#each unassignedEpicsList as epic (epic.id)}
								{@const tc = taskCountForEpic(epic.id)}
								<button
									class="w-full rounded-lg border border-border bg-card p-3 text-left transition-colors hover:bg-accent/50"
									onclick={() => handleEpicClick(epic.id)}
								>
									<div class="flex items-start justify-between gap-2">
										<div class="flex items-center gap-2 min-w-0">
											<StatusIndicator status={epic.status ?? "draft"} mode="dot" />
											<span class="truncate text-sm font-medium">{epic.title}</span>
										</div>
										{#if epic.priority}
											<SmallBadge variant={priorityVariant(epic.priority)}>
												{epic.priority}
											</SmallBadge>
										{/if}
									</div>
									{#if epic.description}
										<p class="mt-1.5 line-clamp-2 text-xs text-muted-foreground">
											{epic.description}
										</p>
									{/if}
									{#if tc.total > 0}
										<div class="mt-2 flex items-center gap-2">
											<div class="h-1.5 flex-1 rounded-full bg-muted">
												<div
													class="h-1.5 rounded-full bg-emerald-500 transition-all"
													style="width: {tc.total > 0 ? (tc.done / tc.total) * 100 : 0}%"
												></div>
											</div>
											<span class="shrink-0 text-[10px] tabular-nums text-muted-foreground">
												{tc.done}/{tc.total}
											</span>
										</div>
									{/if}
								</button>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</ScrollArea.Root>
