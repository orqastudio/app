<!-- MenuBarTrigger — a DropdownMenuTrigger styled for a compact horizontal menu bar.
     Renders a text trigger with muted foreground, accent hover, and accent-on-open states.
     Encapsulates the trigger styling so app code passes no class prop. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import DropdownMenuTrigger from "./dropdown-menu-trigger.svelte";

	let {
		onclick,
		onmouseenter,
		children,
	}: {
		/** Click handler — typically preventDefault then toggle the menu. */
		onclick?: (e: MouseEvent) => void;
		/** Hover handler — used for switching menus while a menu bar is active. */
		onmouseenter?: (e: MouseEvent) => void;
		children?: Snippet;
	} = $props();
</script>

<DropdownMenuTrigger
	class="text-muted-foreground hover:bg-accent hover:text-foreground data-[state=open]:bg-accent data-[state=open]:text-foreground flex h-7 items-center rounded px-2.5 text-xs font-medium transition-colors"
	{onclick}
	{onmouseenter}
>
	{@render children?.()}
</DropdownMenuTrigger>
