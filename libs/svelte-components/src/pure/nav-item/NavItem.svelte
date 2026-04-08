<script lang="ts">
	import type { Component, Snippet } from "svelte";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import { cn } from "../../utils/cn.js";
	import { Stack } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	let {
		icon: Icon,
		label,
		description,
		badge,
		active = false,
		expanded = $bindable(false),
		collapsible = false,
		onclick,
		children,
	}: {
		icon?: Component;
		label: string;
		/** Optional subtitle shown below the label in a smaller, muted style. */
		description?: string;
		badge?: string | number;
		active?: boolean;
		expanded?: boolean;
		collapsible?: boolean;
		onclick?: () => void;
		children?: Snippet;
	} = $props();

	/** Toggle the expanded state when the item is collapsible, then call the optional onclick callback. */
	function handleClick() {
		if (collapsible) {
			expanded = !expanded;
		}
		onclick?.();
	}
</script>

<Stack gap={0}>
	<button
		class={cn(
			"hover:bg-accent/50 flex w-full items-center gap-2 rounded px-2 py-1 text-sm",
			active && "bg-accent text-accent-foreground",
		)}
		onclick={handleClick}
	>
		{#if collapsible}
			<ChevronRightIcon
				class={cn(
					"text-muted-foreground h-3.5 w-3.5 shrink-0 transition-transform",
					expanded && "rotate-90",
				)}
			/>
		{/if}
		{#if Icon}
			<Icon class="text-muted-foreground h-3.5 w-3.5 shrink-0" />
		{/if}
		{#if description}
			<Stack gap={0} align="start" minHeight={0} flex={1}>
				<Text variant="label" truncate>{label}</Text>
				<Text variant="caption" truncate>{description}</Text>
			</Stack>
		{:else}
			<span class="flex-1 truncate text-left">{label}</span>
		{/if}
		{#if badge !== undefined}
			<span
				class="bg-muted text-muted-foreground shrink-0 rounded px-1 py-1 text-[10px] tabular-nums"
				>{badge}</span
			>
		{/if}
	</button>
	{#if collapsible && expanded && children}
		<div class="mt-1 ml-4">
			{@render children()}
		</div>
	{/if}
</Stack>
