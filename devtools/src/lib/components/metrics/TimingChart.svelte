<!-- TimingChart renders a labelled line chart for a single metric category's
     timing history. The chart is an inline SVG polyline — no external deps.
     Used by MetricsView to show the detailed distribution view when a metric
     cell is selected. -->
<script lang="ts">
	import { sparklinePath, Stack, Text, Caption } from "@orqastudio/svelte-components/pure";
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
	// X-axis labels are rendered inside the SVG so no HTML overlay is needed.
	// Reserve space at the bottom of the SVG for the label row.
	const chartHeight = $derived(height - LABEL_HEIGHT);

	// Generate the SVG path for the timing values.
	const path = $derived(sparklinePath(stats.history, width, chartHeight, { padding: PADDING }));

	// Y axis: min and max label values rounded to one decimal.
	const yMin = $derived(stats.min === Infinity ? 0 : Math.round(stats.min * 10) / 10);
	const yMax = $derived(stats.max === -Infinity ? 0 : Math.round(stats.max * 10) / 10);

	// Baseline y coordinate (bottom of the chart area).
	const baselineY = $derived(chartHeight - PADDING);
</script>

<!-- Chart container. Text sizes are set as inline SVG attributes so they are not
     affected by Tailwind's text reset. -->
<Stack gap={1}>
	<!-- Label uses body-strong variant (semibold body text) as the closest semantic fit
	     for a chart title — no class prop, variant drives all styling. -->
	<Text variant="body-strong">{stats.label}</Text>

	{#if stats.history.length < 2}
		<!-- Waiting-for-data placeholder: centered caption in a rounded muted box.
		     Dynamic width/height require inline style since Box has no style prop;
		     scoped class provides the background+radius. -->
		<div class="timing-chart__placeholder" style="width:{width}px;height:{height}px">
			<Caption>Waiting for data…</Caption>
		</div>
	{:else}
		<!-- SVG is a legitimate exception. Tailwind class= removed from SVG elements;
		     fill/stroke values use CSS variable references directly. Scoped class
		     provides the rounded background since Box has no style prop for dynamic width.
		     X-axis labels render inside the SVG below the plot area so HTML never has
		     to match the dynamic pixel width of the chart. -->
		<div class="timing-chart__svg-wrapper" style="width:{width}px;">
			<svg
				{width}
				{height}
				viewBox="0 0 {width} {height}"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<!-- Faint baseline — stroke uses CSS variable directly, no Tailwind class. -->
				<line
					x1="0"
					y1={baselineY}
					x2={width}
					y2={baselineY}
					stroke="hsl(var(--muted-foreground) / 0.2)"
					stroke-width="0.5"
				/>

				<!-- Filled area under the timing line — fill uses CSS variable directly. -->
				<path
					d="{path} L{width},{baselineY} L0,{baselineY} Z"
					fill="hsl(var(--primary))"
					fill-opacity="0.08"
				/>

				<!-- Timing line — stroke uses CSS variable directly. -->
				<path
					d={path}
					stroke="hsl(var(--primary))"
					stroke-width="1.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>

				<!-- Y-axis min label — fill uses CSS variable directly. -->
				<text x={PADDING} y={baselineY - 3} font-size="9" fill="hsl(var(--muted-foreground) / 0.6)"
					>{yMin}ms</text
				>

				<!-- Y-axis max label — fill uses CSS variable directly. -->
				<text x={PADDING} y={PADDING + 9} font-size="9" fill="hsl(var(--muted-foreground) / 0.6)"
					>{yMax}ms</text
				>

				<!-- X-axis labels: first/last sample markers, rendered as native SVG
				     text so they pin to the plot's exact pixel width without an HTML
				     overlay. Tabular-nums styling applied via inline style because
				     Svelte's SVG attribute types don't expose font-variant-numeric. -->
				<text
					x={PADDING}
					y={height - 4}
					font-size="10"
					fill="hsl(var(--muted-foreground))"
					text-anchor="start"
					style="font-variant-numeric: tabular-nums;">−{stats.history.length} samples</text
				>
				<text
					x={width - PADDING}
					y={height - 4}
					font-size="10"
					fill="hsl(var(--muted-foreground))"
					text-anchor="end"
					style="font-variant-numeric: tabular-nums;">now</text
				>
			</svg>
		</div>
	{/if}
</Stack>

<style>
	/* Placeholder shown when there are fewer than 2 data points. */
	.timing-chart__placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		background-color: var(--color-surface-raised);
	}

	/* Container for the SVG chart — provides rounded corners and muted background. */
	.timing-chart__svg-wrapper {
		border-radius: var(--radius-md);
		background-color: var(--color-surface-raised);
		overflow: hidden;
	}
</style>
