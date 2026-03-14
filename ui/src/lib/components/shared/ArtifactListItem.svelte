<script lang="ts">
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import StatusIndicator from "./StatusIndicator.svelte";
	import { cn } from "$lib/utils";

	let {
		label,
		description,
		status,
		actionsNeeded = false,
		active = false,
		onclick,
	}: {
		label: string;
		description?: string;
		status?: string;
		actionsNeeded?: boolean;
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
		{#if actionsNeeded}
			<CircleAlertIcon class="h-3.5 w-3.5 shrink-0 text-amber-500" />
		{/if}
		<span class="truncate">{label}</span>
	</span>
	{#if description}
		<p class="truncate text-xs text-muted-foreground">{description}</p>
	{/if}
</button>
