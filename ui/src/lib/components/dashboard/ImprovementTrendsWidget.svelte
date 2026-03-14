<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import type { HealthSnapshot } from "$lib/types/artifact-graph";

	let snapshots = $state<HealthSnapshot[]>([]);
	let loading = $state(false);
	let loaded = $state(false);

	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !loaded && !loading) {
			void loadSnapshots();
		}
	});

	async function loadSnapshots() {
		loading = true;
		try {
			snapshots = await artifactGraphSDK.getHealthSnapshots(20);
			loaded = true;
		} catch {
			// Non-critical widget — silently degrade
			loaded = true;
		} finally {
			loading = false;
		}
	}

	// Reverse so oldest is first (left-to-right sparkline)
	const chronological = $derived([...snapshots].reverse());

	// Derived values from the most recent snapshot
	const latest = $derived(snapshots[0] ?? null);
	const previous = $derived(snapshots[1] ?? null);
	const hasTrend = $derived(snapshots.length >= 2);

	/** Compute integrity score (0–100) from a snapshot. */
	function integrityScore(s: HealthSnapshot): number {
		if (s.node_count === 0) return 100;
		const healthy = Math.max(0, s.node_count - s.orphan_count - s.broken_ref_count);
		return Math.round((healthy / s.node_count) * 100);
	}

	const SPARKLINE_WIDTH = 80;
	const SPARKLINE_HEIGHT = 32;

	/** Build an SVG polyline path from an array of values. */
	function sparklinePath(values: number[], invert: boolean = false): string {
		if (values.length < 2) return "";
		const min = Math.min(...values);
		const max = Math.max(...values);
		const range = max - min || 1;
		const pad = 2;
		const h = SPARKLINE_HEIGHT - pad * 2;
		const stepX = SPARKLINE_WIDTH / (values.length - 1);
		const points = values.map((v, i) => {
			// If invert=true (lower=better), we flip so visually "good" is up
			const normalised = invert ? 1 - (v - min) / range : (v - min) / range;
			return `${i * stepX},${pad + h - normalised * h}`;
		});
		return `M${points.join(" L")}`;
	}

	interface MetricConfig {
		label: string;
		lowerIsBetter: boolean;
		getValue: (s: HealthSnapshot) => number;
		unit?: string;
	}

	const metrics: MetricConfig[] = [
		{
			label: "Errors",
			lowerIsBetter: true,
			getValue: (s) => s.error_count,
		},
		{
			label: "Warnings",
			lowerIsBetter: true,
			getValue: (s) => s.warning_count,
		},
		{
			label: "Artifacts",
			lowerIsBetter: false,
			getValue: (s) => s.node_count,
		},
		{
			label: "Integrity",
			lowerIsBetter: false,
			getValue: (s) => integrityScore(s),
			unit: "%",
		},
	];

	function currentValue(m: MetricConfig): number {
		return latest ? m.getValue(latest) : 0;
	}

	function percentChange(m: MetricConfig): number | null {
		if (!hasTrend || !latest || !previous) return null;
		const curr = m.getValue(latest);
		const prev = m.getValue(previous);
		if (prev === 0) {
			if (curr === 0) return 0;
			return 100;
		}
		return Math.round(((curr - prev) / prev) * 100);
	}

	/** Is this change considered an improvement? */
	function isImprovement(m: MetricConfig, pct: number): boolean {
		return m.lowerIsBetter ? pct < 0 : pct > 0;
	}

	function trendArrow(m: MetricConfig): string {
		const pct = percentChange(m);
		if (pct === null || pct === 0) return "";
		return pct > 0 ? "↑" : "↓";
	}

	function trendLabel(m: MetricConfig): string {
		const pct = percentChange(m);
		if (pct === null) return "";
		if (pct === 0) return "0%";
		const sign = pct > 0 ? "+" : "";
		return `${sign}${pct}%`;
	}

	function trendColorClass(m: MetricConfig): string {
		const pct = percentChange(m);
		if (pct === null || pct === 0) return "text-muted-foreground";
		return isImprovement(m, pct) ? "text-green-500" : "text-destructive";
	}

	/** Good colour for the sparkline stroke (reflects direction of "good"). */
	function strokeColor(m: MetricConfig): string {
		const pct = percentChange(m);
		if (pct === null || pct === 0) return "#6b7280";
		return isImprovement(m, pct) ? "#22c55e" : "#ef4444";
	}

	function sparklineValues(m: MetricConfig): number[] {
		return chronological.map((s) => m.getValue(s));
	}
</script>

{#if loaded}
	<Card.Root>
		<Card.Header class="pb-3">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					<TrendingUpIcon class="h-4 w-4 text-muted-foreground" />
					Improvement Trends
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content>
			{#if loading}
				<div class="flex items-center justify-center py-4">
					<LoadingSpinner />
				</div>
			{:else if !hasTrend}
				<!-- Single-snapshot or no data state -->
				<div class="grid grid-cols-2 gap-4">
					{#each metrics as m (m.label)}
						<div class="space-y-1">
							<span class="text-xs text-muted-foreground">{m.label}</span>
							<div class="flex items-baseline gap-1">
								<span class="text-lg font-semibold tabular-nums">
									{currentValue(m)}{m.unit ?? ""}
								</span>
							</div>
							<p class="text-[10px] text-muted-foreground">No trend data yet</p>
						</div>
					{/each}
				</div>
			{:else}
				<!-- 2×2 sparkline grid -->
				<div class="grid grid-cols-2 gap-4">
					{#each metrics as m (m.label)}
						{@const values = sparklineValues(m)}
						{@const arrow = trendArrow(m)}
						{@const label = trendLabel(m)}
						{@const colorClass = trendColorClass(m)}
						{@const stroke = strokeColor(m)}
						{@const path = sparklinePath(values, m.lowerIsBetter)}
						<div class="space-y-1">
							<!-- Metric name + current value + trend -->
							<div class="flex items-center justify-between">
								<span class="text-xs text-muted-foreground">{m.label}</span>
								{#if arrow}
									<span class="text-[10px] font-medium {colorClass}">
										{arrow} {label}
									</span>
								{/if}
							</div>
							<div class="text-base font-semibold tabular-nums">
								{currentValue(m)}{m.unit ?? ""}
							</div>
							<!-- Sparkline -->
							{#if path}
								<svg
									width={SPARKLINE_WIDTH}
									height={SPARKLINE_HEIGHT}
									viewBox="0 0 {SPARKLINE_WIDTH} {SPARKLINE_HEIGHT}"
									fill="none"
									xmlns="http://www.w3.org/2000/svg"
									class="w-full"
								>
									<!-- Area fill -->
									<path
										d="{path} L{SPARKLINE_WIDTH},{SPARKLINE_HEIGHT - 2} L0,{SPARKLINE_HEIGHT - 2} Z"
										fill={stroke}
										fill-opacity="0.1"
									/>
									<!-- Line -->
									<path
										d={path}
										stroke={stroke}
										stroke-width="1.5"
										stroke-linecap="round"
										stroke-linejoin="round"
									/>
								</svg>
							{/if}
						</div>
					{/each}
				</div>
				<p class="mt-3 text-[10px] text-muted-foreground">
					Based on {snapshots.length} scan{snapshots.length !== 1 ? "s" : ""}
				</p>
			{/if}
		</Card.Content>
	</Card.Root>
{/if}
