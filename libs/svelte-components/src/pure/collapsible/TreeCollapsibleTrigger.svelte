<!-- TreeCollapsibleTrigger — CollapsibleTrigger with depth-based left padding for hierarchical trees.
     The dynamic pixel padding cannot be expressed as a closed-set Tailwind token.
     Encapsulates the raw style attribute so app code stays free of inline styles.
     Default visual styling (muted text, accent hover, uppercase label) is built in;
     pass no class from app code. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import CollapsibleTrigger from "./collapsible-trigger.svelte";

	let {
		depth,
		step = 12,
		children,
	}: {
		/** Tree depth (0-based). Each level adds `step` pixels of left padding. */
		depth: number;
		/** Pixels per depth level. Defaults to 12. */
		step?: number;
		children?: Snippet;
	} = $props();

	const paddingLeft = $derived(`${depth * step}px`);
</script>

<CollapsibleTrigger
	class="text-muted-foreground hover:bg-accent/50 flex w-full items-center gap-1 rounded px-1 py-1 text-xs font-semibold tracking-wide uppercase"
	style="padding-left: {paddingLeft}"
>
	{@render children?.()}
</CollapsibleTrigger>
