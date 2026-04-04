<script lang="ts">
	import { Tabs as TabsPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		...restProps
	}: Omit<TabsPrimitive.ListProps, "class"> = $props();
</script>

<TabsPrimitive.List
	bind:ref
	data-slot="tabs-list"
	class="bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px]"
	{...restProps}
/>
