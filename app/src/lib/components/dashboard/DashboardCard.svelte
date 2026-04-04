<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardAction } from "@orqastudio/svelte-components/pure";
	import type { Snippet } from "svelte";

	let {
		title,
		description,
		action,
		children,
	}: {
		title?: string;
		description?: string;
		action?: Snippet;
		children?: Snippet;
	} = $props();
</script>

<!--
	DashboardCard — consistent card wrapper for all dashboard widgets.
	Uses gap={2} on CardRoot and compact CardHeader/CardContent for tighter spacing.
-->
<CardRoot gap={2}>
	{#if title || description || action}
		<CardHeader compact>
			{#if title}
				<CardTitle>{title}</CardTitle>
			{/if}
			{#if description}
				<CardDescription>{description}</CardDescription>
			{/if}
			{#if action}
				<CardAction>
					{@render action()}
				</CardAction>
			{/if}
		</CardHeader>
	{/if}
	<CardContent>
		{@render children?.()}
	</CardContent>
</CardRoot>
