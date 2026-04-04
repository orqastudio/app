<script lang="ts">
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import { Button } from "../button/index.js";
	import { Stack, Center } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	let {
		message,
		onRetry,
	}: {
		message: string;
		onRetry?: () => void;
	} = $props();
</script>

<Center>
	<Stack align="center" gap={2}>
		<CircleAlertIcon class="h-10 w-10 text-destructive" />
		<Text variant="caption" tone="destructive">{message}</Text>
		{#if onRetry}
			<Button variant="secondary" size="sm" onclick={onRetry}>
				Retry
			</Button>
		{/if}
	</Stack>
</Center>
