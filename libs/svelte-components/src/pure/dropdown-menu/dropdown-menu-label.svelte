<script lang="ts">
	import type { WithElementRef } from "../../utils/cn.js";
	import type { HTMLAttributes } from "svelte/elements";

	let {
		ref = $bindable(null),
		inset,
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLDivElement>, "class">> & {
		inset?: boolean;
	} = $props();
</script>

<div
	bind:this={ref}
	data-slot="dropdown-menu-label"
	data-inset={inset}
	class="px-2 py-2 text-sm font-semibold data-[inset]:ps-8"
	{...restProps}
>
	{@render children?.()}
</div>
