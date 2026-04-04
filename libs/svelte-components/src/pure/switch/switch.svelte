<!-- Switch primitive wrapping bits-ui Switch.Root. Renders an accessible pill-track
     toggle with a sliding thumb, transitions, and size variants. -->
<script lang="ts">
	import { Switch as SwitchPrimitive } from "bits-ui";
	import { cn } from "../../utils/cn.js";

	// Allowed size values for the switch track and thumb.
	type SwitchSize = "sm" | "default";

	let {
		checked = $bindable(false),
		disabled = false,
		size = "default" as SwitchSize,
		"aria-label": ariaLabel,
		onCheckedChange,
	}: {
		checked?: boolean;
		disabled?: boolean;
		size?: SwitchSize;
		"aria-label"?: string;
		onCheckedChange?: (checked: boolean) => void;
	} = $props();
</script>

<SwitchPrimitive.Root
	bind:checked
	{disabled}
	onCheckedChange={onCheckedChange}
	aria-label={ariaLabel}
	data-slot="switch"
	class={cn(
		"peer inline-flex shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent",
		"transition-colors",
		"focus-visible:outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50",
		"disabled:cursor-not-allowed disabled:opacity-50",
		"data-[state=checked]:bg-primary data-[state=unchecked]:bg-muted-foreground/30",
		size === "sm" ? "h-5 w-9" : "h-6 w-11",
	)}
>
	<SwitchPrimitive.Thumb
		class={cn(
			"pointer-events-none block rounded-full bg-background shadow-sm",
			"transition-transform duration-100",
			size === "sm"
				? "h-4 w-4 data-[state=checked]:translate-x-4 data-[state=unchecked]:translate-x-0"
				: "h-5 w-5 data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
		)}
	/>
</SwitchPrimitive.Root>
