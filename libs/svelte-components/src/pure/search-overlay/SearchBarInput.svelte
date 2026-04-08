<!-- SearchBarInput — transparent, borderless text input for command palette headers.
     Designed to sit inside a SectionHeader alongside an Icon; it fills available
     horizontal space with flex-1 and provides no visible border or background so
     the parent SectionHeader controls the container styling.
     Bind `value` and `ref` from the consumer for controlled input and focus management. -->
<script lang="ts">
	let {
		value = $bindable(""),
		ref = $bindable<HTMLInputElement | null>(null),
		placeholder = "Search...",
	}: {
		value?: string;
		ref?: HTMLInputElement | null;
		placeholder?: string;
	} = $props();
</script>

<input
	bind:this={ref}
	bind:value
	{placeholder}
	class="text-foreground placeholder:text-muted-foreground h-12 flex-1 bg-transparent text-sm outline-none"
/>
