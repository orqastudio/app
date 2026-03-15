<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { Badge } from "$lib/components/ui/badge";
	import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
	import CompassIcon from "@lucide/svelte/icons/compass";
	import MapIcon from "@lucide/svelte/icons/map";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import type { ArtifactNode } from "$lib/types/artifact-graph";

	// -------------------------------------------------------------------------
	// Tab state
	// -------------------------------------------------------------------------

	type TabKey = "actions" | "epics";
	let activeTab = $state<TabKey>("actions");

	// -------------------------------------------------------------------------
	// Pending actions — all artifacts that need human attention
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
	 * Return true if a task has a depends-on list with any unfinished dependency.
	 * The depends-on field is an array of task IDs in frontmatter.
	 */
	function hasUnmetDependencies(node: ArtifactNode): boolean {
		const fm = node.frontmatter as Record<string, unknown>;
		const dependsOn = fm["depends-on"];
		if (!Array.isArray(dependsOn) || dependsOn.length === 0) return false;
		for (const depId of dependsOn) {
			if (typeof depId !== "string") continue;
			const dep = artifactGraphSDK.resolve(depId);
			if (!dep || dep.status !== "done") return true;
		}
		return false;
	}

	const pendingActions = $derived.by((): PendingAction[] => {
		const actions: PendingAction[] = [];

		// Tasks: todo or in-progress with unmet depends-on
		for (const node of artifactGraphSDK.byType("task")) {
			const status = node.status ?? "";
			if (status === "todo" || status === "in-progress") {
				if (hasUnmetDependencies(node)) {
					actions.push({
						id: node.id,
						title: node.title,
						artifactType: "task",
						action: "Blocked — dependency not done",
						path: node.path,
						priority: node.priority,
					});
				}
			}
		}

		// Epics: draft or ready needing attention
		for (const node of artifactGraphSDK.byType("epic")) {
			const status = node.status ?? "";
			if (status === "draft") {
				actions.push({
					id: node.id,
					title: node.title,
					artifactType: "epic",
					action: "Draft — needs docs-required gate",
					path: node.path,
					priority: node.priority,
				});
			} else if (status === "ready") {
				actions.push({
					id: node.id,
					title: node.title,
					artifactType: "epic",
					action: "Ready — awaiting implementation start",
					path: node.path,
					priority: node.priority,
				});
			}
		}

		// Ideas: exploring or shaped awaiting promotion
		for (const node of artifactGraphSDK.byType("idea")) {
			const status = node.status ?? "";
			if (status === "exploring") {
				actions.push({
					id: node.id,
					title: node.title,
					artifactType: "idea",
					action: "Exploring — research in progress",
					path: node.path,
					priority: node.priority,
				});
			} else if (status === "shaped") {
				actions.push({
					id: node.id,
					title: node.title,
					artifactType: "idea",
					action: "Shaped — ready to promote to epic",
					path: node.path,
					priority: node.priority,
				});
			}
		}

		// Decisions: proposed awaiting acceptance
		for (const node of artifactGraphSDK.byType("decision")) {
			if (node.status === "proposed") {
				actions.push({
					id: node.id,
					title: node.title,
					artifactType: "decision",
					action: "Proposed — awaiting acceptance",
					path: node.path,
					priority: node.priority,
				});
			}
		}

		// Lessons: recurring awaiting promotion to rule/skill
		for (const node of artifactGraphSDK.byType("lesson")) {
			if (node.status === "recurring") {
				actions.push({
					id: node.id,
					title: node.title,
					artifactType: "lesson",
					action: "Recurring — promote to rule or skill",
					path: node.path,
					priority: node.priority,
				});
			}
		}

		// Sort: shaped ideas first (most actionable), then by type priority
		const typeOrder: Record<string, number> = {
			idea: 0,
			decision: 1,
			lesson: 2,
			epic: 3,
			task: 4,
		};
		return actions.sort((a, b) => (typeOrder[a.artifactType] ?? 9) - (typeOrder[b.artifactType] ?? 9));
	});

	// -------------------------------------------------------------------------
	// Epics tab — in-progress and next-priority
	// -------------------------------------------------------------------------

	interface EpicEntry {
		id: string;
		title: string;
		status: string;
		priority: string | null;
		path: string;
		/** Fraction of done tasks out of total tasks linked to this epic (0–1). */
		taskProgress: number | null;
		taskDone: number;
		taskTotal: number;
	}

	/** Priority band → sort rank (lower = higher priority). */
	function priorityRank(p: string | null): number {
		if (p === "P1") return 0;
		if (p === "P2") return 1;
		if (p === "P3") return 2;
		return 3;
	}

	const epicEntries = $derived.by((): EpicEntry[] => {
		const entries: EpicEntry[] = [];

		// Pre-index tasks by epic reference (frontmatter `epic` field)
		const tasksByEpic = new Map<string, ArtifactNode[]>();
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
			if (status !== "in-progress" && status !== "ready") continue;

			const tasks = tasksByEpic.get(node.id) ?? [];
			const taskTotal = tasks.length;
			const taskDone = tasks.filter((t) => t.status === "done").length;
			const taskProgress = taskTotal > 0 ? taskDone / taskTotal : null;

			entries.push({
				id: node.id,
				title: node.title,
				status,
				priority: node.priority,
				path: node.path,
				taskProgress,
				taskDone,
				taskTotal,
			});
		}

		// in-progress first, then ready; within each group sort by priority
		return entries.sort((a, b) => {
			const statusOrder: Record<string, number> = { "in-progress": 0, ready: 1 };
			const sa = statusOrder[a.status] ?? 2;
			const sb = statusOrder[b.status] ?? 2;
			if (sa !== sb) return sa - sb;
			return priorityRank(a.priority) - priorityRank(b.priority);
		});
	});

	// -------------------------------------------------------------------------
	// Type badge colours
	// -------------------------------------------------------------------------

	function typeBadgeVariant(type: string): "default" | "secondary" | "destructive" | "outline" {
		switch (type) {
			case "decision": return "destructive";
			case "idea":     return "default";
			case "lesson":   return "secondary";
			default:         return "outline";
		}
	}

	function typeBadgeLabel(type: string): string {
		const labels: Record<string, string> = {
			task: "Task", epic: "Epic", idea: "Idea",
			decision: "Decision", lesson: "Lesson",
		};
		return labels[type] ?? type;
	}

	function priorityBadgeClass(p: string | null): string {
		if (p === "P1") return "text-destructive";
		if (p === "P2") return "text-amber-600 dark:text-amber-400";
		return "text-muted-foreground";
	}

	// -------------------------------------------------------------------------
	// General state
	// -------------------------------------------------------------------------

	const hasData = $derived(artifactGraphSDK.graph.size > 0);

	// -------------------------------------------------------------------------
	// Navigation
	// -------------------------------------------------------------------------

	function openArtifact(path: string) {
		navigationStore.navigateToPath(path);
	}

	function openRoadmap() {
		navigationStore.setActivity("roadmap");
	}
</script>

{#if hasData}
	<Card.Root class="gap-2">
		<Card.Header class="pb-1">
			<Card.Title class="flex items-center gap-1.5 text-sm font-semibold">
				<CompassIcon class="h-4 w-4 text-muted-foreground" />
				What's Next
			</Card.Title>
			<!-- Tab buttons in Card.Action -->
			<Card.Action>
				<div class="flex items-center gap-0">
					<button
						class="px-2 py-1 text-xs transition-colors border-b-2 {activeTab === 'actions' ? 'border-foreground text-foreground font-medium' : 'border-transparent text-muted-foreground hover:text-foreground'}"
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
						class="px-2 py-1 text-xs transition-colors border-b-2 {activeTab === 'epics' ? 'border-foreground text-foreground font-medium' : 'border-transparent text-muted-foreground hover:text-foreground'}"
						onclick={() => (activeTab = "epics")}
					>
						Epics
					</button>
				</div>
			</Card.Action>
		</Card.Header>
		<Card.Content class="p-0 overflow-hidden">
			<div class="max-h-[280px] overflow-y-auto px-6 pb-4">
			{#if activeTab === "actions"}
				<!-- ---------------------------------------------------------- -->
				<!-- Actions tab: all artifacts needing attention               -->
				<!-- ---------------------------------------------------------- -->
				{#if pendingActions.length === 0}
					<div class="flex items-center gap-2 py-4 text-sm text-muted-foreground">
						<CheckCircle2Icon class="h-4 w-4 text-emerald-500 shrink-0" />
						<span>No pending actions — everything is moving</span>
					</div>
				{:else}
					<div class="space-y-1">
						{#each pendingActions as action (action.id)}
							<button
								class="flex w-full items-start justify-between gap-2 rounded-md px-2 py-1.5 text-left transition-colors hover:bg-accent/50"
								onclick={() => openArtifact(action.path)}
							>
								<div class="flex min-w-0 items-start gap-2">
									<div class="mt-0.5 shrink-0">
										<Badge variant={typeBadgeVariant(action.artifactType)} class="text-[10px] px-1.5 py-0 h-4">
											{typeBadgeLabel(action.artifactType)}
										</Badge>
									</div>
									<div class="min-w-0">
										<p class="truncate text-xs font-medium">{action.title}</p>
										<p class="text-[10px] text-muted-foreground">{action.action}</p>
									</div>
								</div>
								{#if action.priority}
									<span class="shrink-0 text-[10px] font-medium {priorityBadgeClass(action.priority)}">
										{action.priority}
									</span>
								{/if}
							</button>
						{/each}
					</div>
				{/if}
			{:else}
				<!-- ---------------------------------------------------------- -->
				<!-- Epics tab: in-progress + next ready epics                  -->
				<!-- ---------------------------------------------------------- -->
				{#if epicEntries.length === 0}
					<div class="flex items-center gap-2 py-4 text-sm text-muted-foreground">
						<MapIcon class="h-4 w-4 shrink-0" />
						<span>No active or ready epics</span>
					</div>
				{:else}
					<div class="space-y-1">
						{#each epicEntries as epic (epic.id)}
							<button
								class="flex w-full items-start justify-between gap-2 rounded-md px-2 py-1.5 text-left transition-colors hover:bg-accent/50"
								onclick={openRoadmap}
							>
								<div class="flex min-w-0 flex-col gap-0.5 flex-1">
									<div class="flex items-center gap-2">
										<Badge
											variant={epic.status === "in-progress" ? "default" : "outline"}
											class="text-[10px] px-1.5 py-0 h-4 shrink-0"
										>
											{epic.status === "in-progress" ? "Active" : "Ready"}
										</Badge>
										{#if epic.priority}
											<span class="text-[10px] font-medium {priorityBadgeClass(epic.priority)} shrink-0">
												{epic.priority}
											</span>
										{/if}
									</div>
									<p class="truncate text-xs font-medium">{epic.title}</p>
									{#if epic.taskProgress !== null}
										<!-- Progress bar -->
										<div class="flex items-center gap-1.5 mt-0.5">
											<div class="h-1 flex-1 rounded-full bg-muted overflow-hidden">
												<div
													class="h-full rounded-full bg-emerald-500 transition-all"
													style:width="{Math.round(epic.taskProgress * 100)}%"
												></div>
											</div>
											<span class="text-[10px] text-muted-foreground tabular-nums shrink-0">
												{epic.taskDone}/{epic.taskTotal}
											</span>
										</div>
									{/if}
								</div>
							</button>
						{/each}
					</div>

					<button
						class="mt-2 w-full text-center text-xs text-muted-foreground underline underline-offset-2 hover:text-foreground"
						onclick={openRoadmap}
					>
						View roadmap
					</button>
				{/if}
			{/if}
			</div>
		</Card.Content>
	</Card.Root>
{/if}
