<!-- TimingChart renders a labelled line chart for a single metric category's
     timing history. The chart is an inline SVG polyline — no external deps.
     Used by MetricsView to show the detailed distribution view when a metric
     cell is selected. -->
<script lang="ts">
	import { sparklinePath } from "@orqastudio/svelte-components/pure";
	import type { MetricStats } from "../../stores/metrics-store.svelte.js";

	let {
		stats,
		width = 320,
		height = 80,
	}: {
		/** Metric statistics from the metrics store. */
		stats: MetricStats;
		width?: number;
		height?: number;
	} = $props();

	const PADDING = 8;
	const LABEL_HEIGHT = 16;
	// Reserve space at bottom for x-axis labels.
	const chartHeight = $derived(height - LABEL_HEIGHT);

	// Generate the SVG path for the timing values.
	const path = $derived(
		sparklinePath(stats.history, width, chartHeight, { padding: PADDING }),
	);

	// Y axis: min and max label values rounded to one decimal.
	const yMin = $derived(stats.min === Infinity ? 0 : Math.round(stats.min * 10) / 10);
	const yMax = $derived(stats.max === -Infinity ? 0 : Math.round(stats.max * 10) / 10);

	// Baseline y coordinate (bottom of the chart area).
	const baselineY = $derived(chartHeight - PADDING);
</script>

<!-- Chart container. Text sizes are set as inline SVG attributes so they are not
     affected by Tailwind's text reset. -->
<div class="flex flex-col gap-1">
	<span class="text-xs font-medium text-content-base">{stats.label}</span>

	{#if stats.history.length < 2}
		<div
			class="flex items-center justify-center rounded bg-surface-raised text-xs text-content-muted"
			style="width:{width}px;height:{height}px"
		>
			Waiting for data…
		</div>
	{:else}
		<svg
			{width}
			height={chartHeight}
			viewBox="0 0 {width} {chartHeight}"
			class="rounded bg-surface-raised"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<!-- Faint baseline -->
			<line
				x1="0"
				y1={baselineY}
				x2={width}
				y2={baselineY}
				stroke="currentColor"
				stroke-width="0.5"
				class="text-muted-foreground/20"
			/>

			<!-- Filled area under the timing line -->
			<path
				d="{path} L{width},{baselineY} L0,{baselineY} Z"
				fill="currentColor"
				fill-opacity="0.08"
				class="text-primary"
			/>

			<!-- Timing line -->
			<path
				d={path}
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="text-primary"
			/>

			<!-- Y-axis min label -->
			<text
				x={PADDING}
				y={baselineY - 3}
				font-size="9"
				fill="currentColor"
				class="text-muted-foreground/60"
				opacity="0.6"
			>{yMin}ms</text>

			<!-- Y-axis max label -->
			<text
				x={PADDING}
				y={PADDING + 9}
				font-size="9"
				fill="currentColor"
				class="text-muted-foreground/60"
				opacity="0.6"
			>{yMax}ms</text>
		</svg>

		<!-- X-axis labels: first sample and last sample count -->
		<div class="flex justify-between text-[10px] text-content-muted" style="width:{width}px">
			<span>−{stats.history.length} samples</span>
			<span>now</span>
		</div>
	{/if}
</div>
