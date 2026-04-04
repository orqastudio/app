<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardContent, LoadingSpinner, Stack, HStack, Grid, Caption, Text } from "@orqastudio/svelte-components/pure";
	import { getStores, logger } from "@orqastudio/sdk";

	const log = logger("dashboard");
	const { artifactGraphSDK } = getStores();
	import type { HealthSnapshot } from "@orqastudio/types";

	let snapshots = $state<HealthSnapshot[]>([]);
	let loading = $state(false);
	let loaded = $state(false);

	// Auto-load snapshots when the graph is ready
	$effect(() => {
		if (artifactGraphSDK.graph.size > 0 && !loaded && !loading) {
			void loadSnapshots();
		}
	});

	/** Fetches the most recent health snapshots from the graph SDK and stores them. */
	async function loadSnapshots() {
		loading = true;
		try {
			snapshots = await artifactGraphSDK.getHealthSnapshots(20);
			loaded = true;
		} catch (err) {
			log.warn("Failed to load health snapshots", { err });
		} finally {
			loading = false;
		}
	}

	// Reverse so oldest is first (for left-to-right sparkline)
	const chronological = $derived([...snapshots].reverse());

	type SparklineTone = "destructive" | "warning" | "muted";

	interface SparklineConfig {
		readonly label: string;
		readonly key: keyof HealthSnapshot;
		readonly tone: SparklineTone;
		readonly strokeColor: string;
	}

	const sparklines: SparklineConfig[] = [
		{ label: "Errors", key: "error_count", tone: "destructive", strokeColor: "#ef4444" },
		{ label: "Warnings", key: "warning_count", tone: "warning", strokeColor: "#f59e0b" },
		{ label: "Orphans", key: "orphan_count", tone: "muted", strokeColor: "#6b7280" },
		{ label: "Broken Refs", key: "broken_ref_count", tone: "muted", strokeColor: "#6b7280" },
	];

	const SPARKLINE_WIDTH = 120;
	const SPARKLINE_HEIGHT = 80;

	/**
	 * Builds an SVG path string for a sparkline from a sequence of health snapshots.
	 * @param data - The health snapshots to render, in chronological order.
	 * @param key - The snapshot field to plot on the y-axis.
	 * @param width - The total SVG width in pixels.
	 * @param height - The total SVG height in pixels.
	 * @returns An SVG path `d` attribute string, or an empty string if fewer than 2 points.
	 */
	function sparklinePath(data: readonly HealthSnapshot[], key: keyof HealthSnapshot, width: number, height: number): string {
		if (data.length < 2) return "";
		const values = data.map((s) => Number(s[key]));
		const max = Math.max(...values, 1); // At least 1 to avoid division by zero
		const padding = 4; // Vertical padding so line doesn't touch edges
		const usableHeight = height - padding * 2;
		const stepX = width / (values.length - 1);
		const points = values.map((v, i) => `${i * stepX},${padding + usableHeight - (v / max) * usableHeight}`);
		return `M${points.join(" L")}`;
	}

	/**
	 * Computes the maximum value across all chronological snapshots for a given key.
	 * @param key - The snapshot field to compute the max for.
	 * @returns The maximum numeric value, or 0 if no snapshots are available.
	 */
	function maxValue(key: keyof HealthSnapshot): number {
		if (chronological.length === 0) return 0;
		const values = chronological.map((s) => Number(s[key]));
		return Math.max(...values, 1);
	}

	/**
	 * Returns the most recent value for a snapshot field.
	 * @param key - The snapshot field to read.
	 * @returns The latest numeric value, or 0 if no snapshots are available.
	 */
	function latestValue(key: keyof HealthSnapshot): number {
		if (snapshots.length === 0) return 0;
		return Number(snapshots[0][key]);
	}

	/**
	 * Computes the percentage change between the two most recent snapshots for a field.
	 * @param key - The snapshot field to compare.
	 * @returns The percentage change as an integer, or null if fewer than 2 snapshots exist.
	 */
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

	/**
	 * Returns a formatted percentage change string for a snapshot field (e.g. "+12%").
	 * @param key - The snapshot field to format.
	 * @returns A formatted string, or an empty string if no trend data is available.
	 */
	function trendIndicator(key: keyof HealthSnapshot): string {
		const pct = trendPercent(key);
		if (pct === null) return "";
		if (pct === 0) return "0%";
		const sign = pct > 0 ? "+" : "";
		return `${sign}${pct}%`;
	}

	/**
	 * Returns an up or down arrow character for a snapshot field's trend direction.
	 * @param key - The snapshot field to evaluate.
	 * @returns "↑", "↓", or an empty string if no trend or no change.
	 */
	function trendArrow(key: keyof HealthSnapshot): string {
		const pct = trendPercent(key);
		if (pct === null || pct === 0) return "";
		return pct > 0 ? "\u2191" : "\u2193";
	}

	/**
	 * Returns the semantic tone for the trend direction of a snapshot field.
	 * For health metrics, lower values are better so increases render as destructive.
	 * @param key - The snapshot field to evaluate.
	 * @returns A tone string for ORQA Text/Caption components, or null when neutral.
	 */
	function trendTone(key: keyof HealthSnapshot): "success" | "destructive" | "muted" {
		const p = trendPercent(key);
		if (p === null || p === 0) return "muted";
		// For these metrics, lower is better
		if (p < 0) return "success";
		return "destructive";
	}
</script>

{#if loaded && snapshots.length >= 2}
	<CardRoot>
		<CardHeader compact>
			<CardTitle>
				<HStack gap={2}>
					<Icon name="trending-up" size="md" />
					Health Trends
				</HStack>
			</CardTitle>
		</CardHeader>
		<CardContent>
			{#if loading}
				<Stack gap={0} align="center" paddingY={4}>
					<LoadingSpinner />
				</Stack>
			{:else}
				<Grid cols={2} gap={6}>
					{#each sparklines as config (config.key)}
						<Stack gap={1}>
							<!-- Header: label + latest value -->
							<HStack justify="between" align="baseline">
								<Caption>{config.label}</Caption>
								<HStack gap={1} align="baseline">
									<Text variant="heading-base" tone={config.tone}>
										{latestValue(config.key)}
									</Text>
									{#if trendPercent(config.key) !== null}
										<Caption variant="caption-strong" tone={trendTone(config.key)}>
											{trendArrow(config.key)} {trendIndicator(config.key)}
										</Caption>
									{/if}
								</HStack>
							</HStack>
							<!-- Sparkline with y-axis scale — custom SVG chart, wrapped in HStack -->
							<HStack gap={1} align="start">
								<!-- Y-axis scale labels use style for precise height alignment -->
								<div style="display: flex; flex-direction: column; justify-content: space-between; height: {SPARKLINE_HEIGHT}px; font-size: 9px; font-variant-numeric: tabular-nums; color: hsl(var(--muted-foreground) / 0.6);">
									<span>{maxValue(config.key)}</span>
									<span>0</span>
								</div>
								<!-- Custom SVG sparkline — no ORQA primitive fits this shape -->
								<svg
									width={SPARKLINE_WIDTH}
									height={SPARKLINE_HEIGHT}
									viewBox="0 0 {SPARKLINE_WIDTH} {SPARKLINE_HEIGHT}"
									class="shrink-0"
									fill="none"
									xmlns="http://www.w3.org/2000/svg"
								>
									<line
										x1="0"
										y1={SPARKLINE_HEIGHT - 4}
										x2={SPARKLINE_WIDTH}
										y2={SPARKLINE_HEIGHT - 4}
										stroke="hsl(var(--muted-foreground) / 0.2)"
										stroke-width="0.5"
									/>
									{#if sparklinePath(chronological, config.key, SPARKLINE_WIDTH, SPARKLINE_HEIGHT)}
										{@const pathD = sparklinePath(chronological, config.key, SPARKLINE_WIDTH, SPARKLINE_HEIGHT)}
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
							</HStack>
						</Stack>
					{/each}
				</Grid>
				<Stack gap={0} marginTop={3}>
					<Caption>Based on {snapshots.length} scan{snapshots.length !== 1 ? "s" : ""}</Caption>
				</Stack>
			{/if}
		</CardContent>
	</CardRoot>
{/if}
