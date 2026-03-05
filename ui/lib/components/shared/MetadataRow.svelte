<script lang="ts">
	import type { Component } from "svelte";
	import SmallBadge from "./SmallBadge.svelte";
	import type { BadgeVariant } from "$lib/components/ui/badge";

	let {
		icon: Icon,
		label,
		items,
		badgeVariant = "secondary",
	}: {
		icon: Component;
		label: string;
		items: string[];
		badgeVariant?: BadgeVariant;
	} = $props();
</script>

{#if items.length > 0}
	<div class="flex flex-wrap items-center gap-1.5">
		<span class="flex items-center gap-1 text-xs text-muted-foreground">
			<Icon class="h-3.5 w-3.5" />
			{label}
		</span>
		{#each items as item, i (i)}
			<SmallBadge variant={badgeVariant}>{item}</SmallBadge>
		{/each}
	</div>
{/if}
