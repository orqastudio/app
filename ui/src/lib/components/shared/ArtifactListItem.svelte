<script lang="ts">
	import StatusIndicator from "./StatusIndicator.svelte";
	import { cn } from "$lib/utils";

	let {
		label,
		description,
		status,
		active = false,
		onclick,
	}: {
		label: string;
		description?: string;
		status?: string;
		active?: boolean;
		onclick: () => void;
	} = $props();
</script>

<button
	class={cn(
		"flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left hover:bg-accent/50",
		active && "bg-accent",
	)}
	{onclick}
>
	<span class="flex items-center gap-1.5 truncate text-sm font-medium">
		{#if status}
			<StatusIndicator {status} mode="dot" />
		{/if}
		<span class="truncate">{label}</span>
	</span>
	{#if description}
		<p class="truncate text-xs text-muted-foreground">{description}</p>
	{/if}
</button>
