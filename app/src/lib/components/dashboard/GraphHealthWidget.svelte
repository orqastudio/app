<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardAction, LoadingSpinner } from "@orqastudio/svelte-components/pure";
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
	const outlierSeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.outlier_count === 0) return "text-success";
		if (graphHealth.outlier_count <= 3) return "text-warning";
		return "text-destructive";
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

	// Avg degree: green >=4, cyan 3-4, amber 2-3, red <2
	const degreeSeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.avg_degree >= 4) return "text-success";
		if (graphHealth.avg_degree >= 3) return "text-primary";
		if (graphHealth.avg_degree >= 2) return "text-warning";
		return "text-destructive";
	});

	// Delivery connectivity: green >=90%, amber 70-90%, red <70%
	const deliverySeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.delivery_connectivity >= 0.9) return "text-success";
		if (graphHealth.delivery_connectivity >= 0.7) return "text-warning";
		return "text-destructive";
	});

	// Learning connectivity: green >=80%, amber 50-80%, red <50%
	const learningSeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.learning_connectivity >= 0.8) return "text-success";
		if (graphHealth.learning_connectivity >= 0.5) return "text-warning";
		return "text-destructive";
	});

	// Pillar traceability: green >=80%, amber 50-80%, red <50%
	const traceabilitySeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.pillar_traceability >= 80) return "text-success";
		if (graphHealth.pillar_traceability >= 50) return "text-warning";
		return "text-destructive";
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
			<div class="flex items-center gap-1">
				<Icon name="eye" size="md" />
				Clarity
			</div>
		</CardTitle>
		<CardDescription size="xs">Where You Are</CardDescription>
		<CardAction>
			{#if loading}
				<LoadingSpinner size="sm" />
			{:else}
				<div class="flex items-center gap-2">
					<span class="text-sm font-semibold tabular-nums">{scoreLabel}</span>
					<span class="relative flex h-3 w-3 shrink-0 items-center justify-center">
						<span class="absolute h-3 w-3 rounded-full {circleClass} opacity-30"></span>
						<span class="h-1.5 w-1.5 rounded-full {circleClass}"></span>
					</span>
				</div>
			{/if}
		</CardAction>
	</CardHeader>
	<CardContent compact>
		<div class="flex flex-col gap-3">
		{#if graphHealth && graphHealth.total_nodes > 0}
			<div class="grid grid-cols-2 gap-2 flex-1 text-center text-xs">
				<!-- Outliers -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="unlink" size="sm" />
						<span class="{outlierSeverity} font-semibold tabular-nums">
							{graphHealth.outlier_count}
						</span>
						<span class="text-muted-foreground">Outlier{graphHealth.outlier_count !== 1 ? "s" : ""}</span>
						{#if outlierAgeSummary}
							<span class="text-muted-foreground/70 text-[10px] leading-tight">{outlierAgeSummary}</span>
						{/if}
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<p class="font-medium mb-1">Pipeline Outliers</p>
						<p class="text-muted-foreground">Active artifacts outside both the delivery pipeline (task / epic / milestone / idea / research / decision / wireframe) and the learning pipeline (lesson / rule). Outliers need attention — connect them or archive them.</p>
						{#if graphHealth.outlier_age_distribution.stale > 0}
							<p class="text-destructive mt-1">{graphHealth.outlier_age_distribution.stale} stale (90d+ or no date) — priority action.</p>
						{/if}
						{#if graphHealth.outlier_age_distribution.aging > 0}
							<p class="text-warning mt-1">{graphHealth.outlier_age_distribution.aging} aging (30–90d) — connect or archive soon.</p>
						{/if}
						{#if graphHealth.outlier_age_distribution.fresh > 0}
							<p class="text-muted-foreground mt-1">{graphHealth.outlier_age_distribution.fresh} fresh (≤30d) — within grace period.</p>
						{/if}
					</TooltipContent>
				</TooltipRoot>

				<!-- Avg Degree -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="git-branch" size="sm" />
						<span class="{degreeSeverity} font-semibold tabular-nums">{fmt(graphHealth.avg_degree)}</span>
						<span class="text-muted-foreground">Avg Degree</span>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<p class="font-medium mb-1">Average Connection Degree</p>
						<p class="text-muted-foreground">The average number of relationships per artifact. Higher means a more interconnected knowledge graph. A well-connected graph has an average degree of 4+ — each artifact relates to multiple others.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Delivery Pipeline -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="package" size="sm" />
						<span class="{deliverySeverity} font-semibold tabular-nums">
							{pct(graphHealth.delivery_connectivity)}%
						</span>
						<span class="text-muted-foreground">Delivery</span>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<p class="font-medium mb-1">Delivery Pipeline Connectivity</p>
						<p class="text-muted-foreground">Percentage of delivery artifacts (task, epic, milestone, idea, research, decision, wireframe) connected in the main delivery component. Target: 90%+.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Learning Pipeline -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="book-open" size="sm" />
						<span class="{learningSeverity} font-semibold tabular-nums">
							{pct(graphHealth.learning_connectivity)}%
						</span>
						<span class="text-muted-foreground">Learning</span>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<p class="font-medium mb-1">Learning Loop Connectivity</p>
						<p class="text-muted-foreground">Percentage of learning artifacts (lesson, rule) connected to each other or to decisions. Disconnected lessons and rules are not feeding back into the delivery process. Target: 80%+.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Scan Results -->
				{#if scanned}
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
							{#if errorCount > 0}
								<Icon name="circle-alert" size="sm" />
								<span class="text-destructive font-semibold tabular-nums">{errorCount}E / {warningCount}W</span>
							{:else if warningCount > 0}
								<Icon name="triangle-alert" size="sm" />
								<span class="text-warning font-semibold tabular-nums">{warningCount}W</span>
							{:else}
								<Icon name="circle-alert" size="sm" />
								<span class="text-success font-semibold">Clean</span>
							{/if}
							<span class="text-muted-foreground">Integrity</span>
						</TooltipTrigger>
						<TooltipContent side="bottom">
							<p class="font-medium mb-1">Integrity Scan Results</p>
							<p class="text-muted-foreground">File-level checks: broken references, invalid statuses, missing required fields, schema violations. Errors must be fixed. Warnings indicate potential issues. Use Auto-fix for machine-fixable problems.</p>
						</TooltipContent>
					</TooltipRoot>
				{:else}
					<div class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 text-muted-foreground">
						<Icon name="scan" size="sm" />
						<span class="tabular-nums">—</span>
						<span>Integrity</span>
					</div>
				{/if}

				<!-- Pillar Traceability -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="target" size="sm" />
						<span class="{traceabilitySeverity} font-semibold tabular-nums">
							{fmt(graphHealth.pillar_traceability)}%
						</span>
						<span class="text-muted-foreground">Traceability</span>
					</TooltipTrigger>
					<TooltipContent side="bottom">
						<p class="font-medium mb-1">Pillar Traceability</p>
						<p class="text-muted-foreground">Percentage of rules that are grounded by at least one pillar via a grounded-by relationship. Rules without pillar grounding are unanchored — they enforce something with no stated rationale.</p>
					</TooltipContent>
				</TooltipRoot>

			</div>
		{/if}

		<!-- Threshold alerts -->
		{#if thresholdAlerts.length > 0}
			<div class="flex flex-col gap-1">
				{#each thresholdAlerts as alert (alert.message)}
					<div
						class="flex items-center gap-1 rounded px-2 py-1 text-xs {alert.level === 'red'
							? 'bg-destructive/10 text-destructive'
							: 'bg-warning/10 text-warning'}"
					>
						<Icon name={alert.level === "red" ? "circle-alert" : "triangle-alert"} size="sm" />
						<span>{alert.message}</span>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Actions -->
		<div class="grid grid-cols-2 gap-2 mt-auto">
			<button class="flex items-center justify-center gap-1 rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent disabled:opacity-50" onclick={onScan} disabled={loading || fixing}>
				{#if loading}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<Icon name="scan" size="sm" />
				{/if}
				Scan
			</button>
			<button class="flex items-center justify-center gap-1 rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent disabled:opacity-50" onclick={onAutoFix} disabled={loading || fixing || !scanned || fixableCount === 0 || !onAutoFix}>
				{#if fixing}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<Icon name="wrench" size="sm" />
				{/if}
				Auto-fix{scanned && fixableCount > 0 ? ` (${fixableCount})` : ""}
			</button>
		</div>
		</div>
	</CardContent>
</CardRoot>
