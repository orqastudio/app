<!-- Table body section (<tbody>). Removes the bottom border from the last row
     via [&_tr:last-child]:border-0 to avoid a double-border at the table edge. -->
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import type { WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLTableSectionElement>, "class">> = $props();
</script>

<tbody bind:this={ref} data-slot="table-body" class="[&_tr:last-child]:border-0" {...restProps}>
	{@render children?.()}
</tbody>
