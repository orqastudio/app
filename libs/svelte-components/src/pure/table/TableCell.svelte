<!-- Table data cell (<td>). Provides consistent padding matching TableHead.
     Set `mono` to render cell content in a monospace font. -->
<script lang="ts">
	import type { HTMLTdAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		mono = false,
		children,
		...restProps
	}: WithElementRef<Omit<HTMLTdAttributes, "class">> & {
		/** Renders content in a monospace font. */
		mono?: boolean;
	} = $props();
</script>

<td
	bind:this={ref}
	data-slot="table-cell"
	class={cn("px-3 py-2", mono && "font-mono")}
	{...restProps}
>
	{@render children?.()}
</td>
