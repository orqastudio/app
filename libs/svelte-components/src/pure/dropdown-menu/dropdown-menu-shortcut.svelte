<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import type { WithElementRef } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		children,
		...restProps
	}: WithElementRef<Omit<HTMLAttributes<HTMLSpanElement>, "class">> = $props();
</script>

<span
	bind:this={ref}
	data-slot="dropdown-menu-shortcut"
	class="text-muted-foreground ms-auto text-xs tracking-widest"
	{...restProps}
>
	{@render children?.()}
</span>
