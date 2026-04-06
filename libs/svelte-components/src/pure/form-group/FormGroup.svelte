<!-- FormGroup: wraps label + input slot + helper/error text in a consistent vertical layout. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { Stack } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	let {
		label,
		for: htmlFor,
		required = false,
		error,
		description,
		children,
	}: {
		label?: string;
		for?: string;
		required?: boolean;
		error?: string;
		description?: string;
		children?: Snippet;
	} = $props();
</script>

<Stack gap={1.5}>
	{#if label}
		<label class="text-sm leading-none font-medium" for={htmlFor}>
			{label}
			{#if required}<span class="text-destructive ml-0.5">*</span>{/if}
		</label>
	{/if}
	{@render children?.()}
	{#if description && !error}
		<Text variant="caption">{description}</Text>
	{/if}
	{#if error}
		<Text variant="caption" tone="destructive">{error}</Text>
	{/if}
</Stack>
