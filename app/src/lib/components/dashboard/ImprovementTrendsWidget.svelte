<script lang="ts">
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { getStores, logger } from "@orqastudio/sdk";

	const log = logger("dashboard");
	const { artifactGraphSDK, pluginRegistry } = getStores();
	import type { HealthSnapshot } from "@orqastudio/types";

	let snapshots = $state<HealthSnapshot[]>([]);
	let loading = $state(false);
	let loaded = $state(false);

	/** All governance artifacts with their created dates — derived from plugin registry. */
	const governanceArtifacts = $derived(
		pluginRegistry.governanceSchemas.flatMap((s) => artifactGraphSDK.byType(s.key))
	);

	/**
	 * Counts governance artifacts that existed on or before a given date.
	 * Artifacts without a created date are assumed to have always existed.
	 * @param dateStr - An ISO date string (YYYY-MM-DD) to use as the upper bound.
	 * @returns The count of governance artifacts active on that date.
	 */
	function governanceAtDate(dateStr: string): number {
		return governanceArtifacts.filter((a) => {
			const created = (a.frontmatter as Record<string, unknown>)["created"];
			if (typeof created !== "string") return true; // no date = assume existed
			return created <= dateStr;
		}).length;
	}

	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !loaded && !loading) {
			void loadSnapshots();
		}
	});

	/** Fetches the most recent health snapshots and marks the component as loaded. */
	async function loadSnapshots() {
		loading = true;
		try {
			snapshots = await artifactGraphSDK.getHealthSnapshots(20);
			loaded = true;
		} catch (err) {
			log.warn("Failed to load health snapshots for improvement trends", { err });
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

	/**
	 * Computes a 0–100 integrity score from a health snapshot.
	 * Score reflects the proportion of nodes that are neither orphaned nor broken.
	 * @param s - The health snapshot to score.
	 * @returns An integer percentage from 0 to 100.
	 */
	function integrityScore(s: HealthSnapshot): number {
		if (s.node_count === 0) return 100;
		const healthy = Math.max(0, s.node_count - s.orphan_count - s.broken_ref_count);
		return Math.round((healthy / s.node_count) * 100);
	}

	const SPARKLINE_HEIGHT = 40;

	/**
	 * Builds an SVG polyline path from an array of values.
	 * Values plot naturally: 0 at bottom, max at top.
	 * Color (not line direction) indicates whether the trend is good or bad.
	 * @param values - The numeric data points to plot.
	 * @param fixedMin - Optional fixed minimum for the y-axis scale.
	 * @param fixedMax - Optional fixed maximum for the y-axis scale.
	 * @returns An SVG path `d` attribute string, or empty string if fewer than 2 points.
	 */
	function sparklinePath(values: number[], fixedMin?: number, fixedMax?: number): string {
		if (values.length < 2) return "";
		const min = fixedMin ?? Math.min(...values);
		const max = fixedMax ?? Math.max(...values);
		const range = max - min;
		const pad = 2;
		const h = SPARKLINE_HEIGHT - pad * 2;
		const totalWidth = 100;
		const stepX = totalWidth / (values.length - 1);
		const points = values.map((v, i) => {
			const normalised = range === 0 ? 0.5 : (v - min) / range;
			return `${i * stepX},${pad + h - normalised * h}`;
		});
		return `M${points.join(" L")}`;
	}

	interface MetricConfig {
		label: string;
		lowerIsBetter: boolean;
		getValue: (s: HealthSnapshot) => number;
		unit?: string;
		fixedMin?: number;
		fixedMax?: number;
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
			label: "Governance",
			lowerIsBetter: false,
			getValue: (s) => governanceAtDate(s.created_at),
		},
		{
			label: "Integrity",
			lowerIsBetter: false,
			getValue: (s) => integrityScore(s),
			unit: "%",
			fixedMin: 0,
			fixedMax: 100,
		},
	];

	/**
	 * Returns the current value for a metric from the latest snapshot.
	 * @param m - The metric configuration.
	 * @returns The metric's current value, or 0 if no snapshot is available.
	 */
	function currentValue(m: MetricConfig): number {
		return latest ? m.getValue(latest) : 0;
	}

	/**
	 * Computes the percentage change for a metric between the two most recent snapshots.
	 * @param m - The metric configuration.
	 * @returns The percentage change as an integer, or null if trend data is unavailable.
	 */
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

	/**
	 * Returns true if the percentage change represents an improvement for the metric.
	 * @param m - The metric configuration, which declares whether lower is better.
	 * @param pct - The percentage change value.
	 * @returns Whether the change is an improvement.
	 */
	function isImprovement(m: MetricConfig, pct: number): boolean {
		return m.lowerIsBetter ? pct < 0 : pct > 0;
	}

	/**
	 * Returns an up or down arrow character for a metric's trend direction.
	 * @param m - The metric configuration.
	 * @returns "↑", "↓", or an empty string if no change or no trend.
	 */
	function trendArrow(m: MetricConfig): string {
		const pct = percentChange(m);
		if (pct === null || pct === 0) return "";
		return pct > 0 ? "↑" : "↓";
	}

	/**
	 * Returns a formatted percentage string for a metric's change (e.g. "+12%" or "0%").
	 * @param m - The metric configuration.
	 * @returns A formatted label string, or empty string if no trend data.
	 */
	function trendLabel(m: MetricConfig): string {
		const pct = percentChange(m);
		if (pct === null) return "";
		if (pct === 0) return "0%";
		const sign = pct > 0 ? "+" : "";
		return `${sign}${pct}%`;
	}

	/**
	 * Returns a Tailwind text color class for a metric's trend, reflecting improvement vs regression.
	 * @param m - The metric configuration.
	 * @returns A Tailwind text color class string.
	 */
	function trendColorClass(m: MetricConfig): string {
		const pct = percentChange(m);
		if (pct === null || pct === 0) return "text-muted-foreground";
		return isImprovement(m, pct) ? "text-green-500" : "text-destructive";
	}

	/**
	 * Returns the sparkline stroke color based on the overall first-to-last trend.
	 * Contextually positive changes render green; negative render red; flat renders cyan.
	 * @param m - The metric configuration.
	 * @returns A hex color string.
	 */
	function strokeColor(m: MetricConfig): string {
		const values = sparklineValues(m);
		if (values.length < 2) return "#06b6d4";
		const first = values[0];
		const last = values[values.length - 1];
		const diff = last - first;
		if (diff === 0) return "#06b6d4";
		const improving = m.lowerIsBetter ? diff < 0 : diff > 0;
		return improving ? "#22c55e" : "#ef4444";
	}

	/** Generate last 7 days as ISO date strings. */
	const last7Days = $derived.by((): string[] => {
		const nowMs = Date.now();
		return Array.from({ length: 7 }, (_, i) => {
			const ms = nowMs - (6 - i) * 86_400_000;
			return new Date(ms).toISOString().slice(0, 10);
		});
	});

	/** Filter snapshots to last 7 days only. */
	const recentSnapshots = $derived(
		chronological.filter((s) => {
			const date = s.created_at.slice(0, 10);
			return date >= last7Days[0];
		})
	);

	/**
	 * Computes the data points to plot for a metric's sparkline.
	 * Governance metrics use daily time points; other metrics use recent snapshots.
	 * @param m - The metric configuration.
	 * @returns An array of numeric values in chronological order.
	 */
	function sparklineValues(m: MetricConfig): number[] {
		if (m.label === "Governance") {
			// Use daily time points, not snapshots
			return last7Days.map((date) => governanceAtDate(date));
		}
		// For snapshot-based metrics, use recent snapshots only
		return recentSnapshots.map((s) => m.getValue(s));
	}


</script>

{#if loaded}
	<!--
		Single card containing a 2x2 grid of trend cells.
		Card title / description are injected by ProjectDashboard.
		Dividers between cells via border-r / border-b on each cell.
	-->
	<div class="grid h-full grid-cols-2 grid-rows-2 overflow-hidden">
		{#each metrics as m, idx (m.label)}
			{@const values = sparklineValues(m)}
			{@const arrow = trendArrow(m)}
			{@const label = trendLabel(m)}
			{@const colorClass = trendColorClass(m)}
			{@const stroke = strokeColor(m)}
			{@const path = hasTrend ? sparklinePath(values, m.fixedMin, m.fixedMax) : ""}
			{@const isLeft = idx % 2 === 0}
			{@const isTop = idx < 2}
			<div class="flex min-h-0 flex-col overflow-hidden {isLeft && 'border-r border-border'} {isTop && 'border-t border-border'}">
				<!-- Metric header -->
				<div class="flex items-center justify-between px-3 pt-3 pb-1">
					<span class="text-xs font-medium text-muted-foreground">{m.label}</span>
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
				<!-- Sparkline — flush to cell edges -->
				{#if loading}
					<div class="flex items-center justify-center py-3">
						<LoadingSpinner size="sm" />
					</div>
				{:else if path}
					<svg
						class="flex-1 w-full min-h-0"
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
					<div class="px-3 pb-3">
						<p class="text-[10px] text-muted-foreground">No trend data yet</p>
					</div>
				{/if}
			</div>
		{/each}
	</div>
{/if}
