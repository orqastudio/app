<script lang="ts">
	import { Popover as PopoverPrimitive } from "bits-ui";
	import PopoverContent from "./popover-content.svelte";
	import type { Snippet } from "svelte";

	let {
		open = $bindable(false),
		align = "center",
		side = "bottom",
		trigger,
		children,
	}: {
		open?: boolean;
		align?: "start" | "center" | "end";
		side?: "top" | "bottom" | "left" | "right";
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
		children: Snippet;
	} = $props();
</script>

<PopoverPrimitive.Root bind:open>
	<PopoverPrimitive.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</PopoverPrimitive.Trigger>
	<PopoverContent {align} {side}>
		{@render children()}
	</PopoverContent>
</PopoverPrimitive.Root>
