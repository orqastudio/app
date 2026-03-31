<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import type { IntegrityCheck, GraphHealthData } from "@orqastudio/types";

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
		graphHealth ? Math.round(graphHealth.largest_component_ratio * 100) : 0
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
		if (status === "green") return "bg-green-500";
		if (status === "amber") return "bg-amber-500";
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

	// Orphan severity: green <5%, amber 5-15%, red >15%
	const orphanSeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.orphan_percentage > 15) return "text-destructive";
		if (graphHealth.orphan_percentage > 5) return "text-warning";
		return "text-emerald-500";
	});

	// Avg degree: green >=4, cyan 3-4, amber 2-3, red <2
	const degreeSeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.avg_degree >= 4) return "text-emerald-500";
		if (graphHealth.avg_degree >= 3) return "text-cyan-500";
		if (graphHealth.avg_degree >= 2) return "text-warning";
		return "text-destructive";
	});

	// Graph density: green >0.05, amber 0.01-0.05, red <0.01
	const densitySeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.graph_density > 0.05) return "text-emerald-500";
		if (graphHealth.graph_density > 0.01) return "text-warning";
		return "text-destructive";
	});

	// Pillar traceability: green >=80%, amber 50-80%, red <50%
	const traceabilitySeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.pillar_traceability >= 80) return "text-emerald-500";
		if (graphHealth.pillar_traceability >= 50) return "text-warning";
		return "text-destructive";
	});

	// Bidirectionality: green >=0.7, amber 0.4-0.7, red <0.4
	const bidirectionalitySeverity = $derived.by(() => {
		if (!graphHealth) return "text-muted-foreground";
		if (graphHealth.bidirectionality_ratio >= 0.7) return "text-emerald-500";
		if (graphHealth.bidirectionality_ratio >= 0.4) return "text-warning";
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

		if (graphHealth.orphan_percentage > 5) {
			alerts.push({
				level: "amber",
				message: `${graphHealth.orphan_percentage}% orphaned artifacts (threshold: 5%)`,
			});
		}

		if (graphHealth.component_count > 3) {
			alerts.push({
				level: "amber",
				message: `${graphHealth.component_count} disconnected clusters (threshold: 3)`,
			});
		}

		if (graphHealth.pillar_traceability < 90) {
			alerts.push({
				level: "red",
				message: `Pillar traceability ${graphHealth.pillar_traceability}% — below 90% target`,
			});
		}

		if (false && graphHealth.bidirectionality_ratio < 0.8) {
			alerts.push({
				level: "amber",
				message: `Bidirectionality ${Math.round(graphHealth.bidirectionality_ratio * 100)}% — below 80% target`,
			});
		}

		return alerts;
	});
</script>

<CardRoot class="gap-2 flex flex-col h-full">
	<CardHeader class="pb-2">
		<CardTitle class="flex items-center gap-1.5 text-sm font-semibold">
			<Icon name="eye" size="md" />
			Clarity
		</CardTitle>
		<CardDescription class="text-xs">Where You Are</CardDescription>
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
	<CardContent class="flex flex-1 flex-col gap-3 pt-0">
		{#if graphHealth && graphHealth.total_nodes > 0}
			<div class="grid grid-cols-2 gap-2 flex-1 text-center text-xs">
				<!-- Clusters -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="network" size="sm" />
						<span class="{graphHealth.component_count > 1 ? 'text-warning font-semibold' : 'text-muted-foreground'} tabular-nums">
							{graphHealth.component_count}
						</span>
						<span class="text-muted-foreground">Cluster{graphHealth.component_count !== 1 ? "s" : ""}</span>
					</TooltipTrigger>
					<TooltipContent side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Connected Components</p>
						<p class="text-muted-foreground">The number of disconnected subgraphs. A healthy graph has 1 cluster — all artifacts are reachable from each other. Multiple clusters indicate orphaned groups of artifacts that aren't connected to the main knowledge graph.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Orphans -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="unlink" size="sm" />
						<span class="{orphanSeverity} font-semibold tabular-nums">
							{graphHealth.orphan_count} <span class="font-normal">({graphHealth.orphan_percentage}%)</span>
						</span>
						<span class="text-muted-foreground">Orphans</span>
					</TooltipTrigger>
					<TooltipContent side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Orphaned Artifacts</p>
						<p class="text-muted-foreground">Artifacts with no incoming references — nothing points to them. They exist in isolation and won't be discovered through graph traversal. Each orphan should either be connected via relationships or removed if no longer relevant.</p>
					</TooltipContent>
				</TooltipRoot>

				<!-- Avg Degree -->
				<TooltipRoot delayDuration={300}>
					<TooltipTrigger class="flex flex-col items-center justify-center gap-1 rounded-md bg-muted/50 py-3 transition-colors hover:bg-muted/80">
						<Icon name="git-branch" size="sm" />
						<span class="{degreeSeverity} font-semibold tabular-nums">{graphHealth.avg_degree}</span>
						<span class="text-muted-foreground">Avg Degree</span>
					</TooltipTrigger>
					<TooltipContent side="bottom" class="w-64 text-xs">
						<p class="font-medium mb-1">Average Connection Degree</p>
						<p class="text-muted-foreground">The average number of relationships per artifact. Higher means a more interconnected knowledge graph. A well-connected graph has an average degree of 4+ — each artifact relates to multiple others.</p>
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
								<span class="text-emerald-500 font-semibold">Clean</span>
							{/if}
							<span class="text-muted-foreground">Integrity</span>
						</TooltipTrigger>
						<TooltipContent side="bottom" class="w-64 text-xs">
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
							{graphHealth.pillar_traceability}%
						</span>
						<span class="text-muted-foreground">Traceability</span>
					</TooltipTrigger>
					<TooltipContent side="bottom" class="w-64 text-xs">
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
						class="flex items-center gap-1.5 rounded px-2 py-1 text-xs {alert.level === 'red'
							? 'bg-destructive/10 text-destructive'
							: 'bg-amber-500/10 text-amber-600 dark:text-amber-400'}"
					>
						<Icon name={alert.level === "red" ? "circle-alert" : "triangle-alert"} size="sm" />
						<span>{alert.message}</span>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Actions -->
		<div class="grid grid-cols-2 gap-2 mt-auto">
			<Button variant="outline" size="sm" onclick={onScan} disabled={loading || fixing}>
				{#if loading}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<Icon name="scan" size="sm" />
				{/if}
				Scan
			</Button>
			<Button variant="outline" size="sm" onclick={onAutoFix} disabled={loading || fixing || !scanned || fixableCount === 0 || !onAutoFix}>
				{#if fixing}
					<span class="mr-2"><LoadingSpinner size="sm" /></span>
				{:else}
					<Icon name="wrench" size="sm" />
				{/if}
				Auto-fix{scanned && fixableCount > 0 ? ` (${fixableCount})` : ""}
			</Button>
		</div>
	</CardContent>
</CardRoot>
