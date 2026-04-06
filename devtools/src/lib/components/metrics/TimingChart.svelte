<!-- TimingChart renders a labelled line chart for a single metric category's
     timing history. The SVG rendering is delegated to TimingChartSvg in the
     library so this file contains no raw HTML. Used by MetricsView to show the
     detailed distribution view when a metric cell is selected. -->
<script lang="ts">
	import {
		Stack,
		Text,
		Caption,
		SurfaceBox,
		TimingChartSvg,
	} from "@orqastudio/svelte-components/pure";
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
</script>

<Stack gap={1}>
	<!-- Label uses body-strong variant as the closest semantic fit for a chart title. -->
	<Text variant="body-strong">{stats.label}</Text>

	{#if stats.history.length < 2}
		<SurfaceBox center widthPx={width} heightPx={height}>
			<Caption>Waiting for data…</Caption>
		</SurfaceBox>
	{:else}
		<SurfaceBox widthPx={width}>
			<TimingChartSvg values={stats.history} {width} {height} sampleCount={stats.history.length} />
		</SurfaceBox>
	{/if}
</Stack>
