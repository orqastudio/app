<script lang="ts">
	import { Tabs as TabsPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		value = $bindable(""),
		...restProps
	}: Omit<TabsPrimitive.RootProps, "class"> = $props();
</script>

<TabsPrimitive.Root
	bind:ref
	bind:value
	data-slot="tabs"
	class="flex flex-col gap-2"
	{...restProps}
/>
