<script lang="ts">
	import type { EventHandler } from "svelte/elements";

	let {
		ref = $bindable(null),
		value = $bindable(),
		placeholder,
		disabled = false,
		rows,
		id,
		name,
		oninput,
		onkeydown,
		onblur,
		"data-slot": dataSlot = "textarea",
	}: {
		ref?: HTMLTextAreaElement | null;
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		rows?: number;
		id?: string;
		name?: string;
		oninput?: EventHandler<Event, HTMLTextAreaElement>;
		onkeydown?: EventHandler<KeyboardEvent, HTMLTextAreaElement>;
		onblur?: EventHandler<FocusEvent, HTMLTextAreaElement>;
		"data-slot"?: string;
	} = $props();
</script>

<textarea
	bind:this={ref}
	data-slot={dataSlot}
	class="border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
	bind:value
	{placeholder}
	{disabled}
	{rows}
	{id}
	{name}
	{oninput}
	{onkeydown}
	{onblur}
></textarea>
