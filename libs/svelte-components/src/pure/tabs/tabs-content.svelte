<script lang="ts">
	import { Tabs as TabsPrimitive } from "bits-ui";

	let { ref = $bindable(null), ...restProps }: Omit<TabsPrimitive.ContentProps, "class"> = $props();
</script>

<TabsPrimitive.Content
	bind:ref
	data-slot="tabs-content"
	class="flex-1 outline-none"
	{...restProps}
/>
