<script lang="ts">
	import { Icon, Heading } from "@orqastudio/svelte-components/pure";
	import {
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
	} from "@orqastudio/svelte-components/pure";
	import {
		EmptyState,
		Stack,
		HStack,
		Grid,
		GridCell,
		Text,
		AppIcon,
	} from "@orqastudio/svelte-components/pure";
	import { Panel, ScrollArea } from "@orqastudio/svelte-components/pure";
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
	const projectName = $derived(projectStore.projectSettings?.name ?? project?.name ?? "");

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

<ScrollArea full>
	<Panel padding="loose">
		<Stack gap={4}>
			{#if !project}
				<EmptyState
					icon="folder-open"
					title="No project open"
					description="Open a project to view its dashboard and governance artifacts."
					action={{ label: "Open Project", onclick: () => {} }}
				/>
			{:else}
				<!-- Project header -->
				<HStack gap={3} align="center">
					{#if projectStore.iconDataUrl}
						<AppIcon
							src={projectStore.iconDataUrl}
							alt={projectName}
							size="md"
							rounded
							objectContain
						/>
					{:else}
						<Icon name="folder-open" size="xl" />
					{/if}
					<Stack gap={0}>
						<Heading level={1}>{projectName}</Heading>
						{#if projectStore.projectSettings?.description}
							<Text variant="body-muted">{projectStore.projectSettings.description}</Text>
						{:else}
							<Text variant="body-muted">{project.path}</Text>
						{/if}
					</Stack>
				</HStack>

				<!-- Row 1: MilestoneContextCard — full width -->
				<MilestoneContextCard />

				<!-- Row 2: Three pillar columns — each card carries its own title -->
				<Grid cols={1} md={3} gap={4}>
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
								<HStack gap={1}>
									<Icon name="trending-up" size="md" />
									Learning
								</HStack>
							</CardTitle>
							<CardDescription>How You're Improving</CardDescription>
						</CardHeader>
						<CardContent>
							<ImprovementTrendsWidget />
						</CardContent>
					</CardRoot>

					<!-- Column 3: What's Next (Purpose) — title lives inside DecisionQueueWidget -->
					<DecisionQueueWidget />
				</Grid>

				<!-- Row 3: Knowledge Pipeline (2/3) + Lesson Velocity (1/3) -->
				<Grid cols={3} items="stretch" gap={4}>
					<GridCell span={2}>
						<PipelineWidget />
					</GridCell>
					<GridCell span={1}>
						<LessonVelocityWidget />
					</GridCell>
				</Grid>

				<!-- Row 4: Pipeline Health — full width at bottom -->
				<IntegrityWidget />

				<!-- Row 5: Plugin Tools — shows registered tool statuses with Run buttons -->
				<ToolStatusWidget />
			{/if}
		</Stack>
	</Panel>
</ScrollArea>
