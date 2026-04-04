<script lang="ts">
	import { Popover as PopoverPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		...restProps
	}: Omit<PopoverPrimitive.TriggerProps, "class"> = $props();
</script>

<PopoverPrimitive.Trigger
	bind:ref
	data-slot="popover-trigger"
	{...restProps}
/>
