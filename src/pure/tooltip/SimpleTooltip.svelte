<script lang="ts">
	import { Tooltip as TooltipPrimitive } from "bits-ui";
	import TooltipContent from "./tooltip-content.svelte";
	import type { Snippet } from "svelte";

	let {
		side = "top",
		delayDuration,
		trigger,
		children,
	}: {
		side?: "top" | "bottom" | "left" | "right";
		delayDuration?: number;
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
		children: Snippet;
	} = $props();
</script>

<TooltipPrimitive.Root {delayDuration}>
	<TooltipPrimitive.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</TooltipPrimitive.Trigger>
	<TooltipContent {side}>
		{@render children()}
	</TooltipContent>
</TooltipPrimitive.Root>
