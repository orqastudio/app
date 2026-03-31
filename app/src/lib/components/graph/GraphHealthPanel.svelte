<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
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
	const healthScore = $derived(
		health ? pct(health.largest_component_ratio) : "0",
	);

	type TrafficLight = "green" | "amber" | "red" | "empty";

	const overallStatus = $derived.by((): TrafficLight => {
		if (!health || health.total_nodes === 0) return "empty";
		if (health.largest_component_ratio > 0.9) return "green";
		if (health.largest_component_ratio > 0.7) return "amber";
		return "red";
	});

	const overallDotClass = $derived.by(() => {
		if (overallStatus === "green") return "bg-green-500";
		if (overallStatus === "amber") return "bg-amber-500";
		if (overallStatus === "red") return "bg-destructive";
		return "bg-muted-foreground/30";
	});

	// --- Per-metric severity helpers ---

	// Outlier severity: green 0, amber 1-3, red >3
	const outlierSeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.outlier_count === 0) return "text-emerald-500";
		if (health.outlier_count <= 3) return "text-warning";
		return "text-destructive";
	});

	// Whether to show the age distribution sub-row.
	const showOutlierAgeDistribution = $derived(
		health !== null && health.outlier_count > 0,
	);

	const degreeSeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.avg_degree >= 4) return "text-emerald-500";
		if (health.avg_degree >= 3) return "text-cyan-500";
		if (health.avg_degree >= 2) return "text-warning";
		return "text-destructive";
	});

	// Delivery connectivity: green >=90%, amber 70-90%, red <70%
	const deliverySeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.delivery_connectivity >= 0.9) return "text-emerald-500";
		if (health.delivery_connectivity >= 0.7) return "text-warning";
		return "text-destructive";
	});

	// Learning connectivity: green >=80%, amber 50-80%, red <50%
	const learningSeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.learning_connectivity >= 0.8) return "text-emerald-500";
		if (health.learning_connectivity >= 0.5) return "text-warning";
		return "text-destructive";
	});

	const traceabilitySeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.pillar_traceability >= 80) return "text-emerald-500";
		if (health.pillar_traceability >= 50) return "text-warning";
		return "text-destructive";
	});

	const brokenRefSeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.broken_ref_count === 0) return "text-emerald-500";
		if (health.broken_ref_count <= 3) return "text-warning";
		return "text-destructive";
	});

	const connectivityClass = $derived.by(() => {
		if (overallStatus === "green") return "text-emerald-500";
		if (overallStatus === "amber") return "text-warning";
		if (overallStatus === "red") return "text-destructive";
		return "text-muted-foreground";
	});

	// --- Delta helpers (higher = better unless noted) ---

	function fmtDeltaNum(current: number, previous: number | undefined): string {
		if (previous === undefined) return "";
		const diff = current - previous;
		if (Math.abs(diff) < 0.005) return "";
		const sign = diff > 0 ? "+" : "";
		return `${sign}${fmt(diff, 1)}`;
	}

	function fmtDeltaPct(currentRatio: number, previousRatio: number | undefined): string {
		if (previousRatio === undefined) return "";
		const diff = Math.round(currentRatio * 100) - Math.round(previousRatio * 100);
		if (diff === 0) return "";
		const sign = diff > 0 ? "+" : "";
		return `${sign}${diff}%`;
	}

	function deltaClass(diff: string, higherIsBetter: boolean): string {
		if (!diff) return "hidden";
		const positive = diff.startsWith("+");
		const good = higherIsBetter ? positive : !positive;
		return `text-[10px] ${good ? "text-emerald-500" : "text-destructive"}`;
	}

	// Pre-compute all delta strings and their classes as derived values.
	const outlierDelta = $derived(fmtDeltaNum(
		health?.outlier_count ?? 0,
		// HealthSnapshot does not yet track outlier_count; delta is unavailable.
		undefined,
	));
	const outlierDeltaClass = $derived(deltaClass(outlierDelta, false));

	const connectivityDelta = $derived(fmtDeltaPct(
		health?.largest_component_ratio ?? 0,
		prevSnapshot?.largest_component_ratio ?? undefined,
	));
	const connectivityDeltaClass = $derived(deltaClass(connectivityDelta, true));

	const degreeDelta = $derived(fmtDeltaNum(
		health?.avg_degree ?? 0,
		prevSnapshot?.avg_degree ?? undefined,
	));
	const degreeDeltaClass = $derived(deltaClass(degreeDelta, true));

	const brokenRefDelta = $derived(fmtDeltaNum(
		health?.broken_ref_count ?? 0,
		prevSnapshot?.broken_ref_count ?? undefined,
	));
	const brokenRefDeltaClass = $derived(deltaClass(brokenRefDelta, false));

	// HealthSnapshot does not yet track pipeline connectivity fields; deltas unavailable.
	const deliveryDelta = $derived(fmtDeltaPct(health?.delivery_connectivity ?? 0, undefined));
	const deliveryDeltaClass = $derived(deltaClass(deliveryDelta, true));

	const learningDelta = $derived(fmtDeltaPct(health?.learning_connectivity ?? 0, undefined));
	const learningDeltaClass = $derived(deltaClass(learningDelta, true));

	const traceabilityDelta = $derived(fmtDeltaNum(
		health?.pillar_traceability ?? 0,
		prevSnapshot?.pillar_traceability ?? undefined,
	));
	const traceabilityDeltaClass = $derived(deltaClass(traceabilityDelta, true));

	const prevDate = $derived(
		prevSnapshot ? new Date(prevSnapshot.created_at).toLocaleDateString() : null,
	);
</script>

<div class="flex h-full flex-col overflow-y-auto border-l border-border bg-background">
	<!-- Panel header -->
	<div class="flex items-center justify-between border-b border-border px-3 py-2">
		<div class="flex items-center gap-1.5">
			<Icon name="activity" size="sm" />
			<span class="text-xs font-semibold">Graph Health</span>
		</div>
		<div class="flex items-center gap-2">
			{#if loading}
				<LoadingSpinner size="sm" />
			{:else if health && health.total_nodes > 0}
				<span class="tabular-nums text-xs font-semibold">{healthScore}%</span>
				<span class="relative flex h-2.5 w-2.5 shrink-0 items-center justify-center">
					<span class="absolute h-2.5 w-2.5 rounded-full {overallDotClass} opacity-30"></span>
					<span class="h-1.5 w-1.5 rounded-full {overallDotClass}"></span>
				</span>
			{/if}
			<button
				class="rounded p-0.5 text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
				onclick={onRefresh}
				disabled={loading}
				aria-label="Refresh health metrics"
			>
				<Icon name="refresh-cw" size="sm" />
			</button>
		</div>
	</div>

	{#if loading && !health}
		<div class="flex flex-1 items-center justify-center">
			<LoadingSpinner size="md" />
		</div>
	{:else if !health || health.total_nodes === 0}
		<div class="flex flex-1 flex-col items-center justify-center gap-2 px-4 text-center text-xs text-muted-foreground">
			<Icon name="activity" size="md" />
			<p>No graph data yet.</p>
			<p>Open a project to analyse health.</p>
		</div>
	{:else}
		<div class="flex flex-col divide-y divide-border">

			<!-- Size overview -->
			<div class="px-3 py-2">
				<p class="mb-1.5 text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">Overview</p>
				<div class="grid grid-cols-2 gap-1.5 text-xs">
					<div class="flex items-center justify-between">
						<span class="text-muted-foreground">Nodes</span>
						<span class="font-semibold tabular-nums">{health.total_nodes}</span>
					</div>
					<div class="flex items-center justify-between">
						<span class="text-muted-foreground">Edges</span>
						<span class="font-semibold tabular-nums">{health.total_edges}</span>
					</div>
				</div>
			</div>

			<!-- Connectivity metrics -->
			<div class="px-3 py-2">
				<p class="mb-1.5 text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">Connectivity</p>
				<div class="flex flex-col gap-1.5 text-xs">

					<!-- Outliers -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="unlink" size="xs" />
								Outliers
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{outlierSeverity} font-semibold tabular-nums">
									{health.outlier_count}
								</span>
								<span class={outlierDeltaClass}>{outlierDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Pipeline Outliers</p>
							<p class="text-muted-foreground">Active artifacts outside both the delivery pipeline and the learning pipeline. Excludes archived, surpassed, knowledge, and doc artifacts.</p>
							{#if health.outlier_age_distribution.stale > 0}
								<p class="text-destructive mt-1">{health.outlier_age_distribution.stale} stale (30d+) — priority.</p>
							{/if}
							{#if health.outlier_age_distribution.aging > 0}
								<p class="text-warning mt-1">{health.outlier_age_distribution.aging} aging (7–30d).</p>
							{/if}
							{#if health.outlier_age_distribution.fresh > 0}
								<p class="text-muted-foreground mt-1">{health.outlier_age_distribution.fresh} fresh (≤7d).</p>
							{/if}
						</TooltipContent>
					</TooltipRoot>

					<!-- Outlier age distribution sub-row -->
					{#if showOutlierAgeDistribution}
						<div class="flex items-center justify-end gap-2 px-1 text-[10px] text-muted-foreground/70">
							{#if health.outlier_age_distribution.stale > 0}
								<span class="text-destructive">{health.outlier_age_distribution.stale} stale</span>
							{/if}
							{#if health.outlier_age_distribution.aging > 0}
								<span class="text-warning">{health.outlier_age_distribution.aging} aging</span>
							{/if}
							{#if health.outlier_age_distribution.fresh > 0}
								<span>{health.outlier_age_distribution.fresh} fresh</span>
							{/if}
						</div>
					{/if}

					<!-- Connectivity score -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="link" size="xs" />
								Connectivity
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{connectivityClass} font-semibold tabular-nums">
									{healthScore}%
								</span>
								<span class={connectivityDeltaClass}>{connectivityDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Largest Component Ratio</p>
							<p class="text-muted-foreground">Percentage of artifacts in the largest connected group. Target: 90%+.</p>
						</TooltipContent>
					</TooltipRoot>

					<!-- Avg Degree -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="git-branch" size="xs" />
								Avg Degree
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{degreeSeverity} font-semibold tabular-nums">{health.avg_degree}</span>
								<span class={degreeDeltaClass}>{degreeDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Average Connection Degree</p>
							<p class="text-muted-foreground">Average relationships per artifact. Target: 4+ for a well-connected graph.</p>
						</TooltipContent>
					</TooltipRoot>

					<!-- Broken refs -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="link-2-off" size="xs" />
								Broken Refs
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{brokenRefSeverity} font-semibold tabular-nums">{health.broken_ref_count}</span>
								<span class={brokenRefDeltaClass}>{brokenRefDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Broken References</p>
							<p class="text-muted-foreground">References whose target artifact does not exist in the graph. Target: 0.</p>
						</TooltipContent>
					</TooltipRoot>
				</div>
			</div>

			<!-- Pipeline metrics -->
			<div class="px-3 py-2">
				<p class="mb-1.5 text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">Pipelines</p>
				<div class="flex flex-col gap-1.5 text-xs">

					<!-- Delivery pipeline -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="package" size="xs" />
								Delivery
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{deliverySeverity} font-semibold tabular-nums">
									{pct(health.delivery_connectivity)}%
								</span>
								<span class={deliveryDeltaClass}>{deliveryDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Delivery Pipeline Connectivity</p>
							<p class="text-muted-foreground">% of delivery artifacts (task, epic, milestone, idea, research, decision, wireframe) in the main delivery component. Target: 90%+.</p>
						</TooltipContent>
					</TooltipRoot>

					<!-- Learning pipeline -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="book-open" size="xs" />
								Learning
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{learningSeverity} font-semibold tabular-nums">
									{pct(health.learning_connectivity)}%
								</span>
								<span class={learningDeltaClass}>{learningDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Learning Loop Connectivity</p>
							<p class="text-muted-foreground">% of learning artifacts (lesson, rule) connected to each other or to decisions. Target: 80%+.</p>
						</TooltipContent>
					</TooltipRoot>
				</div>
			</div>

			<!-- Governance metrics -->
			<div class="px-3 py-2">
				<p class="mb-1.5 text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">Governance</p>
				<div class="flex flex-col gap-1.5 text-xs">

					<!-- Pillar Traceability -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="target" size="xs" />
								Traceability
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{traceabilitySeverity} font-semibold tabular-nums">
									{health.pillar_traceability}%
								</span>
								<span class={traceabilityDeltaClass}>{traceabilityDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Pillar Traceability</p>
							<p class="text-muted-foreground">% of rules grounded by at least one pillar via a grounded-by relationship. Target: 80%+.</p>
						</TooltipContent>
					</TooltipRoot>
				</div>
			</div>

			<!-- Historical comparison note -->
			{#if snapshots.length > 1 && prevDate}
				<div class="px-3 py-2">
					<p class="text-[10px] text-muted-foreground">
						Deltas vs. previous snapshot ({prevDate})
					</p>
				</div>
			{/if}

		</div>
	{/if}
</div>
