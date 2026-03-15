<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import type { Snippet } from "svelte";

	let {
		title,
		description,
		action,
		children,
		class: className = "",
	}: {
		title?: string;
		description?: string;
		action?: Snippet;
		children?: Snippet;
		class?: string;
	} = $props();
</script>

<!--
	DashboardCard — consistent card wrapper for all dashboard widgets.
	Reduces the gap-6 default on Card.Root to gap-2 so header→content
	spacing is tighter across the dashboard.
-->
<Card.Root class="gap-2 {className}">
	{#if title || description || action}
		<Card.Header class="pb-2">
			{#if title}
				<Card.Title class="text-sm font-semibold">{title}</Card.Title>
			{/if}
			{#if description}
				<Card.Description class="text-xs">{description}</Card.Description>
			{/if}
			{#if action}
				<Card.Action>
					{@render action()}
				</Card.Action>
			{/if}
		</Card.Header>
	{/if}
	<Card.Content class="pt-0">
		{@render children?.()}
	</Card.Content>
</Card.Root>
