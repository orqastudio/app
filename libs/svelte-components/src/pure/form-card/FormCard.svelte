<script lang="ts">
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "../card/index.js";
	import type { Snippet } from "svelte";

	let {
		title,
		description,
		children,
	}: {
		title: string;
		description?: string;
		children: Snippet;
	} = $props();
</script>

<Card>
	<CardHeader>
		<CardTitle size="sm">{title}</CardTitle>
		{#if description}
			<CardDescription>{description}</CardDescription>
		{/if}
	</CardHeader>
	<CardContent>
		{@render children()}
	</CardContent>
</Card>
