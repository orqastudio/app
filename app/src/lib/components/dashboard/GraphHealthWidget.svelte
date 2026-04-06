<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardAction, LoadingSpinner, Button, HStack, Stack, Text, Caption, Grid } from "@orqastudio/svelte-components/pure";
	import { Panel } from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import type { IntegrityCheck, GraphHealthData } from "@orqastudio/types";
	import { fmt, pct } from "@orqastudio/sdk";

	interface Props {
		checks: IntegrityCheck[];
		loading: boolean;
		fixing?: boolean;
		scanned: boolean;
		graphHealth: GraphHealthData | null;
		onScan: () => void;
		onAutoFix?: () => void;
	}

	const { checks, loading, fixing = false, scanned, graphHealth, onScan, onAutoFix }: Props = $props();

	// Score: percentage of graph in the largest connected component.
	const healthScore = $derived(
		graphHealth ? pct(graphHealth.largest_component_ratio) : "0"
	);

	// Traffic light based on largest_component_ratio thresholds.
	type HealthStatus = "green" | "amber" | "red" | "empty";
	const status = $derived.by((): HealthStatus => {
		if (!graphHealth || graphHealth.total_nodes === 0) return "empty";
		if (graphHealth.largest_component_ratio > 0.9) return "green";
		if (graphHealth.largest_component_ratio > 0.7) return "amber";
		return "red";
	});

	const circleClass = $derived.by(() => {
		if (status === "green") return "bg-success";
		if (status === "amber") return "bg-warning";
		if (status === "red") return "bg-destructive";
		return "bg-muted-foreground/30";
	});

	const scoreLabel = $derived.by(() => {
		if (!graphHealth || graphHealth.total_nodes === 0) return "—";
		return `${healthScore}%`;
	});

	// Integrity scan counters (complementary to graph metrics).
	const errorCount = $derived(checks.filter((c) => c.severity === "Error").length);
	const warningCount = $derived(checks.filter((c) => c.severity === "Warning").length);
	const fixableCount = $derived(checks.filter((c) => c.auto_fixable).length);

	// Outlier severity: green 0, amber 1-3, red >3
	type MetricTone = "success" | "warning" | "destructive" | "muted";
	const outlierTone = $derived.by((): MetricTone => {
		if (!graphHealth) return "muted";
		if (graphHealth.outlier_count === 0) return "success";
		if (graphHealth.outlier_count <= 3) return "warning";
		return "destructive";
	});

	// Outlier age distribution label (e.g. "2 stale, 1 aging")
	const outlierAgeSummary = $derived.by(() => {
		if (!graphHealth || graphHealth.outlier_count === 0) return null;
		const d = graphHealth.outlier_age_distribution;
		const parts: string[] = [];
		if (d.stale > 0) parts.push(`${d.stale} stale`);
		if (d.aging > 0) parts.push(`${d.aging} aging`);
		if (d.fresh > 0) parts.push(`${d.fresh} fresh`);
		return parts.length > 0 ? parts.join(", ") : null;
	});

	// Avg degree: green >=4, amber 2-3, red <2 (text-primary not a valid tone; map to success as closest)
	const degreeTone = $derived.by((): MetricTone => {
		if (!graphHealth) return "muted";
		if (graphHealth.avg_degree >= 3) return "success";
		if (graphHealth.avg_degree >= 2) return "warning";
		return "destructive";
	});

	// Delivery connectivity: green >=90%, amber 70-90%, red <70%
	const deliveryTone = $derived.by((): MetricTone => {
		if (!graphHealth) return "muted";
		if (graphHealth.delivery_connectivity >= 0.9) return "success";
		if (graphHealth.delivery_connectivity >= 0.7) return "warning";
		return "destructive";
	});

	// Learning connectivity: green >=80%, amber 50-80%, red <50%
	const learningTone = $derived.by((): MetricTone => {
		if (!graphHealth) return "muted";
		if (graphHealth.learning_connectivity >= 0.8) return "success";
		if (graphHealth.learning_connectivity >= 0.5) return "warning";
		return "destructive";
	});

	// Pillar traceability: green >=80%, amber 50-80%, red <50%
	const traceabilityTone = $derived.by((): MetricTone => {
		if (!graphHealth) return "muted";
		if (graphHealth.pillar_traceability >= 80) return "success";
		if (graphHealth.pillar_traceability >= 50) return "warning";
		return "destructive";
	});

	// ---------------------------------------------------------------------------
	// Threshold alerts
	// ---------------------------------------------------------------------------

	type AlertLevel = "amber" | "red";

	interface ThresholdAlert {
		level: AlertLevel;
		message: string;
	}

	const thresholdAlerts = $derived.by((): ThresholdAlert[] => {
		if (!graphHealth || graphHealth.total_nodes === 0) return [];

		const alerts: ThresholdAlert[] = [];

		if (graphHealth.outlier_count > 0) {
			alerts.push({
				level: graphHealth.outlier_count > 3 ? "red" : "amber",
				message: `${graphHealth.outlier_count} outlier artifact${graphHealth.outlier_count !== 1 ? "s" : ""} outside both pipelines`,
			});
		}

		if (graphHealth.delivery_connectivity < 0.9) {
			alerts.push({
				level: "amber",
				message: `Delivery pipeline ${pct(graphHealth.delivery_connectivity)}% connected (target: 90%)`,
			});
		}

		if (graphHealth.pillar_traceability < 90) {
			alerts.push({
				level: "red",
				message: `Pillar traceability ${fmt(graphHealth.pillar_traceability)}% — below 90% target`,
			});
		}

		return alerts;
	});
</script>

<CardRoot gap={2} full>
	<CardHeader compact>
		<CardTitle size="sm">
			<HStack gap={1}>
				<Icon name="eye" size="md" />
				Clarity
			</HStack>
		</CardTitle>
		<CardDescription size="xs">Where You Are</CardDescription>
		<CardAction>
			{#if loading}
				<LoadingSpinner size="sm" />
			{:else}
				<HStack gap={2}>
					<Text variant="body-strong">{scoreLabel}</Text>
					<span aria-hidden="true" style="position: relative; display: flex; height: 0.75rem; width: 0.75rem; flex-shrink: 0; align-items: center; justify-content: center;">
						<span class="absolute h-3 w-3 rounded-full {circleClass} opacity-30"></span>
						<span class="h-1.5 w-1.5 rounded-full {circleClass}"></span>
					</span>
				</HStack>
			{/if}
		</CardAction>
	</CardHeader>
	<CardContent compact>
		<Stack gap={3}>
		{#if graphHealth && graphHealth.total_nodes > 0}
			<Grid cols={2} gap={2}>
				<!-- Outliers -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="unlink" size="sm" />
						<Caption variant="caption-tabular" tone={outlierTone}>
							{graphHealth.outlier_count}
						</Caption>
						<Caption>Outlier{graphHealth.outlier_count !== 1 ? "s" : ""}</Caption>
						{#if outlierAgeSummary}
							<Caption tone="muted">{outlierAgeSummary}</Caption>
						{/if}
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<Text variant="body-strong" block>Pipeline Outliers</Text>
						<Text variant="body-muted" block>Active artifacts outside both the delivery pipeline (task / epic / milestone / idea / research / decision / wireframe) and the learning pipeline (lesson / rule). Outliers need attention — connect them or archive them.</Text>
						{#if graphHealth.outlier_age_distribution.stale > 0}
							<Text variant="body" tone="destructive" block>{graphHealth.outlier_age_distribution.stale} stale (90d+ or no date) — priority action.</Text>
						{/if}
						{#if graphHealth.outlier_age_distribution.aging > 0}
							<Text variant="body" tone="warning" block>{graphHealth.outlier_age_distribution.aging} aging (30–90d) — connect or archive soon.</Text>
						{/if}
						{#if graphHealth.outlier_age_distribution.fresh > 0}
							<Text variant="body-muted" block>{graphHealth.outlier_age_distribution.fresh} fresh (≤30d) — within grace period.</Text>
						{/if}
					</TooltipContent>
				</TooltipRoot>

				<!-- Avg Degree -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="git-branch" size="sm" />
						<Caption variant="caption-tabular" tone={degreeTone}>{fmt(graphHealth.avg_degree)}</Caption>
						<Caption>Avg Degree</Caption>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<Text variant="body-strong" block>Average Connection Degree</Text>
						<Text variant="body-muted" block>The average number of relationships per artifact. Higher means a more interconnected knowledge graph. A well-connected graph has an average degree of 4+ — each artifact relates to multiple others.</Text>
					</TooltipContent>
				</TooltipRoot>

				<!-- Delivery Pipeline -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="package" size="sm" />
						<Caption variant="caption-tabular" tone={deliveryTone}>
							{pct(graphHealth.delivery_connectivity)}%
						</Caption>
						<Caption>Delivery</Caption>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<Text variant="body-strong" block>Delivery Pipeline Connectivity</Text>
						<Text variant="body-muted" block>Percentage of delivery artifacts (task, epic, milestone, idea, research, decision, wireframe) connected in the main delivery component. Target: 90%+.</Text>
					</TooltipContent>
				</TooltipRoot>

				<!-- Learning Pipeline -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="book-open" size="sm" />
						<Caption variant="caption-tabular" tone={learningTone}>
							{pct(graphHealth.learning_connectivity)}%
						</Caption>
						<Caption>Learning</Caption>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<Text variant="body-strong" block>Learning Loop Connectivity</Text>
						<Text variant="body-muted" block>Percentage of learning artifacts (lesson, rule) connected to each other or to decisions. Disconnected lessons and rules are not feeding back into the delivery process. Target: 80%+.</Text>
					</TooltipContent>
				</TooltipRoot>

				<!-- Scan Results -->
				{#if scanned}
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
							{#if errorCount > 0}
								<Icon name="circle-alert" size="sm" />
								<Caption variant="caption-tabular" tone="destructive">{errorCount}E / {warningCount}W</Caption>
							{:else if warningCount > 0}
								<Icon name="triangle-alert" size="sm" />
								<Caption variant="caption-tabular" tone="warning">{warningCount}W</Caption>
							{:else}
								<Icon name="circle-alert" size="sm" />
								<Caption tone="success">Clean</Caption>
							{/if}
							<Caption>Integrity</Caption>
						</TooltipTrigger>
						<TooltipContent side="bottom">
							<Text variant="body-strong" block>Integrity Scan Results</Text>
							<Text variant="body-muted" block>File-level checks: broken references, invalid statuses, missing required fields, schema violations. Errors must be fixed. Warnings indicate potential issues. Use Auto-fix for machine-fixable problems.</Text>
						</TooltipContent>
					</TooltipRoot>
				{:else}
					<Panel padding="normal">
					<Stack gap={1} align="center">
						<Icon name="scan" size="sm" />
						<Caption variant="caption-tabular">—</Caption>
						<Caption>Integrity</Caption>
					</Stack>
					</Panel>
				{/if}

				<!-- Pillar Traceability -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="target" size="sm" />
						<Caption variant="caption-tabular" tone={traceabilityTone}>
							{fmt(graphHealth.pillar_traceability)}%
						</Caption>
						<Caption>Traceability</Caption>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<Text variant="body-strong" block>Pillar Traceability</Text>
						<Text variant="body-muted" block>Percentage of rules that are grounded by at least one pillar via a grounded-by relationship. Rules without pillar grounding are unanchored — they enforce something with no stated rationale.</Text>
					</TooltipContent>
				</TooltipRoot>

			</Grid>
		{/if}

		<!-- Threshold alerts -->
		{#if thresholdAlerts.length > 0}
			<Stack gap={1}>
				{#each thresholdAlerts as alert (alert.message)}
					<Panel padding="tight">
					<HStack gap={1}>
						<Icon name={alert.level === "red" ? "circle-alert" : "triangle-alert"} size="sm" />
						<Caption tone={alert.level === "red" ? "destructive" : "warning"}>{alert.message}</Caption>
					</HStack>
					</Panel>
				{/each}
			</Stack>
		{/if}

		<!-- Scan and Auto-fix action buttons -->
		<Grid cols={2} gap={2}>
			<Button variant="outline" size="sm" onclick={onScan} disabled={loading || fixing}>
				{#if loading}
					<LoadingSpinner size="sm" />
				{:else}
					<Icon name="scan" size="sm" />
				{/if}
				Scan
			</Button>
			<Button variant="outline" size="sm" onclick={onAutoFix} disabled={loading || fixing || !scanned || fixableCount === 0 || !onAutoFix}>
				{#if fixing}
					<LoadingSpinner size="sm" />
				{:else}
					<Icon name="wrench" size="sm" />
				{/if}
				Auto-fix{scanned && fixableCount > 0 ? ` (${fixableCount})` : ""}
			</Button>
		</Grid>
		</Stack>
	</CardContent>
</CardRoot>
