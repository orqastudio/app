<!-- Checkbox primitive wrapping bits-ui Checkbox.Root. Renders a 16x16 accessible
     checkbox with a visible check mark, focus ring, and disabled state. -->
<script lang="ts">
	import { Checkbox as CheckboxPrimitive } from "bits-ui";
	import { cn } from "../../utils/cn.js";

	let {
		checked = $bindable(false),
		disabled = false,
	}: {
		checked?: boolean;
		disabled?: boolean;
	} = $props();
</script>

<CheckboxPrimitive.Root
	bind:checked
	{disabled}
	data-slot="checkbox"
	class={cn(
		"peer h-4 w-4 shrink-0 rounded border border-input shadow-xs",
		"focus-visible:outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:border-ring",
		"disabled:cursor-not-allowed disabled:opacity-50",
		"data-[state=checked]:bg-primary data-[state=checked]:border-primary data-[state=checked]:text-primary-foreground",
		"transition-colors",
	)}
>
	{#snippet children({ checked: isChecked })}
		<!-- Check icon shown only when checked. -->
		<span
			class={cn(
				"flex items-center justify-center text-current",
				"transition-opacity",
				isChecked ? "opacity-100" : "opacity-0"
			)}
			aria-hidden="true"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="3"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="size-3"
			>
				<polyline points="20 6 9 17 4 12" />
			</svg>
		</span>
	{/snippet}
</CheckboxPrimitive.Root>
