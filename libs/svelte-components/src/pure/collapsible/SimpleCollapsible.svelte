<script lang="ts">
	import { Collapsible as CollapsiblePrimitive } from "bits-ui";
	import type { Snippet } from "svelte";

	let {
		open = $bindable(false),
		trigger,
		children,
	}: {
		open?: boolean;
		trigger: Snippet;
		children: Snippet;
	} = $props();
</script>

<CollapsiblePrimitive.Root bind:open>
	<CollapsiblePrimitive.Trigger>
		{@render trigger()}
	</CollapsiblePrimitive.Trigger>
	<CollapsiblePrimitive.Content>
		{@render children()}
	</CollapsiblePrimitive.Content>
</CollapsiblePrimitive.Root>
