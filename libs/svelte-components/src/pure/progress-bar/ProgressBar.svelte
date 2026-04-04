<script lang="ts">
	import { Stack, HStack } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	let {
		label,
		current,
		total,
		colorClass = "bg-primary",
	}: {
		label: string;
		current: number;
		total: number;
		colorClass?: string;
	} = $props();

	const percentage = $derived(total > 0 ? Math.round((current / total) * 100) : 0);
</script>

<Stack gap={1}>
	<HStack justify="between">
		<Text variant="caption">{label}</Text>
		<Text variant="tabular">{current}/{total}</Text>
	</HStack>
	<div class="h-1.5 w-full rounded-full bg-muted">
		<div
			class="h-full rounded-full transition-all {colorClass}"
			style="width: {percentage}%"
		></div>
	</div>
</Stack>
