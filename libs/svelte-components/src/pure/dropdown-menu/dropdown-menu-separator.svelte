<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		...restProps
	}: Omit<DropdownMenuPrimitive.SeparatorProps, "class"> = $props();
</script>

<DropdownMenuPrimitive.Separator
	bind:ref
	data-slot="dropdown-menu-separator"
	class="bg-border -mx-1 my-1 h-px"
	{...restProps}
/>
