<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";

	let {
		ref = $bindable(null),
		...restProps
	}: Omit<AlertDialogPrimitive.DescriptionProps, "class"> = $props();
</script>

<AlertDialogPrimitive.Description
	bind:ref
	data-slot="alert-dialog-description"
	class="text-muted-foreground text-sm"
	{...restProps}
/>
