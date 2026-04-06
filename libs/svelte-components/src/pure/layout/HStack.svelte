<!-- Horizontal stack (flex row) — a structural lego block.

HStack expresses HOW children arrange horizontally. It is not a visual container
— it has NO padding, NO background, NO border, NO rounded, NO margin. Anything
decorative belongs in a purpose-built component (Panel, Toolbar, SectionHeader).

The only props HStack exposes are:
  • STRUCTURAL  — gap, align, justify, wrap, flex, height, width, minHeight, full
  • WIRING      — children, role, tabindex, aria-*, onclick, onkeydown

Defaults: align="center", overflow hidden. Scrollable rows wrap content in a
ScrollArea. There is no overflow prop — by design. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Closed-set gap vocabulary. Adding values means extending design tokens first.
	const gapMap: Record<number, string> = {
		0: "gap-0",
		0.5: "gap-0.5",
		1: "gap-1",
		1.5: "gap-1.5",
		2: "gap-2",
		3: "gap-3",
		4: "gap-4",
		6: "gap-6",
		8: "gap-8",
	};

	const justifyMap: Record<string, string> = {
		start: "justify-start",
		center: "justify-center",
		end: "justify-end",
		between: "justify-between",
		around: "justify-around",
	};

	const alignMap: Record<string, string> = {
		start: "items-start",
		center: "items-center",
		end: "items-end",
		baseline: "items-baseline",
		stretch: "items-stretch",
	};

	const heightMap: Record<string, string> = {
		full: "h-full",
		screen: "h-screen",
	};

	const widthMap: Record<string, string> = {
		full: "w-full",
		screen: "w-screen",
	};

	const flexMap: Record<number, string> = {
		0: "flex-none",
		1: "flex-1",
	};

	// Tree-indent presets. Each level = 8px (ml-2 step). Used for hierarchical
	// rows where depth is data-driven (tree views, outline lists). Closed set.
	const indentMap: Record<number, string> = {
		0: "ml-0",
		1: "ml-2",
		2: "ml-4",
		3: "ml-6",
		4: "ml-8",
		5: "ml-10",
		6: "ml-12",
		7: "ml-14",
		8: "ml-16",
	};

	let {
		gap = 2,
		align = "center",
		justify = "start",
		wrap = false,
		full = false,
		height,
		width,
		minHeight,
		flex,
		indent,
		role,
		tabindex,
		"aria-selected": ariaSelected,
		"aria-label": ariaLabel,
		onclick,
		onkeydown,
		children,
	}: {
		gap?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		align?: "start" | "center" | "end" | "baseline" | "stretch";
		justify?: "start" | "center" | "end" | "between" | "around";
		wrap?: boolean;
		/** Fill available width. */
		full?: boolean;
		/** Fixed height shorthand. */
		height?: "full" | "screen";
		/** Fixed width shorthand. */
		width?: "full" | "screen";
		/** Sets min-h-0 to allow flex children to shrink below content size. */
		minHeight?: 0;
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		/** Tree depth. Each level adds 8px of left margin. Clamped to 0-8. */
		indent?: 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8;
		role?: string;
		tabindex?: number;
		"aria-selected"?: boolean;
		"aria-label"?: string;
		onclick?: (e: MouseEvent) => void;
		onkeydown?: (e: KeyboardEvent) => void;
		children?: Snippet;
	} = $props();

	const gapClass = $derived(gapMap[gap] ?? "gap-2");
	const justifyClass = $derived(justifyMap[justify] ?? "justify-start");
	const alignClass = $derived(alignMap[align] ?? "items-center");
	const heightClass = $derived(height != null ? heightMap[height] : undefined);
	const widthClass = $derived(width != null ? widthMap[width] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
	const indentClass = $derived(indent != null ? indentMap[indent] : undefined);
</script>

<div
	class={cn(
		"flex overflow-hidden",
		gapClass,
		justifyClass,
		alignClass,
		wrap && "flex-wrap",
		full && "w-full",
		onclick && "cursor-pointer",
		heightClass,
		widthClass,
		minHeight === 0 && "min-h-0",
		flexClass,
		indentClass,
	)}
	{role}
	{tabindex}
	aria-selected={ariaSelected}
	aria-label={ariaLabel}
	{onclick}
	{onkeydown}
>
	{@render children?.()}
</div>
