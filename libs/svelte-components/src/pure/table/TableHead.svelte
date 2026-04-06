<!-- Table header cell (<th>). Supports sort state display, text alignment, column width, and monospace rendering.
     Set `sortable` to indicate the column can be sorted (changes cursor + hover).
     Set `sorted` to "asc" or "desc" to show a directional sort arrow icon.
     Set `align` to control text alignment (defaults to left).
     Set `width` to constrain column width to a design-system size token.
     Set `mono` to render cell content in a monospace font. -->
<script lang="ts">
	import type { HTMLThAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "../../utils/cn.js";
	import Icon from "../icon/Icon.svelte";

	let {
		ref = $bindable(null),
		sortable = false,
		sorted = false,
		align = "left",
		width,
		mono = false,
		children,
		...restProps
	}: WithElementRef<Omit<HTMLThAttributes, "class">> & {
		/** Whether the column is sortable — adds pointer cursor and hover colour. */
		sortable?: boolean;
		/** Current sort direction, or false when unsorted. */
		sorted?: "asc" | "desc" | false;
		/** Horizontal alignment of cell content. */
		align?: "left" | "center" | "right";
		/** Constrains column width to a design-system token. */
		width?: "xs" | "sm" | "md" | "auto";
		/** Renders content in a monospace font. */
		mono?: boolean;
	} = $props();

	const widthClass = $derived(
		width === "xs" ? "w-16" : width === "sm" ? "w-24" : width === "md" ? "w-40" : undefined,
	);
</script>

<th
	bind:this={ref}
	data-slot="table-head"
	class={cn(
		"text-muted-foreground px-3 py-2 text-left text-xs font-medium",
		sortable && "hover:text-foreground cursor-pointer select-none",
		align === "center" && "text-center",
		align === "right" && "text-right",
		mono && "font-mono",
		widthClass,
	)}
	{...restProps}
>
	{@render children?.()}
	{#if sorted}
		<Icon name={sorted === "asc" ? "arrow-up" : "arrow-down"} size="xs" />
	{/if}
</th>
