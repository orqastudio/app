<script lang="ts">
	import {
		Icon,
		TooltipRoot,
		TooltipTrigger,
		TooltipContent,
		LoadingSpinner,
		Button,
		Caption,
		Stack,
		HStack,
		Grid,
		ScrollArea,
		Text,
		Center,
		Dot,
		SectionHeader,
		Panel,
	} from "@orqastudio/svelte-components/pure";
	import type { GraphHealthData, HealthSnapshot } from "@orqastudio/types";
	import { fmt, pct } from "@orqastudio/sdk";

	interface Props {
		health: GraphHealthData | null;
		snapshots: HealthSnapshot[];
		loading: boolean;
		onRefresh: () => void;
	}

	const { health, snapshots, loading, onRefresh }: Props = $props();

	// Previous snapshot for historical comparison (index 1 = second most recent).
	const prevSnapshot = $derived(snapshots.length > 1 ? snapshots[1] : null);

	// Overall health score: largest connected component ratio as percentage string.
	const healthScore = $derived(health ? pct(health.largest_component_ratio) : "0");

	type TrafficLight = "green" | "amber" | "red" | "empty";

	const overallStatus = $derived.by((): TrafficLight => {
		if (!health || health.total_nodes === 0) return "empty";
		if (health.largest_component_ratio > 0.9) return "green";
		if (health.largest_component_ratio > 0.7) return "amber";
		return "red";
	});

	// Maps traffic light status to a Dot color for the health indicator.
	type DotColor = "success" | "warning" | "destructive" | "muted";
	const overallDotColor = $derived.by((): DotColor => {
		if (overallStatus === "green") return "success";
		if (overallStatus === "amber") return "warning";
		if (overallStatus === "red") return "destructive";
		return "muted";
	});

	// --- Per-metric severity helpers ---
	// Return semantic tone values for use with the Text/Caption `tone` prop.

	type MetricTone = "success" | "warning" | "destructive" | "muted";

	// Outlier severity: green 0, amber 1-3, red >3
	const outlierTone = $derived.by((): MetricTone => {
		if (!health) return "muted";
		if (health.outlier_count === 0) return "success";
		if (health.outlier_count <= 3) return "warning";
		return "destructive";
	});

	// Whether to show the age distribution sub-row.
	const showOutlierAgeDistribution = $derived(health !== null && health.outlier_count > 0);

	const degreeTone = $derived.by((): MetricTone => {
		if (!health) return "muted";
		if (health.avg_degree >= 4) return "success";
		if (health.avg_degree >= 2) return "warning";
		return "destructive";
	});

	// Delivery connectivity: green >=90%, amber 70-90%, red <70%
	const deliveryTone = $derived.by((): MetricTone => {
		if (!health) return "muted";
		if (health.delivery_connectivity >= 0.9) return "success";
		if (health.delivery_connectivity >= 0.7) return "warning";
		return "destructive";
	});

	// Learning connectivity: green >=80%, amber 50-80%, red <50%
	const learningTone = $derived.by((): MetricTone => {
		if (!health) return "muted";
		if (health.learning_connectivity >= 0.8) return "success";
		if (health.learning_connectivity >= 0.5) return "warning";
		return "destructive";
	});

	const traceabilityTone = $derived.by((): MetricTone => {
		if (!health) return "muted";
		if (health.pillar_traceability >= 80) return "success";
		if (health.pillar_traceability >= 50) return "warning";
		return "destructive";
	});

	const brokenRefTone = $derived.by((): MetricTone => {
		if (!health) return "muted";
		if (health.broken_ref_count === 0) return "success";
		if (health.broken_ref_count <= 3) return "warning";
		return "destructive";
	});

	const connectivityTone = $derived.by((): MetricTone => {
		if (overallStatus === "green") return "success";
		if (overallStatus === "amber") return "warning";
		if (overallStatus === "red") return "destructive";
		return "muted";
	});

	// --- Delta helpers (higher = better unless noted) ---

	/**
	 * Format the numeric difference between two health metric values for display.
	 * @param current - The current snapshot value of the metric.
	 * @param previous - The previous snapshot value, or undefined if no history exists.
	 * @returns A signed string like "+2.3" or "-1.0", or an empty string when the delta is negligible.
	 */
	function fmtDeltaNum(current: number, previous: number | undefined): string {
		if (previous === undefined) return "";
		const diff = current - previous;
		if (Math.abs(diff) < 0.005) return "";
		const sign = diff > 0 ? "+" : "";
		return `${sign}${fmt(diff, 1)}`;
	}

	/**
	 * Format the percentage-point difference between two ratio values for display.
	 * @param currentRatio - The current snapshot ratio (0–1).
	 * @param previousRatio - The previous snapshot ratio, or undefined if no history exists.
	 * @returns A signed percentage string like "+5%" or "-3%", or an empty string when unchanged.
	 */
	function fmtDeltaPct(currentRatio: number, previousRatio: number | undefined): string {
		if (previousRatio === undefined) return "";
		const diff = Math.round(currentRatio * 100) - Math.round(previousRatio * 100);
		if (diff === 0) return "";
		const sign = diff > 0 ? "+" : "";
		return `${sign}${diff}%`;
	}

	// Returns the tone for a delta annotation: success when the change is good, destructive otherwise.
	/**
	 * Determine the semantic tone for a delta annotation based on direction and metric polarity.
	 * @param diff - The formatted delta string (e.g. "+5%"); an empty string means no change.
	 * @param higherIsBetter - True when an increase in the metric is desirable (e.g. connectivity).
	 * @returns "success" or "destructive" based on whether the change is favourable, or null if no change.
	 */
	function deltaTone(diff: string, higherIsBetter: boolean): "success" | "destructive" | null {
		if (!diff) return null;
		const positive = diff.startsWith("+");
		const good = higherIsBetter ? positive : !positive;
		return good ? "success" : "destructive";
	}

	// Pre-compute all delta strings and their tone values as derived state.
	const outlierDelta = $derived(
		fmtDeltaNum(
			health?.outlier_count ?? 0,
			// HealthSnapshot does not yet track outlier_count; delta is unavailable.
			undefined,
		),
	);
	const outlierDeltaTone = $derived(deltaTone(outlierDelta, false));

	const connectivityDelta = $derived(
		fmtDeltaPct(
			health?.largest_component_ratio ?? 0,
			prevSnapshot?.largest_component_ratio ?? undefined,
		),
	);
	const connectivityDeltaTone = $derived(deltaTone(connectivityDelta, true));

	const degreeDelta = $derived(
		fmtDeltaNum(health?.avg_degree ?? 0, prevSnapshot?.avg_degree ?? undefined),
	);
	const degreeDeltaTone = $derived(deltaTone(degreeDelta, true));

	const brokenRefDelta = $derived(
		fmtDeltaNum(health?.broken_ref_count ?? 0, prevSnapshot?.broken_ref_count ?? undefined),
	);
	const brokenRefDeltaTone = $derived(deltaTone(brokenRefDelta, false));

	// HealthSnapshot does not yet track pipeline connectivity fields; deltas unavailable.
	const deliveryDelta = $derived(fmtDeltaPct(health?.delivery_connectivity ?? 0, undefined));
	const deliveryDeltaTone = $derived(deltaTone(deliveryDelta, true));

	const learningDelta = $derived(fmtDeltaPct(health?.learning_connectivity ?? 0, undefined));
	const learningDeltaTone = $derived(deltaTone(learningDelta, true));

	const traceabilityDelta = $derived(
		fmtDeltaNum(health?.pillar_traceability ?? 0, prevSnapshot?.pillar_traceability ?? undefined),
	);
	const traceabilityDeltaTone = $derived(deltaTone(traceabilityDelta, true));

	const prevDate = $derived(
		prevSnapshot ? new Date(prevSnapshot.created_at).toLocaleDateString() : null,
	);
</script>

<ScrollArea full>
	<Stack gap={0}>
		<!-- Panel header -->
		<SectionHeader>
			{#snippet start()}
				<HStack gap={1}>
					<Icon name="activity" size="sm" />
					<Caption>Graph Health</Caption>
				</HStack>
			{/snippet}
			{#snippet end()}
				<HStack gap={2}>
					{#if loading}
						<LoadingSpinner size="sm" />
					{:else if health && health.total_nodes > 0}
						<Caption>{healthScore}%</Caption>
						<Dot color={overallDotColor} size="sm" />
					{/if}
					<Button
						variant="ghost"
						size="icon-sm"
						onclick={onRefresh}
						disabled={loading}
						aria-label="Refresh health metrics"
					>
						<Icon name="refresh-cw" size="sm" />
					</Button>
				</HStack>
			{/snippet}
		</SectionHeader>

		{#if loading && !health}
			<Center>
				<LoadingSpinner size="md" />
			</Center>
		{:else if !health || health.total_nodes === 0}
			<Center flex={1}>
				<Panel padding="normal">
					<Stack gap={2} align="center">
						<Icon name="activity" size="md" />
						<Caption>No graph data yet.</Caption>
						<Caption>Open a project to analyse health.</Caption>
					</Stack>
				</Panel>
			</Center>
		{:else}
			<Stack gap={0}>
				<!-- Size overview -->
				<Panel padding="tight">
					<Caption>Overview</Caption>
					<Grid cols={2} gap={1.5}>
						<HStack justify="between">
							<Caption tone="muted">Nodes</Caption>
							<Text variant="caption-tabular">{health.total_nodes}</Text>
						</HStack>
						<HStack justify="between">
							<Caption tone="muted">Edges</Caption>
							<Text variant="caption-tabular">{health.total_edges}</Text>
						</HStack>
					</Grid>
				</Panel>

				<!-- Connectivity metrics -->
				<Panel padding="tight">
					<Caption>Connectivity</Caption>
					<Stack gap={1}>
						<!-- Outliers -->
						<TooltipRoot delayDuration={300}>
							<TooltipTrigger variant="metric-row">
								<HStack justify="between">
									<HStack gap={1}>
										<Icon name="unlink" size="xs" />
										<Caption tone="muted">Outliers</Caption>
									</HStack>
									<HStack gap={1}>
										<Caption variant="caption-tabular" tone={outlierTone}
											>{health.outlier_count}</Caption
										>
										{#if outlierDelta}<Caption tone={outlierDeltaTone ?? undefined}
												>{outlierDelta}</Caption
											>{/if}
									</HStack>
								</HStack>
							</TooltipTrigger>
							<TooltipContent side="left">
								<Caption variant="caption-strong">Pipeline Outliers</Caption>
								<Caption
									>Active artifacts outside both the delivery pipeline and the learning pipeline.
									Excludes archived, surpassed, knowledge, and doc artifacts.</Caption
								>
								{#if health.outlier_age_distribution.stale > 0}
									<Caption tone="destructive"
										>{health.outlier_age_distribution.stale} stale (30d+) — priority.</Caption
									>
								{/if}
								{#if health.outlier_age_distribution.aging > 0}
									<Caption tone="warning"
										>{health.outlier_age_distribution.aging} aging (7–30d).</Caption
									>
								{/if}
								{#if health.outlier_age_distribution.fresh > 0}
									<Caption>{health.outlier_age_distribution.fresh} fresh (≤7d).</Caption>
								{/if}
							</TooltipContent>
						</TooltipRoot>

						<!-- Outlier age distribution sub-row -->
						{#if showOutlierAgeDistribution}
							<HStack justify="end" gap={2}>
								{#if health.outlier_age_distribution.stale > 0}
									<Caption tone="destructive">{health.outlier_age_distribution.stale} stale</Caption
									>
								{/if}
								{#if health.outlier_age_distribution.aging > 0}
									<Caption tone="warning">{health.outlier_age_distribution.aging} aging</Caption>
								{/if}
								{#if health.outlier_age_distribution.fresh > 0}
									<Caption>{health.outlier_age_distribution.fresh} fresh</Caption>
								{/if}
							</HStack>
						{/if}

						<!-- Connectivity score -->
						<TooltipRoot delayDuration={300}>
							<TooltipTrigger variant="metric-row">
								<HStack justify="between">
									<HStack gap={1}>
										<Icon name="link" size="xs" />
										<Caption tone="muted">Connectivity</Caption>
									</HStack>
									<HStack gap={1}>
										<Caption variant="caption-tabular" tone={connectivityTone}
											>{healthScore}%</Caption
										>
										{#if connectivityDelta}<Caption tone={connectivityDeltaTone ?? undefined}
												>{connectivityDelta}</Caption
											>{/if}
									</HStack>
								</HStack>
							</TooltipTrigger>
							<TooltipContent side="left">
								<Caption variant="caption-strong">Largest Component Ratio</Caption>
								<Caption
									>Percentage of artifacts in the largest connected group. Target: 90%+.</Caption
								>
							</TooltipContent>
						</TooltipRoot>

						<!-- Avg Degree -->
						<TooltipRoot delayDuration={300}>
							<TooltipTrigger variant="metric-row">
								<HStack justify="between">
									<HStack gap={1}>
										<Icon name="git-branch" size="xs" />
										<Caption tone="muted">Avg Degree</Caption>
									</HStack>
									<HStack gap={1}>
										<Caption variant="caption-tabular" tone={degreeTone}
											>{health.avg_degree}</Caption
										>
										{#if degreeDelta}<Caption tone={degreeDeltaTone ?? undefined}
												>{degreeDelta}</Caption
											>{/if}
									</HStack>
								</HStack>
							</TooltipTrigger>
							<TooltipContent side="left">
								<Caption variant="caption-strong">Average Connection Degree</Caption>
								<Caption
									>Average relationships per artifact. Target: 4+ for a well-connected graph.</Caption
								>
							</TooltipContent>
						</TooltipRoot>

						<!-- Broken refs -->
						<TooltipRoot delayDuration={300}>
							<TooltipTrigger variant="metric-row">
								<HStack justify="between">
									<HStack gap={1}>
										<Icon name="link-2-off" size="xs" />
										<Caption tone="muted">Broken Refs</Caption>
									</HStack>
									<HStack gap={1}>
										<Caption variant="caption-tabular" tone={brokenRefTone}
											>{health.broken_ref_count}</Caption
										>
										{#if brokenRefDelta}<Caption tone={brokenRefDeltaTone ?? undefined}
												>{brokenRefDelta}</Caption
											>{/if}
									</HStack>
								</HStack>
							</TooltipTrigger>
							<TooltipContent side="left">
								<Caption variant="caption-strong">Broken References</Caption>
								<Caption
									>References whose target artifact does not exist in the graph. Target: 0.</Caption
								>
							</TooltipContent>
						</TooltipRoot>
					</Stack>
				</Panel>

				<!-- Pipeline metrics -->
				<Panel padding="tight">
					<Caption>Pipelines</Caption>
					<Stack gap={1}>
						<!-- Delivery pipeline -->
						<TooltipRoot delayDuration={300}>
							<TooltipTrigger variant="metric-row">
								<HStack justify="between">
									<HStack gap={1}>
										<Icon name="package" size="xs" />
										<Caption tone="muted">Delivery</Caption>
									</HStack>
									<HStack gap={1}>
										<Caption variant="caption-tabular" tone={deliveryTone}
											>{pct(health.delivery_connectivity)}%</Caption
										>
										{#if deliveryDelta}<Caption tone={deliveryDeltaTone ?? undefined}
												>{deliveryDelta}</Caption
											>{/if}
									</HStack>
								</HStack>
							</TooltipTrigger>
							<TooltipContent side="left">
								<Caption variant="caption-strong">Delivery Pipeline Connectivity</Caption>
								<Caption
									>% of delivery artifacts (task, epic, milestone, idea, research, decision,
									wireframe) in the main delivery component. Target: 90%+.</Caption
								>
							</TooltipContent>
						</TooltipRoot>

						<!-- Learning pipeline -->
						<TooltipRoot delayDuration={300}>
							<TooltipTrigger variant="metric-row">
								<HStack justify="between">
									<HStack gap={1}>
										<Icon name="book-open" size="xs" />
										<Caption tone="muted">Learning</Caption>
									</HStack>
									<HStack gap={1}>
										<Caption variant="caption-tabular" tone={learningTone}
											>{pct(health.learning_connectivity)}%</Caption
										>
										{#if learningDelta}<Caption tone={learningDeltaTone ?? undefined}
												>{learningDelta}</Caption
											>{/if}
									</HStack>
								</HStack>
							</TooltipTrigger>
							<TooltipContent side="left">
								<Caption variant="caption-strong">Learning Loop Connectivity</Caption>
								<Caption
									>% of learning artifacts (lesson, rule) connected to each other or to decisions.
									Target: 80%+.</Caption
								>
							</TooltipContent>
						</TooltipRoot>
					</Stack>
				</Panel>

				<!-- Governance metrics -->
				<Panel padding="tight">
					<Caption>Governance</Caption>
					<Stack gap={1}>
						<!-- Pillar Traceability -->
						<TooltipRoot delayDuration={300}>
							<TooltipTrigger variant="metric-row">
								<HStack justify="between">
									<HStack gap={1}>
										<Icon name="target" size="xs" />
										<Caption tone="muted">Traceability</Caption>
									</HStack>
									<HStack gap={1}>
										<Caption variant="caption-tabular" tone={traceabilityTone}
											>{health.pillar_traceability}%</Caption
										>
										{#if traceabilityDelta}<Caption tone={traceabilityDeltaTone ?? undefined}
												>{traceabilityDelta}</Caption
											>{/if}
									</HStack>
								</HStack>
							</TooltipTrigger>
							<TooltipContent side="left">
								<Caption variant="caption-strong">Pillar Traceability</Caption>
								<Caption
									>% of rules grounded by at least one pillar via a grounded-by relationship.
									Target: 80%+.</Caption
								>
							</TooltipContent>
						</TooltipRoot>
					</Stack>
				</Panel>

				<!-- Historical comparison note -->
				{#if snapshots.length > 1 && prevDate}
					<Panel padding="tight">
						<Caption>
							Deltas vs. previous snapshot ({prevDate})
						</Caption>
					</Panel>
				{/if}
			</Stack>
		{/if}
	</Stack>
</ScrollArea>
