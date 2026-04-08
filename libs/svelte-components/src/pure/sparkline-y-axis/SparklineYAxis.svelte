<!-- SparklineYAxis — a fixed-height column showing top and bottom y-axis scale labels.
     Used alongside custom SVG sparklines to display the axis scale. The height must
     match the SVG viewBox height so labels align with the chart top and bottom edges.
     Font size (9px) and tabular-nums are chart-specific and not design-token values. -->
<script lang="ts">
	let {
		maxLabel,
		minLabel = "0",
		height,
	}: {
		/** Label displayed at the top (maximum value). */
		maxLabel: string | number;
		/** Label displayed at the bottom (minimum value). Defaults to "0". */
		minLabel?: string | number;
		/** Height in pixels — must match the adjacent SVG viewBox height. */
		height: number;
	} = $props();
</script>

<div
	class="flex flex-col justify-between text-right"
	style:height="{height}px"
	style:font-size="9px"
	style:font-variant-numeric="tabular-nums"
	style:color="hsl(var(--muted-foreground) / 0.6)"
>
	<span>{maxLabel}</span>
	<span>{minLabel}</span>
</div>
