<script lang="ts">
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import { Button } from "$lib/components/ui/button";

	let {
		message,
		onRetry,
	}: {
		message: string;
		onRetry?: () => void;
	} = $props();
</script>

<div class="flex flex-col items-center justify-center py-8 text-center">
	<CircleAlertIcon class="mb-3 h-10 w-10 text-destructive" />
	<p class="text-sm text-destructive">{message}</p>
	{#if onRetry}
		<Button variant="secondary" size="sm" class="mt-3" onclick={onRetry}>
			Retry
		</Button>
	{/if}
</div>
