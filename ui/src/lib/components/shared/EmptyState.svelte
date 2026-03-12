<script lang="ts">
	import type { Component } from "svelte";
	import { Button } from "$lib/components/ui/button";

	let {
		icon: Icon,
		title,
		description,
		action,
	}: {
		icon?: Component;
		title: string;
		description?: string;
		action?: { label: string; onclick: () => void };
	} = $props();
</script>

<div class="flex flex-col items-center justify-center py-12 text-center">
	{#if Icon}
		<Icon class="mb-4 h-12 w-12 text-muted-foreground" />
	{/if}
	<h3 class="text-lg font-semibold">{title}</h3>
	{#if description}
		<p class="mt-1 text-sm text-muted-foreground">{description}</p>
	{/if}
	{#if action}
		<Button variant="default" size="sm" class="mt-4" onclick={action.onclick}>
			{action.label}
		</Button>
	{/if}
</div>
