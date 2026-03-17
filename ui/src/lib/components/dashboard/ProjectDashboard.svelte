<script lang="ts">
	import { Icon, ScrollArea } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, CardAction } from "@orqastudio/svelte-components/pure";
	import { EmptyState } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import MilestoneContextCard from "./MilestoneContextCard.svelte";
	import IntegrityWidget from "./IntegrityWidget.svelte";
	import PipelineWidget from "./PipelineWidget.svelte";
	import ImprovementTrendsWidget from "./ImprovementTrendsWidget.svelte";
	import GraphHealthWidget from "./GraphHealthWidget.svelte";
	import LessonVelocityWidget from "./LessonVelocityWidget.svelte";
	import DecisionQueueWidget from "./DecisionQueueWidget.svelte";
	import ToolStatusWidget from "./ToolStatusWidget.svelte";

	const { projectStore, artifactGraphSDK, toast } = getStores();
	import type { IntegrityCheck } from "@orqastudio/types";

	const project = $derived(projectStore.activeProject);
	const projectName = $derived(
		projectStore.projectSettings?.name ?? project?.name ?? "",
	);

	// Graph health widget state (shared scan results for the Clarity column)
	let healthChecks = $state<IntegrityCheck[]>([]);
	let healthLoading = $state(false);
	let healthFixing = $state(false);
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

	async function runHealthAutoFix(): Promise<void> {
		healthFixing = true;
		try {
			const fixableChecks = healthChecks.filter((c) => c.auto_fixable);
			const appliedFixes = await artifactGraphSDK.applyAutoFixes(fixableChecks);
			toast.success(`${appliedFixes.length} fix${appliedFixes.length !== 1 ? "es" : ""} applied`);
			// Refresh graph after fixes wrote to disk, then re-scan
			await artifactGraphSDK.refresh();
			healthChecks = await artifactGraphSDK.runIntegrityScan();
		} catch (err: unknown) {
			toast.error(err instanceof Error ? err.message : String(err));
		} finally {
			healthFixing = false;
		}
	}
</script>

<ScrollArea class="h-full">
	<div class="p-6">
		{#if !project}
			<EmptyState
				icon="folder-open"
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
						<Icon name="folder-open" size="xl" />
					{/if}
					<div>
						<h1 class="text-2xl font-bold">{projectName}</h1>
						{#if projectStore.isOrganisation}
							<p class="text-xs text-primary font-medium">Organisation ({projectStore.childProjects.length} projects)</p>
						{/if}
						{#if projectStore.projectSettings?.description}
							<p class="text-sm text-muted-foreground">{projectStore.projectSettings.description}</p>
						{:else}
							<p class="text-sm text-muted-foreground">{project.path}</p>
						{/if}
					</div>
				</div>
			</div>

			<!-- Org-mode: per-project artifact breakdown -->
			{#if projectStore.isOrganisation && !projectStore.activeChildProject}
				<div class="mb-4">
					<CardRoot class="gap-2">
						<CardHeader class="pb-2">
							<CardTitle class="flex items-center gap-1.5 text-sm font-semibold">
								<Icon name="layers" size="md" />
								Project Breakdown
							</CardTitle>
						</CardHeader>
						<CardContent class="pt-0">
							<div class="grid grid-cols-2 gap-2 md:grid-cols-3 lg:grid-cols-4">
								{#each projectStore.childProjects as child}
									{@const childCount = [...artifactGraphSDK.graph.values()].filter(n => n.project === child.name).length}
									<button
										class="flex items-center gap-2 rounded border border-border p-2 text-left text-xs hover:bg-muted transition-colors"
										onclick={() => { projectStore.activeChildProject = child.name; }}
									>
										<Icon name="folder" size="sm" />
										<div class="min-w-0 flex-1">
											<p class="font-medium truncate">{child.name}</p>
											<p class="text-muted-foreground">{childCount} artifacts</p>
										</div>
									</button>
								{/each}
							</div>
						</CardContent>
					</CardRoot>
				</div>
			{/if}

			<!-- Narrative layout -->
			<div class="flex flex-col gap-4">

				<!-- Row 1: MilestoneContextCard — full width -->
				<MilestoneContextCard />

				<!-- Row 2: Three pillar columns — each card carries its own title -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-3 items-stretch">
					<!-- Column 1: Where You Are (Clarity) — title lives inside GraphHealthWidget -->
					<GraphHealthWidget
						checks={healthChecks}
						loading={healthLoading}
						fixing={healthFixing}
						scanned={healthScanned}
						onScan={runHealthScan}
						onAutoFix={runHealthAutoFix}
					/>

					<!-- Column 2: Learning — ImprovementTrendsWidget wrapped in a card -->
					<CardRoot class="gap-2 overflow-hidden">
						<CardHeader class="pb-2">
							<CardTitle class="flex items-center gap-1.5 text-sm font-semibold">
								<Icon name="trending-up" size="md" />
								Learning
							</CardTitle>
							<CardDescription class="text-xs">How You're Improving</CardDescription>
						</CardHeader>
						<CardContent class="flex flex-col px-0 pt-0 pb-0">
							<ImprovementTrendsWidget />
						</CardContent>
					</CardRoot>

					<!-- Column 3: What's Next (Purpose) — title lives inside DecisionQueueWidget -->
					<DecisionQueueWidget />
				</div>

				<!-- Row 3: Knowledge Pipeline + Lesson Velocity — same height via h-full on both cards -->
				<div class="grid grid-cols-3 gap-4 items-stretch">
					<div class="col-span-2 flex">
						<div class="w-full">
							<PipelineWidget />
						</div>
					</div>
					<div class="col-span-1 flex">
						<div class="w-full">
							<LessonVelocityWidget />
						</div>
					</div>
				</div>

				<!-- Row 4: Pipeline Health — full width at bottom -->
				<IntegrityWidget />

				<!-- Row 5: Plugin Tools — shows registered tool statuses with Run buttons -->
				<ToolStatusWidget />

			</div>
		{/if}
	</div>
</ScrollArea>
