<!-- MetricsView: dashboard grid of performance metrics for OrqaDev.
     Displays four categories (graph build, prompt generation, IPC durations,
     search index) sourced from perf-level events via the metrics store.
     Clicking a metric cell expands a TimingChart below the grid.
     Also shows an error-rate sparkline covering the last 30 minutes. -->
<script lang="ts">
	import { DashboardCard, MetricCell, Sparkline, Stack, HStack, Grid, Heading, Text, Caption, ScrollArea } from "@orqastudio/svelte-components/pure";
	import {
		metrics,
		METRIC_CATEGORIES,
		errorRateHistory,
		totalErrorsInWindow,
		ingestEvent,
	} from "../../stores/metrics-store.svelte.js";
	import TimingChart from "./TimingChart.svelte";

	// The currently expanded category key (null = none expanded).
	let selectedCategory = $state<string | null>(null);

	/** Toggle a category's detail chart: select it if different, deselect if same. */
	function toggleCategory(cat: string): void {
		selectedCategory = selectedCategory === cat ? null : cat;
	}

	// Ordered list of the four required category keys.
	const CATEGORY_KEYS = Object.keys(METRIC_CATEGORIES);

	/**
	 * Format a millisecond value for display. Shows one decimal for sub-100ms
	 * values and rounds to whole ms above that.
	 */
	function fmtMs(ms: number): string {
		if (ms === 0 || ms === Infinity || ms === -Infinity) return "—";
		return ms < 100 ? `${ms.toFixed(1)}ms` : `${Math.round(ms)}ms`;
	}

	/**
	 * Compute the trend percentage for a metric: compares the most recent value
	 * to the value ten samples ago (or the second value if fewer than ten exist).
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
<ScrollArea class="h-full">
<Stack gap={4} class="p-4">

	<!-- Section heading: title left, event count right. -->
	<HStack justify="between">
		<Heading level={5}>Performance Metrics</Heading>
		<Caption>{metrics.totalEvents} events processed</Caption>
	</HStack>

	<!-- Four metric cards in a responsive 2×2 grid. -->
	<Grid cols={2} gap={3}>
		{#each CATEGORY_KEYS as cat (cat)}
			{@const stats = metrics.byCategory[cat]}
			{@const isSelected = selectedCategory === cat}
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="cursor-pointer rounded-lg border transition-colors
				       {isSelected ? 'border-primary/40 bg-surface-raised' : 'border-border hover:border-border/80 hover:bg-surface-raised/50'}"
				onclick={() => toggleCategory(cat)}
			>
				<DashboardCard>
					<MetricCell
						label={stats.label}
						value={fmtMs(stats.current)}
						trend={computeTrend(stats.history)}
						lowerIsBetter={true}
					>
						<!-- Sparkline of last 100 values — uses the shared Sparkline component. -->
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
							<HStack class="h-8"><Caption>Waiting…</Caption></HStack>
						{/if}
						<!-- Min / avg / max row below the sparkline. -->
						<HStack justify="between" class="mt-1 tabular-nums">
							<Caption>min {fmtMs(stats.min)}</Caption>
							<Caption>avg {fmtMs(stats.avg)}</Caption>
							<Caption>max {fmtMs(stats.max)}</Caption>
						</HStack>
					</MetricCell>
				</DashboardCard>
			</div>
		{/each}
	</Grid>

	<!-- Expanded timing chart — shown below the grid when a category is selected. -->
	{#if selectedCategory && metrics.byCategory[selectedCategory]}
		<div class="rounded-lg border border-border bg-surface-raised p-4">
			<TimingChart stats={metrics.byCategory[selectedCategory]} width={560} height={100} />
		</div>
	{/if}

	<!-- Error rate panel: errors-per-minute sparkline over the last 30 minutes. -->
	<DashboardCard title="Error Rate" description="Errors per minute — last 30 minutes">
		<Stack gap={2}>
			<HStack justify="between" align="baseline">
				<Caption>Total in window</Caption>
				<Text
					size="lg"
					class="font-semibold tabular-nums {errTotal > 0 ? 'text-destructive' : 'text-success'}"
				>
					{errTotal}
				</Text>
			</HStack>
			{#if errHistory.length >= 2}
				<Sparkline
					values={errHistory}
					width={280}
					height={48}
					strokeColor={errTotal > 0 ? "oklch(var(--color-destructive))" : "oklch(var(--color-success))"}
					strokeWidth={1.5}
					fillOpacity={0.1}
					showBaseline={true}
					padding={4}
					fixedMin={0}
				/>
			{:else}
				<HStack class="h-12"><Caption>No error data yet</Caption></HStack>
			{/if}
			<HStack justify="between">
				<Caption>−30 min</Caption>
				<Caption>now</Caption>
			</HStack>
		</Stack>
	</DashboardCard>

</Stack>
</ScrollArea>
