<script lang="ts">
	import type { HTMLInputTypeAttribute, FullAutoFill } from "svelte/elements";

	type InputType = Exclude<HTMLInputTypeAttribute, "file">;

	let {
		ref = $bindable(null),
		value = $bindable(),
		type,
		files = $bindable(),
		placeholder,
		disabled = false,
		id,
		name,
		autocomplete,
		oninput,
		onkeydown,
		onchange,
		onblur,
		"data-slot": dataSlot = "input",
	}: {
		ref?: HTMLInputElement | null;
		value?: string | number | readonly string[];
		type?: InputType | "file";
		files?: FileList;
		placeholder?: string;
		disabled?: boolean;
		id?: string;
		name?: string;
		autocomplete?: FullAutoFill;
		oninput?: (e: Event & { currentTarget: HTMLInputElement }) => void;
		onkeydown?: (e: KeyboardEvent & { currentTarget: HTMLInputElement }) => void;
		onchange?: (e: Event & { currentTarget: HTMLInputElement }) => void;
		onblur?: (e: FocusEvent & { currentTarget: HTMLInputElement }) => void;
		"data-slot"?: string;
	} = $props();
</script>

{#if type === "file"}
	<input
		bind:this={ref}
		data-slot={dataSlot}
		class="selection:bg-primary dark:bg-input/30 selection:text-primary-foreground border-input ring-offset-background placeholder:text-muted-foreground flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 pt-1.5 text-sm font-medium shadow-xs transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive"
		type="file"
		bind:files
		bind:value
		{placeholder}
		{disabled}
		{id}
		{name}
		{autocomplete}
		{oninput}
		{onkeydown}
		{onchange}
		{onblur}
	/>
{:else}
	<input
		bind:this={ref}
		data-slot={dataSlot}
		class="border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground flex h-9 w-full min-w-0 rounded-md border px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive"
		{type}
		bind:value
		{placeholder}
		{disabled}
		{id}
		{name}
		{autocomplete}
		{oninput}
		{onkeydown}
		{onchange}
		{onblur}
	/>
{/if}
