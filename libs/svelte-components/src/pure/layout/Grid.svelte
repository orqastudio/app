<!-- CSS grid layout primitive with optional responsive column breakpoints.
     The `items` prop controls align-items for the grid (default: "stretch").
     For dynamic runtime column counts, use DynamicGrid instead. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Maps numeric gap values to Tailwind gap classes.
	const gapMap: Record<number, string> = {
		0: "gap-0",
		1: "gap-1",
		2: "gap-2",
		3: "gap-3",
		4: "gap-4",
		6: "gap-6",
		8: "gap-8",
	};

	// Maps column count to Tailwind grid-cols classes (base, sm, md, lg prefixes).
	const colsMap: Record<number, string> = {
		1: "grid-cols-1",
		2: "grid-cols-2",
		3: "grid-cols-3",
		4: "grid-cols-4",
		5: "grid-cols-5",
		6: "grid-cols-6",
		12: "grid-cols-12",
	};

	const smColsMap: Record<number, string> = {
		1: "sm:grid-cols-1",
		2: "sm:grid-cols-2",
		3: "sm:grid-cols-3",
		4: "sm:grid-cols-4",
		5: "sm:grid-cols-5",
		6: "sm:grid-cols-6",
		12: "sm:grid-cols-12",
	};

	const mdColsMap: Record<number, string> = {
		1: "md:grid-cols-1",
		2: "md:grid-cols-2",
		3: "md:grid-cols-3",
		4: "md:grid-cols-4",
		5: "md:grid-cols-5",
		6: "md:grid-cols-6",
		12: "md:grid-cols-12",
	};

	const lgColsMap: Record<number, string> = {
		1: "lg:grid-cols-1",
		2: "lg:grid-cols-2",
		3: "lg:grid-cols-3",
		4: "lg:grid-cols-4",
		5: "lg:grid-cols-5",
		6: "lg:grid-cols-6",
		12: "lg:grid-cols-12",
	};

	// align-items preset for the grid container.
	const itemsMap: Record<string, string> = {
		stretch: "items-stretch",
		start: "items-start",
		center: "items-center",
		end: "items-end",
	};

	let {
		cols = 1,
		gap = 2,
		sm,
		md,
		lg,
		items,
		children,
	}: {
		cols?: 1 | 2 | 3 | 4 | 5 | 6 | 12;
		gap?: 0 | 1 | 2 | 3 | 4 | 6 | 8;
		sm?: 1 | 2 | 3 | 4 | 5 | 6 | 12;
		md?: 1 | 2 | 3 | 4 | 5 | 6 | 12;
		lg?: 1 | 2 | 3 | 4 | 5 | 6 | 12;
		/** align-items for all grid cells. Omit to use browser default (stretch). */
		items?: "stretch" | "start" | "center" | "end";
		children?: Snippet;
	} = $props();

	const colsClass = $derived(colsMap[cols] ?? "grid-cols-1");
	const gapClass = $derived(gapMap[gap] ?? "gap-2");
	const smClass = $derived(sm != null ? smColsMap[sm] : undefined);
	const mdClass = $derived(md != null ? mdColsMap[md] : undefined);
	const lgClass = $derived(lg != null ? lgColsMap[lg] : undefined);
	const itemsClass = $derived(items != null ? itemsMap[items] : undefined);
</script>

<div class={cn("grid", colsClass, gapClass, smClass, mdClass, lgClass, itemsClass)}>
	{@render children?.()}
</div>
