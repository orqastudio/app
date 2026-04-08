<!-- TreeIndent — depth-based padding container for hierarchical tree-view leaf rows.
     Applies left padding computed from depth and step (default 12px) so leaf nodes
     align under their parent labels. The dynamic pixel value cannot be expressed
     as a closed-set Tailwind token. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		depth,
		step = 12,
		children,
	}: {
		/** Tree depth level (0-based). Each level adds `step` pixels of left padding. */
		depth: number;
		/** Pixels per depth level. Defaults to 12. */
		step?: number;
		children?: Snippet;
	} = $props();

	const paddingLeft = $derived(`${depth * step}px`);
</script>

<div style="padding-left: {paddingLeft}">
	{@render children?.()}
</div>
