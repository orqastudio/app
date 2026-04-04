<script lang="ts">
	import type { Component, Snippet } from "svelte";
	import { formatTrend, trendArrow, trendColorClass } from "../sparkline/sparkline-utils.js";
	import { Stack, HStack } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	const VALUE_COLOR_MAP = {
		default: "text-foreground",
		muted: "text-muted-foreground",
		primary: "text-primary",
		success: "text-success",
		warning: "text-warning",
		destructive: "text-destructive",
	} as const;

	let {
		label,
		value,
		trend,
		lowerIsBetter = true,
		icon: Icon,
		valueColor = "default",
		children,
	}: {
		/** Metric label text */
		label: string;
		/** Primary metric value (displayed large) */
		value: string | number;
		/** Trend percentage — shows arrow + percentage change */
		trend?: number | null;
		/** If true, negative trend is good (green). Default true. */
		lowerIsBetter?: boolean;
		/** Optional icon displayed with the label */
		icon?: Component;
		/** Semantic colour for the value */
		valueColor?: keyof typeof VALUE_COLOR_MAP;
		/** Optional content below the value (e.g. a Sparkline) */
		children?: Snippet;
	} = $props();

	const trendClass = $derived(trendColorClass(trend ?? null, lowerIsBetter));
</script>

<Stack gap={1}>
	<HStack justify="between" align="baseline">
		<HStack gap={1}>
			{#if Icon}
				<Icon class="h-3.5 w-3.5" />
			{/if}
			<Text variant="caption">{label}</Text>
		</HStack>
		<HStack gap={1.5} align="center">
			<span class="text-lg font-semibold tabular-nums {VALUE_COLOR_MAP[valueColor]}">
				{value}
			</span>
			{#if trend !== undefined && trend !== null}
				<span class="text-xs font-medium {trendClass}">
					{trendArrow(trend)} {formatTrend(trend)}
				</span>
			{/if}
		</HStack>
	</HStack>
	{#if children}
		{@render children()}
	{/if}
</Stack>
