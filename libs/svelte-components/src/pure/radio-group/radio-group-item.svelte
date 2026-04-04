<!-- RadioGroupItem wrapping bits-ui RadioGroup.Item. Renders a circular radio
     button with a filled indicator when selected and a focus ring on keyboard nav. -->
<script lang="ts">
	import { RadioGroup as RadioGroupPrimitive } from "bits-ui";
	import { cn } from "../../utils/cn.js";

	let {
		value,
		disabled = false,
	}: {
		value: string;
		disabled?: boolean;
	} = $props();
</script>

<RadioGroupPrimitive.Item
	{value}
	{disabled}
	data-slot="radio-group-item"
	class={cn(
		"aspect-square h-4 w-4 rounded-full border border-input shadow-xs",
		"focus-visible:outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:border-ring",
		"disabled:cursor-not-allowed disabled:opacity-50",
		"data-[state=checked]:border-primary",
		"transition-colors",
	)}
>
	{#snippet children({ checked })}
		<!-- Filled dot shown when this item is selected. -->
		<span
			class={cn(
				"flex items-center justify-center",
				"transition-opacity",
				checked ? "opacity-100" : "opacity-0"
			)}
			aria-hidden="true"
		>
			<span class="h-2 w-2 rounded-full bg-primary"></span>
		</span>
	{/snippet}
</RadioGroupPrimitive.Item>
