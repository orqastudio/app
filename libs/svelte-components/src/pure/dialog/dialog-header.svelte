<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import type { WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLDivElement>, "class">> = $props();
</script>

<div
	bind:this={ref}
	data-slot="dialog-header"
	class="flex flex-col gap-2 text-center sm:text-start"
	{...restProps}
>
	{@render children?.()}
</div>
