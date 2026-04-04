<script lang="ts">
	import * as ResizablePrimitive from "paneforge";

	let {
		ref = $bindable(null),
		this: paneGroup = $bindable(),
		...restProps
	}: Omit<ResizablePrimitive.PaneGroupProps, "class"> & {
		this?: ResizablePrimitive.PaneGroup;
	} = $props();
</script>

<ResizablePrimitive.PaneGroup
	bind:this={paneGroup}
	data-slot="resizable-pane-group"
	class="flex h-full w-full data-[direction=vertical]:flex-col"
	{...restProps}
/>
