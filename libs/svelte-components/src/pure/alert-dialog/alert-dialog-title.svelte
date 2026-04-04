<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		...restProps
	}: Omit<AlertDialogPrimitive.TitleProps, "class"> = $props();
</script>

<AlertDialogPrimitive.Title
	bind:ref
	data-slot="alert-dialog-title"
	class="text-lg font-semibold"
	{...restProps}
/>
