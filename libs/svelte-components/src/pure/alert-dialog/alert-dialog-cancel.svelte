<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";
	import { buttonVariants } from "../button/index.js";

	let {
		ref = $bindable(null),
		...restProps
	}: Omit<AlertDialogPrimitive.CancelProps, "class"> = $props();
</script>

<AlertDialogPrimitive.Cancel
	bind:ref
	data-slot="alert-dialog-cancel"
	class={buttonVariants({ variant: "outline" })}
	{...restProps}
/>
