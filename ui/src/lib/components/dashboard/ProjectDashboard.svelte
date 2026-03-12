<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { Button } from "$lib/components/ui/button";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import NetworkIcon from "@lucide/svelte/icons/network";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import CoverageIndicator from "$lib/components/governance/CoverageIndicator.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { governanceStore } from "$lib/stores/governance.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { ARTIFACT_TYPES } from "$lib/types/artifact-graph";
	import type { Component } from "svelte";
	import { SvelteMap } from "svelte/reactivity";

	const project = $derived(projectStore.activeProject);
	const projectName = $derived(
		projectStore.projectSettings?.name ?? project?.name ?? "",
	);

	interface ArtifactCategory {
		icon: Component;
		label: string;
		activity: "docs" | "agents" | "rules" | "skills" | "hooks";
	}

	const artifactCategories: ArtifactCategory[] = [
		{ icon: FileTextIcon, label: "Docs", activity: "docs" },
		{ icon: BotIcon, label: "Agents", activity: "agents" },
		{ icon: ShieldIcon, label: "Rules", activity: "rules" },
		{ icon: ZapIcon, label: "Skills", activity: "skills" },
		{ icon: GitBranchIcon, label: "Hooks", activity: "hooks" },
	];

	function navigateToCategory(activity: "docs" | "agents" | "rules" | "skills" | "hooks") {
		navigationStore.setActivity(activity);
	}

	// Derived graph data for the insights card.
	const graphStats = $derived(artifactGraphSDK.stats);
	const graphLoading = $derived(artifactGraphSDK.loading);
	const graphError = $derived(artifactGraphSDK.error);

	/** Count of nodes per artifact type — computed from the in-memory graph. */
	const typeCounts = $derived.by(() => {
		const counts: { type: string; count: number }[] = [];
		for (const t of ARTIFACT_TYPES) {
			const count = artifactGraphSDK.byType(t).length;
			if (count > 0) counts.push({ type: t, count });
		}
		return counts.sort((a, b) => b.count - a.count);
	});

	/** Count of nodes per status value — computed from the in-memory graph. */
	const statusCounts = $derived.by(() => {
		const map = new SvelteMap<string, number>();
		for (const node of artifactGraphSDK.graph.values()) {
			const s = node.status ?? "(none)";
			map.set(s, (map.get(s) ?? 0) + 1);
		}
		return [...map.entries()]
			.map(([status, count]) => ({ status, count }))
			.sort((a, b) => b.count - a.count);
	});

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

			<!-- Detected stack -->
			{#if project.detected_stack}
				<Card.Root class="mb-4">
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

			<!-- Governance health -->
			<Card.Root class="mb-4">
				<Card.Header class="pb-3">
					<div class="flex items-center justify-between">
						<Card.Title class="text-base">
							<div class="flex items-center gap-2">
								<ShieldIcon class="h-4 w-4" />
								Governance Health
							</div>
						</Card.Title>
						<Button
							variant="ghost"
							size="sm"
							onclick={() => governanceStore.showWizard()}
						>
							<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
							Re-analyze Governance
						</Button>
					</div>
				</Card.Header>
				<Card.Content>
					{#if governanceStore.loading}
						<div class="flex items-center justify-center py-4">
							<LoadingSpinner />
						</div>
					{:else if governanceStore.error}
						<ErrorDisplay
							message={governanceStore.error}
							onRetry={() => {
								const p = projectStore.activeProject;
								if (p) governanceStore.scan(p.id);
							}}
						/>
					{:else if governanceStore.scanResult}
						<CoverageIndicator
							areas={governanceStore.scanResult.areas}
							coverageRatio={governanceStore.scanResult.coverage_ratio}
						/>
					{:else}
						<p class="text-sm text-muted-foreground">Scan not yet run</p>
					{/if}
				</Card.Content>
			</Card.Root>

			<!-- Artifact Graph Insights -->
			<Card.Root class="mb-4">
				<Card.Header class="pb-3">
					<div class="flex items-center justify-between">
						<Card.Title class="text-base">
							<div class="flex items-center gap-2">
								<NetworkIcon class="h-4 w-4" />
								Artifact Graph
							</div>
						</Card.Title>
						<Button
							variant="ghost"
							size="sm"
							onclick={() => artifactGraphSDK.refresh()}
							disabled={graphLoading}
						>
							<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
							Refresh
						</Button>
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
							No artifact graph data. Click Refresh to build the index.
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

						<!-- By type -->
						{#if typeCounts.length > 0}
							<div class="mb-3">
								<p class="mb-1.5 text-xs font-medium text-muted-foreground uppercase tracking-wide">By Type</p>
								<div class="flex flex-wrap gap-x-4 gap-y-1">
									{#each typeCounts as { type, count } (type)}
										<span class="text-sm">
											<span class="font-medium capitalize">{type}</span>
											<span class="ml-1 text-muted-foreground tabular-nums">({count})</span>
										</span>
									{/each}
								</div>
							</div>
						{/if}

						<!-- By status -->
						{#if statusCounts.length > 0}
							<div>
								<p class="mb-1.5 text-xs font-medium text-muted-foreground uppercase tracking-wide">By Status</p>
								<div class="flex flex-wrap gap-x-4 gap-y-1">
									{#each statusCounts as { status, count } (status)}
										<span class="text-sm">
											<span class="font-medium">{status}</span>
											<span class="ml-1 text-muted-foreground tabular-nums">({count})</span>
										</span>
									{/each}
								</div>
							</div>
						{/if}
					{/if}
				</Card.Content>
			</Card.Root>

			<!-- Governance artifacts -->
			<Card.Root class="mb-4">
				<Card.Header class="pb-3">
					<Card.Title class="text-base">Governance Artifacts</Card.Title>
					<Card.Description>Click a category to browse its artifacts</Card.Description>
				</Card.Header>
				<Card.Content>
					<div class="grid grid-cols-2 gap-2 sm:grid-cols-3 md:grid-cols-5">
						{#each artifactCategories as cat (cat.activity)}
							{@const Icon = cat.icon}
							<button
								class="flex flex-col items-center gap-1.5 rounded-lg border border-border p-3 transition-colors hover:bg-accent/50"
								onclick={() => navigateToCategory(cat.activity)}
							>
								<Icon class="h-5 w-5 text-muted-foreground" />
								<span class="text-sm font-medium">{cat.label}</span>
							</button>
						{/each}
					</div>
				</Card.Content>
			</Card.Root>

		{/if}
	</div>
</ScrollArea.Root>
