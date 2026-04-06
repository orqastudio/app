<!-- Table caption element. Renders below the table (caption-side: bottom).
     Used for summary text, row counts, or accessibility descriptions. -->
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import type { WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLElement>, "class">> = $props();
</script>

<caption
	bind:this={ref}
	data-slot="table-caption"
	class="text-muted-foreground mt-2 text-sm"
	{...restProps}
>
	{@render children?.()}
</caption>
