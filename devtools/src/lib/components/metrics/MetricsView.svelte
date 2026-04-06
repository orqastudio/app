<!-- MetricsView: dashboard grid of performance metrics for OrqaDev.
     Displays four categories (graph build, prompt generation, IPC durations,
     search index) sourced from perf-level events via the metrics store.
     Clicking a metric cell expands a TimingChart below the grid.
     Also shows an error-rate sparkline covering the last 30 minutes. -->
<script lang="ts">
	import {
		CardRoot,
		CardContent,
		MetricCell,
		Panel,
		Sparkline,
		Stack,
		HStack,
		Grid,
		Heading,
		Text,
		Caption,
		Center,
		ScrollArea,
		SurfaceBox,
	} from "@orqastudio/svelte-components/pure";
	import {
		metrics,
		METRIC_CATEGORIES,
		errorRateHistory,
		totalErrorsInWindow,
	} from "../../stores/metrics-store.svelte.js";
	import TimingChart from "./TimingChart.svelte";

	// The currently expanded category key (null = none expanded).
	let selectedCategory = $state<string | null>(null);

	/**
	 * Toggle a category's detail chart: selects the given category or deselects it if already selected.
	 * @param cat - The metric category key to select or deselect.
	 */
	function toggleCategory(cat: string): void {
		selectedCategory = selectedCategory === cat ? null : cat;
	}

	// Ordered list of the four required category keys.
	const CATEGORY_KEYS = Object.keys(METRIC_CATEGORIES);

	/**
	 * Format a millisecond value for display, showing one decimal for sub-100ms values.
	 * @param ms - Duration in milliseconds to format.
	 * @returns Human-readable duration like "12.3ms" or "145ms", or "—" for degenerate values.
	 */
	function fmtMs(ms: number): string {
		if (ms === 0 || ms === Infinity || ms === -Infinity) return "—";
		return ms < 100 ? `${ms.toFixed(1)}ms` : `${Math.round(ms)}ms`;
	}

	/**
	 * Compute the trend percentage for a metric by comparing the most recent value to ten samples ago.
	 * @param history - Array of historical metric values, ordered oldest to newest.
	 * @returns Percentage change rounded to the nearest integer, or null if insufficient data.
	 */
	function computeTrend(history: number[]): number | null {
		if (history.length < 2) return null;
		const recent = history[history.length - 1];
		const reference = history[Math.max(0, history.length - 10)];
		if (reference === 0) return null;
		return Math.round(((recent - reference) / reference) * 100);
	}

	// Derive error-rate data series. Recomputed from the store on every render.
	const errHistory = $derived(errorRateHistory());
	const errTotal = $derived(totalErrorsInWindow());

	// Metrics are populated from real perf-level events ingested by the metrics
	// store. No data is shown until the dev environment is running and emitting
	// perf events through the daemon event bus.
</script>

<!-- Full-height scrollable content area. -->
<ScrollArea full>
	<Panel padding="normal">
		<Stack gap={4}>
			<!-- Section heading: title left, event count right. -->
			<HStack justify="between">
				<Heading level={5}>Performance Metrics</Heading>
				<Caption>{metrics.totalEvents} events processed</Caption>
			</HStack>

			<!-- Four metric cards in a responsive 2×2 grid.
			     CardRoot with interactive+selected replaces the scoped div wrapper + DashboardCard. -->
			<Grid cols={2} gap={3}>
				{#each CATEGORY_KEYS as cat (cat)}
					{@const stats = metrics.byCategory[cat]}
					{@const isSelected = selectedCategory === cat}
					<CardRoot
						interactive={true}
						selected={isSelected}
						gap={2}
						onclick={() => toggleCategory(cat)}
					>
						<CardContent compact={true}>
							<MetricCell
								label={stats.label}
								value={fmtMs(stats.current)}
								trend={computeTrend(stats.history)}
								lowerIsBetter={true}
							>
								<!-- Sparkline of last 100 values — uses the shared Sparkline component. -->
								<Stack gap={1}>
									{#if stats.history.length >= 2}
										<Sparkline
											values={stats.history}
											width={120}
											height={32}
											strokeColor="oklch(var(--color-primary))"
											strokeWidth={1.5}
											fillOpacity={0.1}
											showBaseline={false}
											padding={2}
										/>
									{:else}
										<!-- Centered placeholder while waiting for first data points. -->
										<Center>
											<Caption>Waiting…</Caption>
										</Center>
									{/if}
									<!-- Min / avg / max row: caption-tabular variant for numeric alignment. -->
									<HStack justify="between">
										<Caption variant="caption-tabular">min {fmtMs(stats.min)}</Caption>
										<Caption variant="caption-tabular">avg {fmtMs(stats.avg)}</Caption>
										<Caption variant="caption-tabular">max {fmtMs(stats.max)}</Caption>
									</HStack>
								</Stack>
							</MetricCell>
						</CardContent>
					</CardRoot>
				{/each}
			</Grid>

			<!-- Expanded timing chart: SurfaceBox provides rounded raised background with padding. -->
			{#if selectedCategory && metrics.byCategory[selectedCategory]}
				<SurfaceBox padding="md">
					<TimingChart stats={metrics.byCategory[selectedCategory]} width={560} height={100} />
				</SurfaceBox>
			{/if}

			<!-- Error rate panel: errors-per-minute sparkline over the last 30 minutes. -->
			<CardRoot gap={2}>
				<CardContent>
					<Stack gap={2}>
						<HStack justify="between" align="baseline">
							<Caption>Error Rate</Caption>
							<Caption>Errors per minute — last 30 minutes</Caption>
						</HStack>
						<HStack justify="between" align="baseline">
							<Caption>Total in window</Caption>
							<!-- tone prop drives color; heading-base provides semibold weight for emphasis. -->
							<Text variant="heading-base" tone={errTotal > 0 ? "destructive" : "success"}>
								{errTotal}
							</Text>
						</HStack>
						{#if errHistory.length >= 2}
							<Sparkline
								values={errHistory}
								width={280}
								height={48}
								strokeColor={errTotal > 0
									? "oklch(var(--color-destructive))"
									: "oklch(var(--color-success))"}
								strokeWidth={1.5}
								fillOpacity={0.1}
								showBaseline={true}
								padding={4}
								fixedMin={0}
							/>
						{:else}
							<!-- Centered placeholder while waiting for first error data. -->
							<Center>
								<Caption>No error data yet</Caption>
							</Center>
						{/if}
						<HStack justify="between">
							<Caption>−30 min</Caption>
							<Caption>now</Caption>
						</HStack>
					</Stack>
				</CardContent>
			</CardRoot>
		</Stack>
	</Panel>
</ScrollArea>
