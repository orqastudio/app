<script lang="ts">
	import { Input } from "$lib/components/ui/input";
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

	const sizeClasses: Record<"sm" | "xs", { input: string; icon: string }> = {
		sm: { input: "h-8 pl-8", icon: "left-2.5" },
		xs: { input: "h-7 pl-7", icon: "left-2" },
	};

	const classes = $derived(sizeClasses[size]);
</script>

<div class="relative">
	<SearchIcon
		class="absolute {classes.icon} top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground"
	/>
	<Input class="{classes.input} text-xs" {placeholder} bind:value />
</div>
