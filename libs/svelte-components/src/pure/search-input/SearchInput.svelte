<script lang="ts">
	import SearchIcon from "@lucide/svelte/icons/search";

	let {
		value = $bindable(""),
		placeholder = "Search...",
		size = "sm",
	}: {
		value?: string;
		placeholder?: string;
		size?: "sm" | "xs";
	} = $props();

	const inputSizeClass = $derived(size === "xs" ? "h-7 pl-7" : "h-8 pl-8");
	const iconClass = $derived(size === "xs" ? "left-2" : "left-2.5");
</script>

<div class="relative">
	<SearchIcon
		class="absolute {iconClass} text-muted-foreground top-1/2 h-3.5 w-3.5 -translate-y-1/2"
	/>
	<input
		class="border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 flex w-full min-w-0 rounded-md border px-3 py-1 text-xs shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 {inputSizeClass}"
		{placeholder}
		bind:value
	/>
</div>
