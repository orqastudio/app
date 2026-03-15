<script lang="ts">
	import * as Card from "$lib/components/ui/card";
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

	const SPARKLINE_HEIGHT = 40;

	/** Build an SVG polyline path from an array of values. */
	function sparklinePath(values: number[], invert: boolean = false): string {
		if (values.length < 2) return "";
		const min = Math.min(...values);
		const max = Math.max(...values);
		const range = max - min || 1;
		const pad = 2;
		const h = SPARKLINE_HEIGHT - pad * 2;
		// We use 100 as viewBox width and scale via preserveAspectRatio
		const totalWidth = 100;
		const stepX = totalWidth / (values.length - 1);
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

	/**
	 * Sparkline stroke colour:
	 * - Contextually positive trend (errors going DOWN, integrity going UP) → green
	 * - Contextually negative trend → red
	 * - No trend / neutral → grey
	 */
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
	<div class="grid grid-cols-2 gap-3">
		{#each metrics as m (m.label)}
			{@const values = sparklineValues(m)}
			{@const arrow = trendArrow(m)}
			{@const label = trendLabel(m)}
			{@const colorClass = trendColorClass(m)}
			{@const stroke = strokeColor(m)}
			{@const path = hasTrend ? sparklinePath(values, m.lowerIsBetter) : ""}
			<Card.Root class="overflow-hidden py-3 gap-1">
				<Card.Header class="pb-1 pt-3 px-4">
					<div class="flex items-center justify-between">
						<Card.Title class="text-xs font-medium text-muted-foreground">
							{m.label}
						</Card.Title>
						<div class="flex items-baseline gap-1.5">
							<span class="text-base font-semibold tabular-nums">
								{currentValue(m)}{m.unit ?? ""}
							</span>
							{#if arrow}
								<span class="text-[10px] font-medium {colorClass}">
									{arrow} {label}
								</span>
							{/if}
						</div>
					</div>
				</Card.Header>
				<Card.Content class="px-0 pb-0 pt-1">
					{#if loading}
						<div class="flex items-center justify-center py-3">
							<LoadingSpinner size="sm" />
						</div>
					{:else if path}
						<!-- Sparkline flush to card edges — no horizontal padding -->
						<svg
							width="100%"
							height={SPARKLINE_HEIGHT}
							viewBox="0 0 100 {SPARKLINE_HEIGHT}"
							preserveAspectRatio="none"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<!-- Area fill -->
							<path
								d="{path} L100,{SPARKLINE_HEIGHT - 2} L0,{SPARKLINE_HEIGHT - 2} Z"
								fill={stroke}
								fill-opacity="0.12"
							/>
							<!-- Line -->
							<path
								d={path}
								stroke={stroke}
								stroke-width="1.5"
								stroke-linecap="round"
								stroke-linejoin="round"
								vector-effect="non-scaling-stroke"
							/>
						</svg>
					{:else}
						<div class="px-4 pb-3">
							<p class="text-[10px] text-muted-foreground">No trend data yet</p>
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		{/each}
	</div>
{/if}
