<!-- GridCell — a grid item that can span multiple columns.
     Used inside a Grid when a child needs to occupy more than one column track.
     For single-column-span children, use direct Grid children without a wrapper. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Maps span count to Tailwind col-span classes.
	const spanMap: Record<number, string> = {
		1: "col-span-1",
		2: "col-span-2",
		3: "col-span-3",
		4: "col-span-4",
		5: "col-span-5",
		6: "col-span-6",
		12: "col-span-12",
	};

	let {
		span = 1,
		children,
	}: {
		/** Number of columns this cell spans. */
		span?: 1 | 2 | 3 | 4 | 5 | 6 | 12;
		children?: Snippet;
	} = $props();

	const spanClass = $derived(spanMap[span] ?? "col-span-1");
</script>

<div class={cn(spanClass)}>
	{@render children?.()}
</div>
