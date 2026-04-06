<!-- Structural container — a lego block with no visual opinions.

Box is the "I need to participate in flex/absolute positioning without visual
styling" primitive. It has NO padding, NO background, NO border, NO rounded,
NO margin. Anything decorative belongs in a purpose-built component (Panel,
Card, Toolbar, SectionHeader).

The only props Box exposes are:
  • STRUCTURAL  — flex, height, width, minHeight, minWidth, position, inset,
                  top, right, bottom, left, zIndex
  • WIRING      — children, role, tabindex, aria-*, onclick

Overflow is hardcoded to hidden. Scrollable regions wrap content in a ScrollArea. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	const heightMap: Record<string, string> = {
		full: "h-full",
		screen: "h-screen",
	};

	const widthMap: Record<string, string> = {
		full: "w-full",
		screen: "w-screen",
		auto: "w-auto",
	};

	const positionMap: Record<string, string> = {
		relative: "relative",
		absolute: "absolute",
		fixed: "fixed",
		sticky: "sticky",
	};

	const insetMap: Record<number, string> = {
		0: "inset-0",
	};

	const topMap: Record<number, string> = {
		0: "top-0",
		1: "top-1",
		2: "top-2",
		3: "top-3",
	};

	const rightMap: Record<number, string> = {
		0: "right-0",
		1: "right-1",
		2: "right-2",
		3: "right-3",
	};

	const bottomMap: Record<number, string> = {
		0: "bottom-0",
		1: "bottom-1",
		2: "bottom-2",
		3: "bottom-3",
	};

	const leftMap: Record<number, string> = {
		0: "left-0",
		1: "left-1",
		2: "left-2",
		3: "left-3",
	};

	const zIndexMap: Record<number, string> = {
		10: "z-10",
		20: "z-20",
		30: "z-30",
		40: "z-40",
		50: "z-50",
	};

	const flexMap: Record<number, string> = {
		0: "flex-none",
		1: "flex-1",
	};

	let {
		height,
		width,
		minHeight,
		minWidth,
		position,
		inset,
		top,
		right,
		bottom,
		left,
		zIndex,
		flex,
		role,
		tabindex,
		"aria-label": ariaLabel,
		onclick,
		children,
	}: {
		/** Fixed height shorthand. */
		height?: "full" | "screen";
		/** Fixed width shorthand. */
		width?: "full" | "screen" | "auto";
		/** Sets min-h-0 to allow flex children to shrink below content size. */
		minHeight?: 0;
		/** Sets min-w-0 to allow flex children to shrink below content size. */
		minWidth?: 0;
		/** CSS position. Use with top/right/bottom/left or inset for offsets. */
		position?: "relative" | "absolute" | "fixed" | "sticky";
		/** Sets inset-0 (all four offsets to 0). */
		inset?: 0;
		/** Top offset. */
		top?: 0 | 1 | 2 | 3;
		/** Right offset. */
		right?: 0 | 1 | 2 | 3;
		/** Bottom offset. */
		bottom?: 0 | 1 | 2 | 3;
		/** Left offset. */
		left?: 0 | 1 | 2 | 3;
		/** z-index layer. */
		zIndex?: 10 | 20 | 30 | 40 | 50;
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		role?: string;
		tabindex?: number;
		"aria-label"?: string;
		onclick?: (e: MouseEvent) => void;
		children?: Snippet;
	} = $props();

	const heightClass = $derived(height != null ? heightMap[height] : undefined);
	const widthClass = $derived(width != null ? widthMap[width] : undefined);
	const positionClass = $derived(position != null ? positionMap[position] : undefined);
	const insetClass = $derived(inset != null ? insetMap[inset] : undefined);
	const topClass = $derived(top != null ? topMap[top] : undefined);
	const rightClass = $derived(right != null ? rightMap[right] : undefined);
	const bottomClass = $derived(bottom != null ? bottomMap[bottom] : undefined);
	const leftClass = $derived(left != null ? leftMap[left] : undefined);
	const zIndexClass = $derived(zIndex != null ? zIndexMap[zIndex] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
</script>

<div
	class={cn(
		"overflow-hidden",
		heightClass,
		widthClass,
		minHeight === 0 && "min-h-0",
		minWidth === 0 && "min-w-0",
		positionClass,
		insetClass,
		topClass,
		rightClass,
		bottomClass,
		leftClass,
		zIndexClass,
		flexClass,
		onclick && "cursor-pointer",
	)}
	{role}
	{tabindex}
	aria-label={ariaLabel}
	{onclick}
>
	{@render children?.()}
</div>
