<!-- TimingChartSvg — inline SVG line chart for a metric timing history.
     Renders a filled area chart with a baseline, y-axis labels, and x-axis
     sample count / "now" labels. Used by the devtools MetricsView timing panel.

     All raw SVG attributes and style directives live here in the library so
     that devtools components remain free of raw HTML. -->
<script lang="ts">
	import { sparklinePath } from "./sparkline-utils.js";

	let {
		values,
		width = 320,
		height = 80,
		sampleCount,
	}: {
		/** Timing values to plot in milliseconds (oldest → newest). */
		values: readonly number[];
		/** SVG width in pixels. Must match the parent SurfaceBox widthPx. */
		width?: number;
		/** SVG height in pixels (includes space reserved for x-axis labels). */
		height?: number;
		/** Total number of samples collected (shown in the x-axis left label). */
		sampleCount: number;
	} = $props();

	const PADDING = 8;
	const LABEL_HEIGHT = 16;

	// Reserve space at the bottom for x-axis label row.
	const chartHeight = $derived(height - LABEL_HEIGHT);

	const path = $derived(sparklinePath(values, width, chartHeight, { padding: PADDING }));

	// Y-axis scale labels rounded to one decimal place.
	const yMin = $derived(
		Math.min(...values) === Infinity ? 0 : Math.round(Math.min(...values) * 10) / 10,
	);
	const yMax = $derived(
		Math.max(...values) === -Infinity ? 0 : Math.round(Math.max(...values) * 10) / 10,
	);

	// Baseline y coordinate (bottom of the chart area, above the label row).
	const baselineY = $derived(chartHeight - PADDING);
</script>

<svg {width} {height} viewBox="0 0 {width} {height}" fill="none" xmlns="http://www.w3.org/2000/svg">
	<!-- Faint baseline rule at the bottom of the plot area. -->
	<line
		x1="0"
		y1={baselineY}
		x2={width}
		y2={baselineY}
		stroke="hsl(var(--muted-foreground) / 0.2)"
		stroke-width="0.5"
	/>

	<!-- Filled area under the timing line. -->
	<path
		d="{path} L{width},{baselineY} L0,{baselineY} Z"
		fill="hsl(var(--primary))"
		fill-opacity="0.08"
	/>

	<!-- Timing line. -->
	<path
		d={path}
		stroke="hsl(var(--primary))"
		stroke-width="1.5"
		stroke-linecap="round"
		stroke-linejoin="round"
	/>

	<!-- Y-axis minimum value label (bottom-left of plot area). -->
	<text x={PADDING} y={baselineY - 3} font-size="9" fill="hsl(var(--muted-foreground) / 0.6)"
		>{yMin}ms</text
	>

	<!-- Y-axis maximum value label (top-left of plot area). -->
	<text x={PADDING} y={PADDING + 9} font-size="9" fill="hsl(var(--muted-foreground) / 0.6)"
		>{yMax}ms</text
	>

	<!-- X-axis labels: sample count on the left, "now" on the right.
	     style:font-variant-numeric is used because Svelte's SVG attribute types
	     do not expose font-variant-numeric as a typed SVG attribute. -->
	<text
		x={PADDING}
		y={height - 4}
		font-size="10"
		fill="hsl(var(--muted-foreground))"
		text-anchor="start"
		style:font-variant-numeric="tabular-nums">−{sampleCount} samples</text
	>
	<text
		x={width - PADDING}
		y={height - 4}
		font-size="10"
		fill="hsl(var(--muted-foreground))"
		text-anchor="end"
		style:font-variant-numeric="tabular-nums">now</text
	>
</svg>
