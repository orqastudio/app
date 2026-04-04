<!-- Renders a keyboard shortcut key with consistent styling. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();
</script>

<kbd
	class="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground"
>
	{@render children?.()}
</kbd>
