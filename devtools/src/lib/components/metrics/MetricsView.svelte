<!-- MetricsView: dashboard grid of performance metrics for OrqaDev.
     Displays four categories (graph build, prompt generation, IPC durations,
     search index) sourced from perf-level events via the metrics store.
     Clicking a metric cell expands a TimingChart below the grid.
     Also shows an error-rate sparkline covering the last 30 minutes. -->
<script lang="ts">
	import { DashboardCard, MetricCell, Panel, Sparkline, Stack, HStack, Grid, Heading, Text, Caption, ScrollArea } from "@orqastudio/svelte-components/pure";
	import {
		metrics,
		METRIC_CATEGORIES,
		errorRateHistory,
		totalErrorsInWindow,
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
<ScrollArea full>
<Panel padding="normal">
<Stack gap={4}>

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
			<!-- Scoped div wrapper provides border+selected state styling. Tailwind class=
			     pattern replaced with data-selected attribute + scoped CSS. -->
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="metrics-card"
				data-selected={isSelected}
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
								<!-- Scoped class provides height without Tailwind utility on HStack. -->
								<div class="metrics-card__waiting"><Caption>Waiting…</Caption></div>
							{/if}
							<!-- Min / avg / max row: caption-tabular variant for numeric alignment. -->
							<HStack justify="between">
								<Caption variant="caption-tabular">min {fmtMs(stats.min)}</Caption>
								<Caption variant="caption-tabular">avg {fmtMs(stats.avg)}</Caption>
								<Caption variant="caption-tabular">max {fmtMs(stats.max)}</Caption>
							</HStack>
						</Stack>
					</MetricCell>
				</DashboardCard>
			</div>
		{/each}
	</Grid>

	<!-- Expanded timing chart — shown below the grid when a category is selected. -->
	{#if selectedCategory && metrics.byCategory[selectedCategory]}
		<div class="metrics-chart-card">
			<TimingChart stats={metrics.byCategory[selectedCategory]} width={560} height={100} />
		</div>
	{/if}

	<!-- Error rate panel: errors-per-minute sparkline over the last 30 minutes. -->
	<DashboardCard title="Error Rate" description="Errors per minute — last 30 minutes">
		<Stack gap={2}>
			<HStack justify="between" align="baseline">
				<Caption>Total in window</Caption>
				<!-- tone prop drives color; heading-base provides semibold weight for emphasis. -->
				<Text
					variant="heading-base"
					tone={errTotal > 0 ? "destructive" : "success"}
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
				<!-- Scoped class provides height without Tailwind utility on HStack. -->
				<div class="metrics-card__error-waiting"><Caption>No error data yet</Caption></div>
			{/if}
			<HStack justify="between">
				<Caption>−30 min</Caption>
				<Caption>now</Caption>
			</HStack>
		</Stack>
	</DashboardCard>

</Stack>
</Panel>
</ScrollArea>

<style>
	/* Metric card wrapper: border, rounded corners, pointer cursor, and hover/selected states. */
	.metrics-card {
		cursor: pointer;
		border-radius: var(--radius-lg);
		border: 1px solid var(--color-border);
		transition: border-color 150ms, background-color 150ms;
	}

	.metrics-card:hover {
		border-color: color-mix(in srgb, var(--color-border) 80%, transparent);
		background-color: color-mix(in srgb, var(--color-surface-raised) 50%, transparent);
	}

	/* Selected state: stronger primary-tinted border + raised background. */
	.metrics-card[data-selected="true"] {
		border-color: color-mix(in srgb, var(--color-primary) 40%, transparent);
		background-color: var(--color-surface-raised);
	}

	/* Expanded chart card: raised surface with padding. */
	.metrics-chart-card {
		border-radius: var(--radius-lg);
		border: 1px solid var(--color-border);
		background-color: var(--color-surface-raised);
		padding: var(--spacing-4);
	}

	/* Compact waiting-for-data placeholder within metric cells. */
	.metrics-card__waiting {
		display: flex;
		align-items: center;
		height: 2rem;
	}

	/* Compact waiting state for the error rate panel. */
	.metrics-card__error-waiting {
		display: flex;
		align-items: center;
		height: 3rem;
	}
</style>
