<script lang="ts">
	import { Dialog as DialogPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		...restProps
	}: Omit<DialogPrimitive.TitleProps, "class"> = $props();
</script>

<DialogPrimitive.Title
	bind:ref
	data-slot="dialog-title"
	class="text-lg font-semibold"
	{...restProps}
/>
