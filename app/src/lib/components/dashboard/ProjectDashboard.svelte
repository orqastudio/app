<script lang="ts">
	import { Icon, Heading } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
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
	import type { IntegrityCheck, GraphHealthData } from "@orqastudio/types";

	const project = $derived(projectStore.activeProject);
	const projectName = $derived(
		projectStore.projectSettings?.name ?? project?.name ?? "",
	);

	// Graph health widget state (shared scan results for the Clarity column)
	let healthChecks = $state<IntegrityCheck[]>([]);
	let healthLoading = $state(false);
	let healthFixing = $state(false);
	let healthScanned = $state(false);
	let graphHealth = $state<GraphHealthData | null>(null);

	// Auto-scan when the graph is ready
	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !healthScanned && !healthLoading) {
			void runHealthScan();
		}
	});

	/** Refreshes the artifact graph and runs a health scan, storing the resulting snapshot. */
	async function runHealthScan(): Promise<void> {
		healthLoading = true;
		try {
			await artifactGraphSDK.refresh();
			const [checks, health] = await Promise.all([
				artifactGraphSDK.runIntegrityScan(),
				artifactGraphSDK.getGraphHealth(),
			]);
			healthChecks = checks;
			graphHealth = health;
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

	/** Applies all auto-fixable integrity checks, then re-scans to reflect the updated state. */
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

<div class="h-full overflow-y-auto">
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
			<div class="mb-6 flex items-center gap-3">
				{#if projectStore.iconDataUrl}
					<img src={projectStore.iconDataUrl} alt={projectName} class="h-12 w-12 rounded object-contain" />
				{:else}
					<Icon name="folder-open" size="xl" />
				{/if}
				<div>
					<Heading level={1}>{projectName}</Heading>
					{#if projectStore.projectSettings?.description}
						<p class="text-sm text-muted-foreground">{projectStore.projectSettings.description}</p>
					{:else}
						<p class="text-sm text-muted-foreground">{project.path}</p>
					{/if}
				</div>
			</div>

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
						{graphHealth}
						onScan={runHealthScan}
						onAutoFix={runHealthAutoFix}
					/>

					<!-- Column 2: Learning — ImprovementTrendsWidget wrapped in a card -->
					<CardRoot>
						<CardHeader compact>
							<CardTitle>
								<div class="flex items-center gap-1">
									<Icon name="trending-up" size="md" />
									Learning
								</div>
							</CardTitle>
							<CardDescription>How You're Improving</CardDescription>
						</CardHeader>
						<CardContent>
							<ImprovementTrendsWidget />
						</CardContent>
					</CardRoot>

					<!-- Column 3: What's Next (Purpose) — title lives inside DecisionQueueWidget -->
					<DecisionQueueWidget />
				</div>

				<!-- Row 3: Knowledge Pipeline + Lesson Velocity — same height via h-full on both cards -->
				<div class="grid grid-cols-3 gap-4 items-stretch">
					<div class="col-span-2">
						<PipelineWidget />
					</div>
					<div class="col-span-1">
						<LessonVelocityWidget />
					</div>
				</div>

				<!-- Row 4: Pipeline Health — full width at bottom -->
				<IntegrityWidget />

				<!-- Row 5: Plugin Tools — shows registered tool statuses with Run buttons -->
				<ToolStatusWidget />

			</div>
		{/if}
	</div>
</div>
