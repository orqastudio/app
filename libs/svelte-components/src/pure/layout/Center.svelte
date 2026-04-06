<!-- Center — a structural lego block that centers content both axes.

Center expresses HOW children align. It is not a visual container — it has NO
padding, NO background, NO border. Anything decorative belongs in a purpose-built
component. For a centered card on a padded background, wrap Center in Panel. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	const gapMap: Record<number, string> = {
		0: "gap-0",
		1: "gap-1",
		2: "gap-2",
		3: "gap-3",
		4: "gap-4",
	};

	const flexMap: Record<number, string> = {
		0: "flex-none",
		1: "flex-1",
	};

	let {
		full = false,
		gap,
		flex,
		children,
	}: {
		/** When true, expands to fill the full height of the parent. */
		full?: boolean;
		/** Gap between children when Center contains multiple items. */
		gap?: 0 | 1 | 2 | 3 | 4;
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		children?: Snippet;
	} = $props();

	const gapClass = $derived(gap != null ? gapMap[gap] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
</script>

<div
	class={cn(
		"flex items-center justify-center",
		full && "h-full",
		gapClass,
		flexClass,
	)}
>
	{@render children?.()}
</div>
