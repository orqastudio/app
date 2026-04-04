<!-- Table header section (<thead>). Applies a muted background and bottom border.
     Child <tr> elements inherit the border-b via the [&_tr]:border-b selector. -->
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import type { WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLTableSectionElement>, "class">> = $props();
</script>

<thead
	bind:this={ref}
	data-slot="table-header"
	class="border-b bg-muted/50 [&_tr]:border-b"
	{...restProps}
>
	{@render children?.()}
</thead>
