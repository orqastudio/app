<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import type { HealthSnapshot } from "$lib/types/artifact-graph";

	let snapshots = $state<HealthSnapshot[]>([]);
	let loading = $state(false);
	let loaded = $state(false);

	// Auto-load snapshots when the graph is ready
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
		} finally {
			loading = false;
		}
	}

	// Reverse so oldest is first (for left-to-right sparkline)
	const chronological = $derived([...snapshots].reverse());

	interface SparklineConfig {
		label: string;
		key: keyof HealthSnapshot;
		color: string;
		strokeColor: string;
	}

	const sparklines: SparklineConfig[] = [
		{ label: "Errors", key: "error_count", color: "text-destructive", strokeColor: "#ef4444" },
		{ label: "Warnings", key: "warning_count", color: "text-warning", strokeColor: "#f59e0b" },
		{ label: "Orphans", key: "orphan_count", color: "text-muted-foreground", strokeColor: "#6b7280" },
		{ label: "Broken Refs", key: "broken_ref_count", color: "text-muted-foreground", strokeColor: "#6b7280" },
	];

	const SPARKLINE_WIDTH = 120;
	const SPARKLINE_HEIGHT = 80;

	function sparklinePath(data: HealthSnapshot[], key: keyof HealthSnapshot, width: number, height: number): string {
		if (data.length < 2) return "";
		const values = data.map((s) => Number(s[key]));
		const max = Math.max(...values, 1); // At least 1 to avoid division by zero
		const padding = 4; // Vertical padding so line doesn't touch edges
		const usableHeight = height - padding * 2;
		const stepX = width / (values.length - 1);
		const points = values.map((v, i) => `${i * stepX},${padding + usableHeight - (v / max) * usableHeight}`);
		return `M${points.join(" L")}`;
	}

	/** Compute the max value for the y-axis scale label. */
	function maxValue(key: keyof HealthSnapshot): number {
		if (chronological.length === 0) return 0;
		const values = chronological.map((s) => Number(s[key]));
		return Math.max(...values, 1);
	}

	function latestValue(key: keyof HealthSnapshot): number {
		if (snapshots.length === 0) return 0;
		return Number(snapshots[0][key]);
	}

	function trendPercent(key: keyof HealthSnapshot): number | null {
		if (snapshots.length < 2) return null;
		const current = Number(snapshots[0][key]);
		const previous = Number(snapshots[1][key]);
		if (previous === 0) {
			if (current === 0) return 0;
			return 100; // Went from 0 to something
		}
		return Math.round(((current - previous) / previous) * 100);
	}

	function trendIndicator(key: keyof HealthSnapshot): string {
		const pct = trendPercent(key);
		if (pct === null) return "";
		if (pct === 0) return "0%";
		const sign = pct > 0 ? "+" : "";
		return `${sign}${pct}%`;
	}

	function trendArrow(key: keyof HealthSnapshot): string {
		const pct = trendPercent(key);
		if (pct === null || pct === 0) return "";
		return pct > 0 ? "\u2191" : "\u2193";
	}

	function trendColor(key: keyof HealthSnapshot): string {
		const pct = trendPercent(key);
		if (pct === null || pct === 0) return "text-muted-foreground";
		// For these metrics, lower is better
		if (pct < 0) return "text-green-500";
		return "text-destructive";
	}
</script>

{#if loaded && snapshots.length >= 2}
	<Card.Root>
		<Card.Header class="pb-3">
			<Card.Title class="text-base">
				<div class="flex items-center gap-2">
					<TrendingUpIcon class="h-4 w-4 text-muted-foreground" />
					Health Trends
				</div>
			</Card.Title>
		</Card.Header>
		<Card.Content>
			{#if loading}
				<div class="flex items-center justify-center py-4">
					<LoadingSpinner />
				</div>
			{:else}
				<div class="grid grid-cols-2 gap-6">
					{#each sparklines as config (config.key)}
						<div class="space-y-1">
							<!-- Header: label + latest value -->
							<div class="flex items-baseline justify-between">
								<span class="text-xs text-muted-foreground">{config.label}</span>
								<div class="flex items-center gap-1.5">
									<span class="text-lg font-semibold tabular-nums {config.color}">
										{latestValue(config.key)}
									</span>
									{#if trendPercent(config.key) !== null}
										<span class="text-xs font-medium {trendColor(config.key)}">
											{trendArrow(config.key)} {trendIndicator(config.key)}
										</span>
									{/if}
								</div>
							</div>
							<!-- Sparkline with y-axis scale -->
							<div class="flex items-start gap-1">
								<div class="flex flex-col justify-between text-[9px] tabular-nums text-muted-foreground/60" style="height: {SPARKLINE_HEIGHT}px;">
									<span>{maxValue(config.key)}</span>
									<span>0</span>
								</div>
								<svg
									width={SPARKLINE_WIDTH}
									height={SPARKLINE_HEIGHT}
									viewBox="0 0 {SPARKLINE_WIDTH} {SPARKLINE_HEIGHT}"
									class="shrink-0"
									fill="none"
									xmlns="http://www.w3.org/2000/svg"
								>
									<!-- Faint baseline at y=0 -->
									<line
										x1="0"
										y1={SPARKLINE_HEIGHT - 4}
										x2={SPARKLINE_WIDTH}
										y2={SPARKLINE_HEIGHT - 4}
										stroke="currentColor"
										stroke-width="0.5"
										class="text-muted-foreground/20"
									/>
									<!-- Area fill under the sparkline -->
									{@const pathD = sparklinePath(chronological, config.key, SPARKLINE_WIDTH, SPARKLINE_HEIGHT)}
									{#if pathD}
										<path
											d="{pathD} L{SPARKLINE_WIDTH},{SPARKLINE_HEIGHT - 4} L0,{SPARKLINE_HEIGHT - 4} Z"
											fill={config.strokeColor}
											fill-opacity="0.08"
										/>
										<path
											d={pathD}
											stroke={config.strokeColor}
											stroke-width="1.5"
											stroke-linecap="round"
											stroke-linejoin="round"
											fill="none"
										/>
									{/if}
								</svg>
							</div>
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
