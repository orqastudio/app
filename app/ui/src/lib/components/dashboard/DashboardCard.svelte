<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, CardAction } from "@orqastudio/svelte-components/pure";
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
<CardRoot class="gap-2 {className}">
	{#if title || description || action}
		<CardHeader class="pb-2">
			{#if title}
				<CardTitle class="text-sm font-semibold">{title}</CardTitle>
			{/if}
			{#if description}
				<CardDescription class="text-xs">{description}</CardDescription>
			{/if}
			{#if action}
				<CardAction>
					{@render action()}
				</CardAction>
			{/if}
		</CardHeader>
	{/if}
	<CardContent class="pt-0">
		{@render children?.()}
	</CardContent>
</CardRoot>
