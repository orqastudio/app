<script lang="ts">
	import type { WithElementRef } from "../../utils/cn.js";
	import type { HTMLAttributes } from "svelte/elements";

	let {
		ref = $bindable(null),
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLDivElement>, "class">> = $props();
</script>

<div
	bind:this={ref}
	data-slot="alert-dialog-footer"
	class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"
	{...restProps}
>
	{@render children?.()}
</div>
