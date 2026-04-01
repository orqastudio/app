<!-- MetricsView: dashboard grid of performance metrics for OrqaDev.
     Displays four categories (graph build, prompt generation, IPC durations,
     search index) sourced from perf-level events via the metrics store.
     Clicking a metric cell expands a TimingChart below the grid.
     Also shows an error-rate sparkline covering the last 30 minutes. -->
<script lang="ts">
	import { DashboardCard, MetricCell, Sparkline } from "@orqastudio/svelte-components/pure";
	import {
		metrics,
		METRIC_CATEGORIES,
		errorRateHistory,
		totalErrorsInWindow,
		ingestEvent,
	} from "../../stores/metrics-store.svelte.js";
	import TimingChart from "./TimingChart.svelte";
	import { onMount, onDestroy } from "svelte";

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

	// Demo mode: inject synthetic perf events so the view renders real data
	// while the log store integration is not yet wired up. Each interval tick
	// fires one event per category with a plausible random duration.
	let demoTimer: ReturnType<typeof setInterval> | null = null;

	// Pseudo-random base durations per category (milliseconds).
	const DEMO_BASE: Record<string, number> = {
		graph_build: 45,
		prompt_gen: 120,
		ipc: 8,
		search_index: 30,
	};

	/** Fire one synthetic perf event per category plus occasional error events. */
	function fireDemoEvents(): void {
		for (const cat of CATEGORY_KEYS) {
			const base = DEMO_BASE[cat] ?? 20;
			// ±30% jitter around the base value.
			const jitter = (Math.random() - 0.5) * 0.6 * base;
			ingestEvent({
				level: "perf",
				category: cat,
				durationMs: Math.max(1, Math.round(base + jitter)),
				timestamp: Date.now(),
			});
		}
		// Occasionally inject an error so the error rate panel shows activity.
		if (Math.random() < 0.15) {
			ingestEvent({ level: "error", category: "ipc", timestamp: Date.now() });
		}
	}

	onMount(() => {
		// Seed with a batch of events so sparklines render immediately.
		for (let i = 0; i < 20; i++) fireDemoEvents();
		demoTimer = setInterval(fireDemoEvents, 2000);
	});

	onDestroy(() => {
		if (demoTimer !== null) clearInterval(demoTimer);
	});
</script>

<!-- Full-height scrollable content area. -->
<div class="flex h-full flex-col gap-4 overflow-y-auto p-4">

	<!-- Section heading -->
	<div class="flex items-center justify-between">
		<span class="text-sm font-semibold text-content-base">Performance Metrics</span>
		<span class="text-xs text-content-muted">{metrics.totalEvents} events processed</span>
	</div>

	<!-- Four metric cards in a responsive 2×2 grid. -->
	<div class="grid grid-cols-2 gap-3">
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
							<div class="h-8 text-xs text-content-muted flex items-center">Waiting…</div>
						{/if}
						<!-- Min / avg / max row below the sparkline. -->
						<div class="mt-1 flex justify-between text-[10px] text-content-muted tabular-nums">
							<span>min {fmtMs(stats.min)}</span>
							<span>avg {fmtMs(stats.avg)}</span>
							<span>max {fmtMs(stats.max)}</span>
						</div>
					</MetricCell>
				</DashboardCard>
			</div>
		{/each}
	</div>

	<!-- Expanded timing chart — shown below the grid when a category is selected. -->
	{#if selectedCategory && metrics.byCategory[selectedCategory]}
		<div class="rounded-lg border border-border bg-surface-raised p-4">
			<TimingChart stats={metrics.byCategory[selectedCategory]} width={560} height={100} />
		</div>
	{/if}

	<!-- Error rate panel: errors-per-minute sparkline over the last 30 minutes. -->
	<DashboardCard title="Error Rate" description="Errors per minute — last 30 minutes">
		<div class="flex flex-col gap-2">
			<div class="flex items-baseline justify-between">
				<span class="text-xs text-content-muted">Total in window</span>
				<span
					class="text-lg font-semibold tabular-nums
					       {errTotal > 0 ? 'text-destructive' : 'text-success'}"
				>
					{errTotal}
				</span>
			</div>
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
				<div class="h-12 text-xs text-content-muted flex items-center">No error data yet</div>
			{/if}
			<div class="flex justify-between text-[10px] text-content-muted">
				<span>−30 min</span>
				<span>now</span>
			</div>
		</div>
	</DashboardCard>

</div>
