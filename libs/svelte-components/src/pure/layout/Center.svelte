<!-- Centers content both horizontally and vertically using flexbox. -->
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
	};

	// Maps uniform padding values to Tailwind padding classes.
	const paddingMap: Record<number, string> = {
		0: "p-0",
		1: "p-1",
		2: "p-2",
		3: "p-3",
		4: "p-4",
		6: "p-6",
		8: "p-8",
	};

	// Maps flex values to Tailwind flex-shrink/grow classes.
	const flexMap: Record<number, string> = {
		0: "flex-none",
		1: "flex-1",
	};

	let {
		full = false,
		gap,
		padding,
		flex,
		children,
	}: {
		/** When true, expands to fill the full height of the parent. */
		full?: boolean;
		/** Gap between children when Center contains multiple items. */
		gap?: 0 | 1 | 2 | 3 | 4;
		/** Uniform padding on all sides. */
		padding?: 0 | 1 | 2 | 3 | 4 | 6 | 8;
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		children?: Snippet;
	} = $props();

	const gapClass = $derived(gap != null ? gapMap[gap] : undefined);
	const paddingClass = $derived(padding != null ? paddingMap[padding] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
</script>

<div
	class={cn(
		"flex items-center justify-center",
		full && "h-full",
		gapClass,
		paddingClass,
		flexClass,
	)}
>
	{@render children?.()}
</div>
