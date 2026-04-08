<!-- MetricGridCell — a flex-column grid cell with optional per-edge border decorations.
     Used in 2×2 or N×M metric grids where cells need independent top and right borders
     to create internal dividers without a visible outer border on the grid container.
     The cell shrinks below content size (min-h-0) so it participates correctly in
     fixed-height grid layouts. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	let {
		borderRight = false,
		borderTop = false,
		children,
	}: {
		/** Apply a right divider border (used on left-column cells in 2-column grids). */
		borderRight?: boolean;
		/** Apply a top divider border (used on cells in the top row). */
		borderTop?: boolean;
		children?: Snippet;
	} = $props();
</script>

<div
	class={cn(
		"flex min-h-0 flex-col overflow-hidden",
		borderRight && "border-border border-r",
		borderTop && "border-border border-t",
	)}
>
	{@render children?.()}
</div>
