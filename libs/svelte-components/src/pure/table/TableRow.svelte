<!-- Table row (<tr>). Supports interactive hover styling and selected state.
     Set `interactive` for clickable rows (adds hover + pointer cursor).
     Set `selected` to highlight the row with the accent background. -->
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		interactive = false,
		selected = false,
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLTableRowElement>, "class">> & {
		/** Adds hover highlight and pointer cursor for clickable rows. */
		interactive?: boolean;
		/** Highlights the row with the accent background colour. */
		selected?: boolean;
	} = $props();
</script>

<tr
	bind:this={ref}
	data-slot="table-row"
	class={cn(
		"border-b transition-colors",
		interactive && "cursor-pointer hover:bg-muted/30",
		selected && "bg-accent",
	)}
	{...restProps}
>
	{@render children?.()}
</tr>
