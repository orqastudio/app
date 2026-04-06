<!-- FormSection: groups multiple FormGroups under a section heading with optional description. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { Stack } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	let {
		title,
		description,
		children,
	}: {
		title?: string;
		description?: string;
		children?: Snippet;
	} = $props();
</script>

<Stack gap={4}>
	{#if title}
		<Stack gap={0.5}>
			<Text variant="heading-lg">{title}</Text>
			{#if description}
				<Text variant="body-muted">{description}</Text>
			{/if}
		</Stack>
	{/if}
	{@render children?.()}
</Stack>
