<!-- ProgressBar — labelled progress indicator with fraction text.
     For the full-featured bar with label and fraction, use the default (mini=false).
     For a compact inline bar without label or fraction (e.g. in list items or cards),
     pass mini={true} with a ratio prop instead of label/current/total. -->
<script lang="ts">
	import { Stack, HStack } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	let {
		mini = false,
		ratio,
		label,
		current,
		total,
		colorClass = "bg-primary",
	}: {
		/**
		 * When true, renders a compact inline bar only (no label or fraction text).
		 * Requires the ratio prop (0.0–1.0). label/current/total are ignored.
		 */
		mini?: boolean;
		/** Progress value from 0.0 to 1.0. Used only when mini=true. */
		ratio?: number;
		/** Label text shown above the bar. Required when mini=false. */
		label?: string;
		/** Current progress count. Required when mini=false. */
		current?: number;
		/** Total progress count. Required when mini=false. */
		total?: number;
		colorClass?: string;
	} = $props();

	// Full-featured mode: percentage from current/total.
	const percentage = $derived(
		mini
			? Math.round(Math.max(0, Math.min(1, ratio ?? 0)) * 100)
			: total != null && total > 0
				? Math.round(((current ?? 0) / total) * 100)
				: 0,
	);
</script>

{#if mini}
	<!-- Compact inline bar — no label or fraction text. -->
	<div class="bg-muted h-1 w-full flex-1 overflow-hidden rounded-full">
		<div class="h-full rounded-full transition-all {colorClass}" style:width="{percentage}%"></div>
	</div>
{:else}
	<Stack gap={1}>
		<HStack justify="between">
			<Text variant="caption">{label}</Text>
			<Text variant="tabular">{current}/{total}</Text>
		</HStack>
		<div class="bg-muted h-1.5 w-full rounded-full">
			<div
				class="h-full rounded-full transition-all {colorClass}"
				style="width: {percentage}%"
			></div>
		</div>
	</Stack>
{/if}
