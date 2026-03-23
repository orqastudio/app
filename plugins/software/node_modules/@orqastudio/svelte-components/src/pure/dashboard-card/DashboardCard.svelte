<script lang="ts">
	import type { Snippet } from "svelte";
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardAction } from "../card/index.js";

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
