<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";
	import { buttonVariants } from "../button/index.js";

	let { ref = $bindable(null), ...restProps }: Omit<AlertDialogPrimitive.ActionProps, "class"> =
		$props();
</script>

<AlertDialogPrimitive.Action
	bind:ref
	data-slot="alert-dialog-action"
	class={buttonVariants()}
	{...restProps}
/>
