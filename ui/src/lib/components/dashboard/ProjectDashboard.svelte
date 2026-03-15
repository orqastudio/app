<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
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

				<!-- Row 2: Three pillar columns -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<!-- Column 1: Where You Are (Clarity) -->
					<div class="flex flex-col gap-4">
						<div class="px-0">
							<p class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">Where You Are</p>
							<p class="text-xs text-muted-foreground">Clarity</p>
						</div>
						<GraphHealthWidget
							checks={healthChecks}
							loading={healthLoading}
							fixing={healthFixing}
							scanned={healthScanned}
							onScan={runHealthScan}
							onAutoFix={runHealthAutoFix}
						/>
					</div>

					<!-- Column 2: How You're Improving (Learning) -->
					<div class="flex flex-col gap-4">
						<div class="px-0">
							<p class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">How You're Improving</p>
							<p class="text-xs text-muted-foreground">Learning</p>
						</div>
						<ImprovementTrendsWidget />
					</div>

					<!-- Column 3: What's Next (Purpose) -->
					<div class="flex flex-col gap-4">
						<div class="px-0">
							<p class="text-sm font-semibold text-muted-foreground uppercase tracking-wide">What's Next</p>
							<p class="text-xs text-muted-foreground">Purpose</p>
						</div>
						<DecisionQueueWidget />
					</div>
				</div>

				<!-- Row 3: Knowledge Pipeline + Lesson Velocity — shared row -->
				<div class="grid grid-cols-3 gap-4">
					<div class="col-span-2">
						<PipelineWidget />
					</div>
					<div class="col-span-1">
						<LessonVelocityWidget />
					</div>
				</div>

				<!-- Row 4: Pipeline Health — full width at bottom -->
				<IntegrityWidget />

			</div>
		{/if}
	</div>
</ScrollArea.Root>
