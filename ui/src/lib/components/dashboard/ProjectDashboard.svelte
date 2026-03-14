<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import * as Card from "$lib/components/ui/card";
	import * as Collapsible from "$lib/components/ui/collapsible";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { Button } from "$lib/components/ui/button";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import NetworkIcon from "@lucide/svelte/icons/network";
	import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { ARTIFACT_TYPES } from "$lib/types/artifact-graph";
	import type { ArtifactGraphType } from "$lib/types/artifact-graph";
	import MilestoneContextCard from "./MilestoneContextCard.svelte";
	import IntegrityWidget from "./IntegrityWidget.svelte";
	import PipelineWidget from "./PipelineWidget.svelte";
	import ImprovementTrendsWidget from "./ImprovementTrendsWidget.svelte";
	import GraphHealthWidget from "./GraphHealthWidget.svelte";
	import LessonVelocityWidget from "./LessonVelocityWidget.svelte";
	import DecisionQueueWidget from "./DecisionQueueWidget.svelte";
	import { toast } from "$lib/stores/toast.svelte";
	import type { IntegrityCheck } from "$lib/types/artifact-graph";

	const project = $derived(projectStore.activeProject);
	const projectName = $derived(
		projectStore.projectSettings?.name ?? project?.name ?? "",
	);

	// Graph data for the Artifacts summary card
	const graphStats = $derived(artifactGraphSDK.stats);
	const graphLoading = $derived(artifactGraphSDK.loading);
	const graphError = $derived(artifactGraphSDK.error);
	const hasGraphData = $derived(artifactGraphSDK.graph.size > 0);

	// Collapsible state for Power User Details
	let detailsOpen = $state(false);
	// Collapsible state for Knowledge Pipeline
	let pipelineOpen = $state(false);

	// Graph health widget state (shared scan results for the Clarity column)
	let healthChecks = $state<IntegrityCheck[]>([]);
	let healthLoading = $state(false);
	let healthScanned = $state(false);

	// Auto-scan when the graph is ready
	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !healthScanned && !healthLoading) {
			void runHealthScan();
		}
	});

	async function runHealthScan(): Promise<void> {
		healthLoading = true;
		try {
			await artifactGraphSDK.refresh();
			healthChecks = await artifactGraphSDK.runIntegrityScan();
			healthScanned = true;
			const errors = healthChecks.filter((c) => c.severity === "Error").length;
			const warnings = healthChecks.filter((c) => c.severity === "Warning").length;
			await artifactGraphSDK.storeHealthSnapshot(errors, warnings).catch(() => {
				// Non-critical — don't block the UI if snapshot storage fails
			});
		} catch (err: unknown) {
			toast.error(err instanceof Error ? err.message : String(err));
		} finally {
			healthLoading = false;
		}
	}

	/** Humanize an artifact type string (e.g. "epic" → "Epics"). */
	function humanizeType(t: string): string {
		const singular = t.charAt(0).toUpperCase() + t.slice(1);
		if (singular.endsWith("s") || singular.endsWith("ch")) return singular + "es";
		return singular + "s";
	}

	/**
	 * Map artifact graph type to navigation activity key.
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

			<!-- Narrative layout -->
			<div class="flex flex-col gap-4">

				<!-- Row 1: MilestoneContextCard — full width -->
				<MilestoneContextCard />

				<!-- Row 2: Three pillar columns -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<!-- Column 1: Where You Are (Clarity) -->
					<Card.Root class="flex flex-col">
						<Card.Header class="pb-2">
							<Card.Title class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">
								Where You Are
							</Card.Title>
							<p class="text-xs text-muted-foreground">Clarity</p>
						</Card.Header>
						<Card.Content class="flex-1">
							<GraphHealthWidget
								checks={healthChecks}
								loading={healthLoading}
								scanned={healthScanned}
								onScan={runHealthScan}
							/>
						</Card.Content>
					</Card.Root>

					<!-- Column 2: How You're Improving (Learning) -->
					<ImprovementTrendsWidget />

					<!-- Column 3: What's Next (Purpose) -->
					<div class="flex flex-col gap-4">
						<div class="px-0">
							<p class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">What's Next</p>
							<p class="text-xs text-muted-foreground">Purpose</p>
						</div>
						<LessonVelocityWidget />
						<DecisionQueueWidget />
					</div>
				</div>

				<!-- Row 3: Power User Details (collapsible) -->
				<Collapsible.Root bind:open={detailsOpen}>
					<div class="rounded-lg border border-border">
						<Collapsible.Trigger class="w-full">
							<div class="flex items-center justify-between px-4 py-3 text-sm font-medium hover:bg-accent/30 rounded-lg transition-colors">
								<span class="text-muted-foreground">Power User Details</span>
								{#if detailsOpen}
									<ChevronDownIcon class="h-4 w-4 text-muted-foreground" />
								{:else}
									<ChevronRightIcon class="h-4 w-4 text-muted-foreground" />
								{/if}
							</div>
						</Collapsible.Trigger>
						<Collapsible.Content>
							<div class="border-t border-border p-4">
								<div class="grid grid-cols-1 gap-4 xl:grid-cols-2">
									<!-- Pipeline Health -->
									<IntegrityWidget />

									<!-- Detected stack -->
									{#if project.detected_stack}
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
									{/if}

									<!-- Artifacts summary -->
									<Card.Root class="xl:col-span-2">
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
												<div class="grid grid-cols-2 gap-2 sm:grid-cols-3 md:grid-cols-4 xl:grid-cols-6">
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
						</Collapsible.Content>
					</div>
				</Collapsible.Root>

				<!-- Row 4: Knowledge Pipeline (collapsible) -->
				<Collapsible.Root bind:open={pipelineOpen}>
					<div class="rounded-lg border border-border">
						<Collapsible.Trigger class="w-full">
							<div class="flex items-center justify-between px-4 py-3 text-sm font-medium hover:bg-accent/30 rounded-lg transition-colors">
								<span class="text-muted-foreground">Knowledge Pipeline</span>
								{#if pipelineOpen}
									<ChevronDownIcon class="h-4 w-4 text-muted-foreground" />
								{:else}
									<ChevronRightIcon class="h-4 w-4 text-muted-foreground" />
								{/if}
							</div>
						</Collapsible.Trigger>
						<Collapsible.Content>
							<div class="border-t border-border p-4">
								<PipelineWidget />
							</div>
						</Collapsible.Content>
					</div>
				</Collapsible.Root>

			</div>
		{/if}
	</div>
</ScrollArea.Root>
