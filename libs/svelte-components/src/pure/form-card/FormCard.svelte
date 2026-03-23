<script lang="ts">
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "../card/index.js";
	import type { Snippet } from "svelte";

	let {
		title,
		description,
		children,
		class: className = "",
	}: {
		title: string;
		description?: string;
		children: Snippet;
		class?: string;
	} = $props();
</script>

<Card class={className}>
	<CardHeader>
		<CardTitle class="text-sm font-medium">{title}</CardTitle>
		{#if description}
			<CardDescription>{description}</CardDescription>
		{/if}
	</CardHeader>
	<CardContent>
		{@render children()}
	</CardContent>
</Card>
