<!-- RadioGroup root wrapping bits-ui RadioGroup.Root. Manages a group of mutually
     exclusive radio items with keyboard navigation. -->
<script lang="ts">
	import { RadioGroup as RadioGroupPrimitive } from "bits-ui";

	let {
		value = $bindable(""),
	}: {
		value?: string;
	} = $props();
</script>

<RadioGroupPrimitive.Root
	bind:value
	data-slot="radio-group"
	class="flex flex-col gap-2"
/>
