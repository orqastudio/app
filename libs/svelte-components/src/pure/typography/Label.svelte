<!-- Form label element. Renders as <label> with Text variant="label" styling. Shows a destructive asterisk when required. -->
<script lang="ts">
	import Text from "./Text.svelte";
	import type { Snippet } from "svelte";
	import type { HTMLLabelAttributes } from "svelte/elements";

	export interface LabelProps extends Omit<HTMLLabelAttributes, "class" | "style" | "for"> {
		htmlFor?: string;
		required?: boolean;
		children?: Snippet;
	}

	let { htmlFor, required = false, children, ...restProps }: LabelProps = $props();
</script>

<label
	for={htmlFor}
	class="leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
	{...restProps}
>
	<Text variant="label">
		{@render children?.()}
		{#if required}<span class="text-destructive ml-0.5">*</span>{/if}
	</Text>
</label>
