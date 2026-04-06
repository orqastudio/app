<!-- Root table element. Wraps a native <table> with consistent caption-bottom
     text sizing and full-width layout. -->
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import type { WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLTableElement>, "class">> = $props();
</script>

<table bind:this={ref} data-slot="table" class="w-full caption-bottom text-sm" {...restProps}>
	{@render children?.()}
</table>
