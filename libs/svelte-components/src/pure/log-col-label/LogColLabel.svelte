<!-- LogColLabel — column header label for the virtualised log table header row.
     Displays an overline-style uppercase muted label. Two variants:
     - fixed: a flex-shrink-0 div with an explicit pixel width matching the column.
     - fill: a min-w-0 flex-1 div that fills remaining space (used for the message column). -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	let {
		width,
		fill = false,
		children,
	}: {
		/** Pixel width for fixed-width columns. Omit when fill=true. */
		width?: number;
		/** When true, the column expands to fill remaining row width (message column). */
		fill?: boolean;
		children?: Snippet;
	} = $props();
</script>

<div
	data-slot="log-col-label"
	class={cn(
		"bg-surface-base text-muted-foreground text-[10px] font-medium tracking-[0.05em] uppercase",
		fill ? "min-w-0 flex-1" : "shrink-0",
	)}
	style={width != null ? `width: ${width}px;` : undefined}
>
	{@render children?.()}
</div>
