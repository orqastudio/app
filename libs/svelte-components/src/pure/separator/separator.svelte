<script lang="ts">
	import { Separator as SeparatorPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		orientation = "horizontal",
		"data-slot": dataSlot = "separator",
	}: {
		ref?: HTMLElement | null;
		orientation?: "horizontal" | "vertical";
		"data-slot"?: string;
	} = $props();
</script>

<SeparatorPrimitive.Root
	bind:ref
	data-slot={dataSlot}
	{orientation}
	class="bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:min-h-full data-[orientation=vertical]:w-px"
/>
