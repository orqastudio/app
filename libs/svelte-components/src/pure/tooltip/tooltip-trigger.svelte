<!-- tooltip-trigger.svelte — tooltip trigger wrapper.
     Forwards all props to the bits-ui Tooltip.Trigger. The `full` prop adds
     w-full so the trigger spans the parent container's full width. -->
<script lang="ts">
	import { Tooltip as TooltipPrimitive } from "bits-ui";
	import { cn } from "../../utils/cn.js";

	let {
		ref = $bindable(null),
		full = false,
		class: extraClass,
		...restProps
	}: TooltipPrimitive.TriggerProps & { full?: boolean; class?: string } = $props();
</script>

<TooltipPrimitive.Trigger
	bind:ref
	data-slot="tooltip-trigger"
	class={cn(full && "w-full", extraClass)}
	{...restProps}
/>
