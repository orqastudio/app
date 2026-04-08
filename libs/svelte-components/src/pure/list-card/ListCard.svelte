<script lang="ts">
	import type { Component } from "svelte";
	import { cn } from "../../utils/cn.js";
	import { HStack } from "../layout/index.js";
	import { Caption } from "../typography/index.js";

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
		"hover:bg-accent/50 flex w-full flex-col gap-1 rounded px-2 py-2 text-left",
		active && "bg-accent",
	)}
	{onclick}
>
	<HStack gap={2}>
		{#if StatusIcon}
			<StatusIcon class="text-muted-foreground inline-block h-3.5 w-3.5 shrink-0" />
		{:else if badge}
			<span
				class="bg-muted text-muted-foreground shrink-0 rounded px-1 py-1 text-[10px] font-normal"
				>{badge}</span
			>
		{/if}
		<span class="truncate text-sm font-medium">{title}</span>
	</HStack>
	{#if description}
		<Caption truncate>{description}</Caption>
	{/if}
</button>
