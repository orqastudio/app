<script lang="ts">
	import { Dialog as DialogPrimitive } from "bits-ui";
	import { cn } from "../../utils/cn.js";

	let { ref = $bindable(null), ...restProps }: Omit<DialogPrimitive.DescriptionProps, "class"> =
		$props();
</script>

<DialogPrimitive.Description
	bind:ref
	data-slot="dialog-description"
	class={cn("text-muted-foreground text-sm")}
	{...restProps}
/>
