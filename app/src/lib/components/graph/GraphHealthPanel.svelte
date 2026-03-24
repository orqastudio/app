<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import type { GraphHealthData, HealthSnapshot } from "@orqastudio/types";

	interface Props {
		health: GraphHealthData | null;
		snapshots: HealthSnapshot[];
		loading: boolean;
		onRefresh: () => void;
	}

	const { health, snapshots, loading, onRefresh }: Props = $props();

	// Previous snapshot for historical comparison (index 1 = second most recent).
	const prevSnapshot = $derived(snapshots.length > 1 ? snapshots[1] : null);

	// Overall health score: largest connected component ratio as percentage.
	const healthScore = $derived(
		health ? Math.round(health.largest_component_ratio * 100) : 0,
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

	const orphanSeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.orphan_percentage > 15) return "text-destructive";
		if (health.orphan_percentage > 5) return "text-warning";
		return "text-emerald-500";
	});

	const degreeSeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.avg_degree >= 4) return "text-emerald-500";
		if (health.avg_degree >= 3) return "text-cyan-500";
		if (health.avg_degree >= 2) return "text-warning";
		return "text-destructive";
	});

	const densitySeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.graph_density > 0.05) return "text-emerald-500";
		if (health.graph_density > 0.01) return "text-warning";
		return "text-destructive";
	});

	const traceabilitySeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.pillar_traceability >= 80) return "text-emerald-500";
		if (health.pillar_traceability >= 50) return "text-warning";
		return "text-destructive";
	});

	const bidirectionalitySeverity = $derived.by(() => {
		if (!health) return "text-muted-foreground";
		if (health.bidirectionality_ratio >= 0.7) return "text-emerald-500";
		if (health.bidirectionality_ratio >= 0.4) return "text-warning";
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
		return `${sign}${diff.toFixed(1)}`;
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
	const clusterDelta = $derived(fmtDeltaNum(
		health?.component_count ?? 0,
		prevSnapshot?.component_count ?? undefined,
	));
	const clusterDeltaClass = $derived(deltaClass(clusterDelta, false));

	const connectivityDelta = $derived(fmtDeltaPct(
		health?.largest_component_ratio ?? 0,
		prevSnapshot?.largest_component_ratio ?? undefined,
	));
	const connectivityDeltaClass = $derived(deltaClass(connectivityDelta, true));

	const orphanDelta = $derived(fmtDeltaNum(
		health?.orphan_percentage ?? 0,
		prevSnapshot?.orphan_percentage ?? undefined,
	));
	const orphanDeltaClass = $derived(deltaClass(orphanDelta, false));

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

	const densityDelta = $derived(fmtDeltaNum(
		(health?.graph_density ?? 0) * 100,
		prevSnapshot ? prevSnapshot.graph_density * 100 : undefined,
	));
	const densityDeltaClass = $derived(deltaClass(densityDelta, true));

	const bidirectionalDelta = $derived(fmtDeltaPct(
		health?.bidirectionality_ratio ?? 0,
		prevSnapshot?.bidirectionality_ratio ?? undefined,
	));
	const bidirectionalDeltaClass = $derived(deltaClass(bidirectionalDelta, true));

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

					<!-- Clusters -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="network" size="xs" />
								Clusters
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{health.component_count > 1 ? 'text-warning font-semibold' : 'text-emerald-500 font-semibold'} tabular-nums">
									{health.component_count}
								</span>
								<span class={clusterDeltaClass}>{clusterDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Connected Components</p>
							<p class="text-muted-foreground">Disconnected subgraphs. A healthy graph has 1 cluster — all artifacts reachable from each other.</p>
						</TooltipContent>
					</TooltipRoot>

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

					<!-- Orphans -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="unlink" size="xs" />
								Orphans
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{orphanSeverity} font-semibold tabular-nums">
									{health.orphan_count} <span class="font-normal text-muted-foreground">({health.orphan_percentage}%)</span>
								</span>
								<span class={orphanDeltaClass}>{orphanDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Orphaned Artifacts</p>
							<p class="text-muted-foreground">Artifacts with no incoming references. Target: below 5% of total nodes.</p>
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

			<!-- Structure metrics -->
			<div class="px-3 py-2">
				<p class="mb-1.5 text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">Structure</p>
				<div class="flex flex-col gap-1.5 text-xs">

					<!-- Density -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="layers" size="xs" />
								Density
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{densitySeverity} font-semibold tabular-nums">
									{(health.graph_density * 100).toFixed(2)}%
								</span>
								<span class={densityDeltaClass}>{densityDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Graph Density</p>
							<p class="text-muted-foreground">Edge count as a % of maximum possible edges. Governance graphs: 1–5% is healthy.</p>
						</TooltipContent>
					</TooltipRoot>

					<!-- Bidirectionality -->
					<TooltipRoot delayDuration={300}>
						<TooltipTrigger class="flex w-full items-center justify-between rounded px-1 py-0.5 hover:bg-muted/60 transition-colors text-left">
							<span class="flex items-center gap-1 text-muted-foreground">
								<Icon name="arrow-left-right" size="xs" />
								Bidirectional
							</span>
							<div class="flex items-center gap-1.5">
								<span class="{bidirectionalitySeverity} font-semibold tabular-nums">
									{Math.round(health.bidirectionality_ratio * 100)}%
								</span>
								<span class={bidirectionalDeltaClass}>{bidirectionalDelta}</span>
							</div>
						</TooltipTrigger>
						<TooltipContent side="left" class="w-56 text-xs">
							<p class="font-medium mb-1">Bidirectionality Ratio</p>
							<p class="text-muted-foreground">% of typed relationship edges with their inverse present. Missing inverses create navigation asymmetry. Target: 70%+.</p>
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
