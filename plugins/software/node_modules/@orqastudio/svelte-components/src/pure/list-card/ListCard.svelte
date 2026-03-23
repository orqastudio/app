<script lang="ts">
	import type { Component } from "svelte";
	import { cn } from "../../utils/cn.js";

	let {
		title,
		description,
		badge,
		statusIcon: StatusIcon,
		active = false,
		onclick,
	}: {
		title: string;
		description?: string;
		badge?: string;
		statusIcon?: Component;
		active?: boolean;
		onclick?: () => void;
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
		{#if StatusIcon}
			<StatusIcon class="inline-block h-3.5 w-3.5 shrink-0 text-muted-foreground" />
		{:else if badge}
			<span class="shrink-0 rounded bg-muted px-1 py-0.5 text-[10px] font-normal text-muted-foreground">{badge}</span>
		{/if}
		<span class="truncate">{title}</span>
	</span>
	{#if description}
		<p class="truncate text-xs text-muted-foreground">{description}</p>
	{/if}
</button>
