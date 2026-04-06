<script lang="ts">
	import type { Snippet } from "svelte";
	import {
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		CardAction,
	} from "../card/index.js";

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

<CardRoot gap={2}>
	{#if title || description || action}
		<CardHeader compact={true}>
			{#if title}
				<CardTitle size="sm">{title}</CardTitle>
			{/if}
			{#if description}
				<CardDescription size="xs">{description}</CardDescription>
			{/if}
			{#if action}
				<CardAction>
					{@render action()}
				</CardAction>
			{/if}
		</CardHeader>
	{/if}
	<CardContent compact={true}>
		{@render children?.()}
	</CardContent>
</CardRoot>
