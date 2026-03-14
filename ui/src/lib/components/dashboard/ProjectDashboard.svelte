<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import * as Card from "$lib/components/ui/card";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import NetworkIcon from "@lucide/svelte/icons/network";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { ARTIFACT_TYPES } from "$lib/types/artifact-graph";
	import type { ArtifactGraphType } from "$lib/types/artifact-graph";
	import IntegrityWidget from "./IntegrityWidget.svelte";
	import PipelineWidget from "./PipelineWidget.svelte";
	import HealthTrendWidget from "./HealthTrendWidget.svelte";

	const project = $derived(projectStore.activeProject);
	const projectName = $derived(
		projectStore.projectSettings?.name ?? project?.name ?? "",
	);

	// Derived graph data for the insights card.
	const graphStats = $derived(artifactGraphSDK.stats);
	const graphLoading = $derived(artifactGraphSDK.loading);
	const graphError = $derived(artifactGraphSDK.error);

	/** Humanize an artifact type string (e.g. "epic" → "Epics"). */
	function humanizeType(t: string): string {
		const singular = t.charAt(0).toUpperCase() + t.slice(1);
		// Pluralize
		if (singular.endsWith("s") || singular.endsWith("ch")) return singular + "es";
		return singular + "s";
	}

	/**
	 * Map artifact graph type to navigation activity key.
	 * The navigation keys are plural (e.g. "epics", "tasks") while
	 * graph types are singular (e.g. "epic", "task").
	 */
	function typeToNavKey(t: ArtifactGraphType): string | null {
		const mapping: Record<string, string> = {
			epic: "epics",
			task: "tasks",
			milestone: "milestones",
			idea: "ideas",
			decision: "decisions",
			research: "research",
			lesson: "lessons",
			rule: "rules",
			agent: "agents",
			skill: "skills",
			hook: "hooks",
			pillar: "pillars",
			doc: "docs",
		};
		return mapping[t] ?? null;
	}

	/** Per-type card data with status breakdown. */
	const typeCards = $derived.by(() => {
		const cards: {
			type: ArtifactGraphType;
			label: string;
			count: number;
			statuses: { status: string; count: number; dotClass: string }[];
		}[] = [];
		for (const t of ARTIFACT_TYPES) {
			const nodes = artifactGraphSDK.byType(t);
			if (nodes.length === 0) continue;

			// Compute status breakdown
			const statusMap = new SvelteMap<string, number>();
			for (const node of nodes) {
				const s = node.status ?? "(none)";
				statusMap.set(s, (statusMap.get(s) ?? 0) + 1);
			}
			const statuses = [...statusMap.entries()]
				.map(([status, count]) => ({
					status,
					count,
					dotClass: statusDotClass(status),
				}))
				.sort((a, b) => b.count - a.count);

			cards.push({ type: t, label: humanizeType(t), count: nodes.length, statuses });
		}
		return cards.sort((a, b) => b.count - a.count);
	});

	/** Map status to a dot color class. */
	function statusDotClass(status: string): string {
		const map: Record<string, string> = {
			active: "bg-blue-500",
			"in-progress": "bg-blue-500",
			exploring: "bg-blue-500",
			ready: "bg-blue-500",
			done: "bg-emerald-500",
			complete: "bg-emerald-500",
			accepted: "bg-emerald-500",
			shaped: "bg-emerald-500",
			draft: "bg-zinc-400",
			captured: "bg-zinc-400",
			todo: "bg-zinc-400",
			proposed: "bg-zinc-400",
			planning: "bg-zinc-400",
			review: "bg-amber-500",
			recurring: "bg-amber-500",
			promoted: "bg-purple-500",
			inactive: "bg-zinc-500/60",
			superseded: "bg-zinc-500/60",
			deprecated: "bg-zinc-500/60",
			archived: "bg-zinc-500/60",
			surpassed: "bg-zinc-500/60",
		};
		return map[status] ?? "bg-zinc-400";
	}

	function navigateToType(t: ArtifactGraphType) {
		const key = typeToNavKey(t);
		if (key) {
			navigationStore.setActivity(key);
		}
	}

	const hasGraphData = $derived(artifactGraphSDK.graph.size > 0);
</script>

<ScrollArea.Root class="h-full">
	<div class="p-6">
		{#if !project}
			<EmptyState
				icon={FolderOpenIcon}
				title="No project open"
				description="Open a project to view its dashboard and governance artifacts."
				action={{ label: "Open Project", onclick: () => {} }}
			/>
		{:else}
			<!-- Project header -->
			<div class="mb-6">
				<div class="flex items-center gap-3">
					{#if projectStore.iconDataUrl}
						<img src={projectStore.iconDataUrl} alt={projectName} class="h-12 w-12 rounded object-contain" />
					{:else}
						<FolderOpenIcon class="h-12 w-12 text-muted-foreground" />
					{/if}
					<div>
						<h1 class="text-2xl font-bold">{projectName}</h1>
						{#if projectStore.projectSettings?.description}
							<p class="text-sm text-muted-foreground">{projectStore.projectSettings.description}</p>
						{:else}
							<p class="text-sm text-muted-foreground">{project.path}</p>
						{/if}
					</div>
				</div>
			</div>

			<!-- Dashboard grid -->
			<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
				<!-- Health Trends — prominent, spans 2 cols on wide screens -->
				<div class="xl:col-span-2">
					<HealthTrendWidget />
				</div>

				<!-- Pipeline Health — compact when clear, expands with errors -->
				<div>
					<IntegrityWidget />
				</div>

				<!-- Detected stack -->
				{#if project.detected_stack}
					<div>
						<Card.Root>
							<Card.Header class="pb-3">
								<Card.Title class="text-base">
									<div class="flex items-center gap-2">
										<LayersIcon class="h-4 w-4" />
										Detected Stack
									</div>
								</Card.Title>
							</Card.Header>
							<Card.Content>
								<div class="grid grid-cols-2 gap-3 text-sm">
									<div>
										<span class="text-muted-foreground">Languages:</span>
										<span class="ml-1 font-medium">{project.detected_stack.languages.join(", ") || "None detected"}</span>
									</div>
									<div>
										<span class="text-muted-foreground">Frameworks:</span>
										<span class="ml-1 font-medium">{project.detected_stack.frameworks.join(", ") || "None detected"}</span>
									</div>
									{#if project.detected_stack.package_manager}
										<div>
											<span class="text-muted-foreground">Package Manager:</span>
											<span class="ml-1 font-medium">{project.detected_stack.package_manager}</span>
										</div>
									{/if}
									<div>
										<span class="text-muted-foreground">Claude Config:</span>
										<span class="ml-1 font-medium">{project.detected_stack.has_claude_config ? "Yes" : "No"}</span>
									</div>
								</div>
							</Card.Content>
						</Card.Root>
					</div>
				{/if}

				<!-- Knowledge Pipeline -->
				<div>
					<PipelineWidget />
				</div>

				<!-- Artifacts summary -->
				<div class="md:col-span-2 xl:col-span-1">
					<Card.Root>
						<Card.Header class="pb-3">
							<div class="flex items-center justify-between">
								<Card.Title class="text-base">
									<div class="flex items-center gap-2">
										<NetworkIcon class="h-4 w-4" />
										Artifacts
										{#if graphLoading}
											<LoadingSpinner size="sm" />
										{/if}
									</div>
								</Card.Title>
							</div>
						</Card.Header>
						<Card.Content>
							{#if graphLoading && !hasGraphData}
								<div class="flex items-center justify-center py-4">
									<LoadingSpinner />
								</div>
							{:else if graphError && !hasGraphData}
								<ErrorDisplay
									message={graphError}
									onRetry={() => artifactGraphSDK.refresh()}
								/>
							{:else if !hasGraphData}
								<p class="text-sm text-muted-foreground">
									No artifact graph data. Use Re-index in the status bar to build the index.
								</p>
							{:else}
								<!-- Summary stats row -->
								{#if graphStats}
									<div class="mb-4 grid grid-cols-4 gap-3 text-sm">
										<div class="text-center">
											<div class="text-lg font-semibold tabular-nums">{graphStats.node_count}</div>
											<div class="text-xs text-muted-foreground">Nodes</div>
										</div>
										<div class="text-center">
											<div class="text-lg font-semibold tabular-nums">{graphStats.edge_count}</div>
											<div class="text-xs text-muted-foreground">Edges</div>
										</div>
										<div class="text-center">
											<div class="text-lg font-semibold tabular-nums {graphStats.orphan_count > 0 ? 'text-warning' : ''}">{graphStats.orphan_count}</div>
											<div class="text-xs text-muted-foreground">Orphans</div>
										</div>
										<div class="text-center">
											<div class="text-lg font-semibold tabular-nums {graphStats.broken_ref_count > 0 ? 'text-destructive' : ''}">{graphStats.broken_ref_count}</div>
											<div class="text-xs text-muted-foreground">Broken Refs</div>
										</div>
									</div>
								{/if}

								<!-- Per-type cards -->
								<div class="grid grid-cols-2 gap-2 sm:grid-cols-3 md:grid-cols-4">
									{#each typeCards as card (card.type)}
										<button
											class="flex flex-col gap-1.5 rounded-lg border border-border p-3 text-left transition-colors hover:bg-accent/50"
											onclick={() => navigateToType(card.type)}
										>
											<div class="flex items-baseline justify-between">
												<span class="text-sm font-medium">{card.label}</span>
												<span class="text-xs tabular-nums text-muted-foreground">{card.count}</span>
											</div>
											{#if card.statuses.length > 0}
												<div class="flex flex-wrap gap-1">
													{#each card.statuses as s (s.status)}
														<span class="flex items-center gap-1 text-[10px] text-muted-foreground">
															<span class="inline-block h-1.5 w-1.5 rounded-full {s.dotClass}"></span>
															{s.status}
															<span class="tabular-nums">({s.count})</span>
														</span>
													{/each}
												</div>
											{/if}
										</button>
									{/each}
								</div>
							{/if}
						</Card.Content>
					</Card.Root>
				</div>
			</div>
		{/if}
	</div>
</ScrollArea.Root>
