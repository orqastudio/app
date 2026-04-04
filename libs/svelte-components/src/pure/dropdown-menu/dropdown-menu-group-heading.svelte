<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
	import type { ComponentProps } from "svelte";

	let {
		ref = $bindable(null),
		inset,
		...restProps
	}: Omit<ComponentProps<typeof DropdownMenuPrimitive.GroupHeading>, "class"> & {
		inset?: boolean;
	} = $props();
</script>

<DropdownMenuPrimitive.GroupHeading
	bind:ref
	data-slot="dropdown-menu-group-heading"
	data-inset={inset}
	class="px-2 py-1.5 text-sm font-semibold data-[inset]:ps-8"
	{...restProps}
/>
