<script lang="ts">
	import type { HTMLInputTypeAttribute, HTMLInputAttributes } from "svelte/elements";

	type InputType = Exclude<HTMLInputTypeAttribute, "file">;

	// Extra semantic props (aria-*, data-*, id, autofocus, title, name, ...) are
	// forwarded via ...restProps. `class` and `style` are blocked — typed props
	// are the only styling interface.
	let {
		ref = $bindable(null),
		value = $bindable(),
		type,
		size = "default",
		files = $bindable(),
		"data-slot": dataSlot = "input",
		...restProps
	}: Omit<HTMLInputAttributes, "class" | "style" | "size"> & {
		ref?: HTMLInputElement | null;
		type?: InputType | "file";
		/** Size variant. compact reduces height and font for use in dense inline contexts. */
		size?: "default" | "compact";
		files?: FileList;
	} = $props();
</script>

{#if type === "file"}
	<input
		bind:this={ref}
		data-slot={dataSlot}
		class="selection:bg-primary dark:bg-input/30 selection:text-primary-foreground border-input ring-offset-background placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 pt-1.5 text-sm font-medium shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50"
		type="file"
		bind:files
		bind:value
		{...restProps}
	/>
{:else}
	<input
		bind:this={ref}
		data-slot={dataSlot}
		class={size === "compact"
			? "border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive flex h-[22px] w-full min-w-0 rounded-md border px-1.5 py-0 text-[11px] shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50"
			: "border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive flex h-9 w-full min-w-0 rounded-md border px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"}
		{type}
		bind:value
		{...restProps}
	/>
{/if}
