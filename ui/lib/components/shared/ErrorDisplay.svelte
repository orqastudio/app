<script lang="ts">
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";

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
		<button
			class="mt-3 rounded-md bg-secondary px-4 py-2 text-sm text-secondary-foreground hover:bg-secondary/80"
			onclick={onRetry}
		>
			Retry
		</button>
	{/if}
</div>
