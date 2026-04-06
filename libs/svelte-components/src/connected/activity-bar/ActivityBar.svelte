<script lang="ts">
	import type { Component, Snippet } from "svelte";
	import { Tooltip } from "../../pure/tooltip/index.js";
	import { cn } from "../../utils/cn.js";
	import { Separator } from "../../pure/separator/index.js";

	export type ActivityBarItem = {
		icon: Component;
		label: string;
		key: string;
		active?: boolean;
		onclick: () => void;
	};

	let {
		topItems = [],
		bottomItems = [],
		extraTop,
		extraBottom,
	}: {
		topItems?: ActivityBarItem[];
		bottomItems?: ActivityBarItem[];
		extraTop?: Snippet;
		extraBottom?: Snippet;
	} = $props();
</script>

<div class="border-border bg-muted/30 flex w-12 flex-col items-center border-r py-2">
	{#each topItems as item (item.key)}
		<Tooltip side="right">
			{#snippet trigger({ props })}
				<button
					{...props}
					class={cn(
						"text-muted-foreground hover:bg-accent hover:text-accent-foreground flex h-9 w-9 items-center justify-center rounded-md transition-colors",
						item.active && "bg-accent text-accent-foreground",
					)}
					onclick={item.onclick}
				>
					<item.icon class="h-4 w-4" />
				</button>
			{/snippet}
			<p>{item.label}</p>
		</Tooltip>
	{/each}

	{#if extraTop}
		<div class="my-1 w-6"><Separator /></div>
		{@render extraTop()}
	{/if}

	<div class="flex-1"></div>

	{#if extraBottom}
		{@render extraBottom()}
		<div class="my-1 w-6"><Separator /></div>
	{/if}

	{#each bottomItems as item (item.key)}
		<Tooltip side="right">
			{#snippet trigger({ props })}
				<button
					{...props}
					class={cn(
						"text-muted-foreground hover:bg-accent hover:text-accent-foreground flex h-9 w-9 items-center justify-center rounded-md transition-colors",
						item.active && "bg-accent text-accent-foreground",
					)}
					onclick={item.onclick}
				>
					<item.icon class="h-4 w-4" />
				</button>
			{/snippet}
			<p>{item.label}</p>
		</Tooltip>
	{/each}
</div>
