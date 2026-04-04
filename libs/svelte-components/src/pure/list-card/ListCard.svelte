<script lang="ts">
	import type { Component } from "svelte";
	import { cn } from "../../utils/cn.js";
	import { Stack, HStack } from "../layout/index.js";
	import { Text, Caption } from "../typography/index.js";

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
	<HStack gap={1.5}>
		{#if StatusIcon}
			<StatusIcon class="inline-block h-3.5 w-3.5 shrink-0 text-muted-foreground" />
		{:else if badge}
			<span class="shrink-0 rounded bg-muted px-1 py-0.5 text-[10px] font-normal text-muted-foreground">{badge}</span>
		{/if}
		<span class="truncate text-sm font-medium">{title}</span>
	</HStack>
	{#if description}
		<Caption truncate>{description}</Caption>
	{/if}
</button>
