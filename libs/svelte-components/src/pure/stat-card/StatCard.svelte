<!-- StatCard — a compact, clickable metric cell for dashboard stat grids.
     Renders as a muted rounded button with hover treatment, laying its children
     out in a centred flex column. Intended to be used inside a TooltipRoot as
     the trigger so the stat label and value show a detail tooltip on hover.
     Uses TooltipTrigger so screen readers and keyboard users can access the tooltip. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { TooltipTrigger } from "../tooltip/index.js";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();
</script>

<TooltipTrigger
	class="bg-muted/50 hover:bg-muted/80 flex flex-col items-center justify-center gap-1 rounded-md py-3 transition-colors"
>
	{@render children?.()}
</TooltipTrigger>
