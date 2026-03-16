<script lang="ts">
	import { sparklinePath } from "./sparkline-utils.js";

	let {
		values,
		width = 120,
		height = 80,
		strokeColor = "currentColor",
		strokeWidth = 1.5,
		fillOpacity = 0.08,
		showBaseline = true,
		padding = 4,
		fixedMin,
		fixedMax,
	}: {
		/** Numeric values to plot (left-to-right) */
		values: number[];
		width?: number;
		height?: number;
		/** Stroke colour (CSS value or hex) */
		strokeColor?: string;
		strokeWidth?: number;
		/** Opacity of the area fill under the line */
		fillOpacity?: number;
		/** Show a faint baseline at y=0 */
		showBaseline?: boolean;
		padding?: number;
		fixedMin?: number;
		fixedMax?: number;
	} = $props();

	const path = $derived(sparklinePath(values, width, height, { padding, fixedMin, fixedMax }));
	const baselineY = $derived(height - padding);
</script>

{#if path}
	<svg
		{width}
		{height}
		viewBox="0 0 {width} {height}"
		class="shrink-0"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		{#if showBaseline}
			<line
				x1="0"
				y1={baselineY}
				x2={width}
				y2={baselineY}
				stroke="currentColor"
				stroke-width="0.5"
				class="text-muted-foreground/20"
			/>
		{/if}
		<path
			d="{path} L{width},{baselineY} L0,{baselineY} Z"
			fill={strokeColor}
			fill-opacity={fillOpacity}
		/>
		<path
			d={path}
			stroke={strokeColor}
			stroke-width={strokeWidth}
			stroke-linecap="round"
			stroke-linejoin="round"
			fill="none"
		/>
	</svg>
{/if}
