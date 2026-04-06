<!-- tooltip-trigger.svelte — tooltip trigger wrapper.
     The `full` prop adds w-full so the trigger spans the parent container's full width.
     The `variant` prop applies a closed-set visual treatment:
       "metric-row" — compact hoverable row for metric/stat triggers in dense panels. -->
<script lang="ts">
	import { Tooltip as TooltipPrimitive } from "bits-ui";
	import { cn } from "../../utils/cn.js";

	// Variant classes — each variant encodes the complete visual treatment for the trigger element.
	const variantMap: Record<string, string> = {
		// Compact row with hover background — used in metric panels (e.g. GraphHealthPanel).
		"metric-row": "hover:bg-muted/60 w-full rounded px-1 py-0.5 text-left transition-colors",
	};

	let {
		ref = $bindable(null),
		full = false,
		variant,
		...restProps
	}: TooltipPrimitive.TriggerProps & { full?: boolean; variant?: "metric-row" } = $props();

	const variantClass = $derived(variant != null ? variantMap[variant] : undefined);
</script>

<TooltipPrimitive.Trigger
	bind:ref
	data-slot="tooltip-trigger"
	class={cn(full && "w-full", variantClass)}
	{...restProps}
/>
