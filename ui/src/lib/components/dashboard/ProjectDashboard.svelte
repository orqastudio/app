<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import * as Card from "$lib/components/ui/card";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import MilestoneContextCard from "./MilestoneContextCard.svelte";
	import IntegrityWidget from "./IntegrityWidget.svelte";
	import PipelineWidget from "./PipelineWidget.svelte";
	import ImprovementTrendsWidget from "./ImprovementTrendsWidget.svelte";
	import GraphHealthWidget from "./GraphHealthWidget.svelte";
	import LessonVelocityWidget from "./LessonVelocityWidget.svelte";
	import DecisionQueueWidget from "./DecisionQueueWidget.svelte";
	import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
	import { toast } from "$lib/stores/toast.svelte";
	import type { IntegrityCheck } from "$lib/types/artifact-graph";

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

				<!-- Row 2: Three pillar columns — each card carries its own title -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
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
					<Card.Root class="gap-2 overflow-hidden">
						<Card.Header class="pb-2">
							<Card.Title class="flex items-center gap-1.5 text-sm font-semibold">
								<TrendingUpIcon class="h-4 w-4 text-muted-foreground" />
								Learning
							</Card.Title>
							<Card.Description class="text-xs">How You're Improving</Card.Description>
						</Card.Header>
						<Card.Content class="flex flex-col px-0 pt-0 pb-0">
							<ImprovementTrendsWidget />
						</Card.Content>
					</Card.Root>

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

			</div>
		{/if}
	</div>
</ScrollArea.Root>
