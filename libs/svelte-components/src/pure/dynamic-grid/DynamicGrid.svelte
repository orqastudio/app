<!-- DynamicGrid — a CSS grid that accepts a runtime column count and minimum column width.
     Used when the number of columns is not known at compile time (e.g. a kanban board
     whose columns come from a plugin registry). For static column counts, use Grid instead.
     Renders: grid-template-columns: repeat(columns, minmax(minWidth, 1fr)) -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		columns,
		minWidth = "200px",
		gap = 4,
		children,
	}: {
		/** Number of columns to generate. */
		columns: number;
		/** Minimum column width token string (e.g. "200px", "12rem"). Defaults to "200px". */
		minWidth?: string;
		/** Gap size in Tailwind spacing units. Defaults to 4 (1rem). */
		gap?: 1 | 2 | 3 | 4 | 6 | 8;
		children?: Snippet;
	} = $props();

	const gapClass: Record<number, string> = {
		1: "gap-1",
		2: "gap-2",
		3: "gap-3",
		4: "gap-4",
		6: "gap-6",
		8: "gap-8",
	};
</script>

<div
	class="grid {gapClass[gap] ?? 'gap-4'}"
	style:grid-template-columns="repeat({columns}, minmax({minWidth}, 1fr))"
>
	{@render children?.()}
</div>
