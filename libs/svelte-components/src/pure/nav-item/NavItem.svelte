<script lang="ts">
	import type { Component, Snippet } from "svelte";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import { cn } from "../../utils/cn.js";
	import { HStack, Stack } from "../layout/index.js";
	import { Text, Caption } from "../typography/index.js";

	let {
		icon: Icon,
		label,
		badge,
		active = false,
		expanded = $bindable(false),
		collapsible = false,
		onclick,
		children,
	}: {
		icon?: Component;
		label: string;
		badge?: string | number;
		active?: boolean;
		expanded?: boolean;
		collapsible?: boolean;
		onclick?: () => void;
		children?: Snippet;
	} = $props();

	function handleClick() {
		if (collapsible) {
			expanded = !expanded;
		}
		onclick?.();
	}
</script>

<div>
	<button
		class={cn(
			"flex w-full items-center gap-1.5 rounded px-2 py-1 text-sm hover:bg-accent/50",
			active && "bg-accent text-accent-foreground",
		)}
		onclick={handleClick}
	>
		{#if collapsible}
			<ChevronRightIcon
				class={cn("h-3.5 w-3.5 shrink-0 text-muted-foreground transition-transform", expanded && "rotate-90")}
			/>
		{/if}
		{#if Icon}
			<Icon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
		{/if}
		<span class="flex-1 truncate text-left">{label}</span>
		{#if badge !== undefined}
			<span class="shrink-0 rounded bg-muted px-1 py-0.5 text-[10px] text-muted-foreground tabular-nums">{badge}</span>
		{/if}
	</button>
	{#if collapsible && expanded && children}
		<div class="ml-4 mt-0.5">
			{@render children()}
		</div>
	{/if}
</div>
