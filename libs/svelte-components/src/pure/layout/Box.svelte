<!-- Structural container — a lego block with no visual opinions.

Box is the "I need to participate in flex/absolute positioning without visual
styling" primitive. It has NO padding, NO background, NO border, NO rounded,
NO margin. Anything decorative belongs in a purpose-built component (Panel,
Card, Toolbar, SectionHeader).

The only props Box exposes are:
  • STRUCTURAL  — flex, height, width, maxWidth, minHeight, minWidth, position,
                  inset, top, right, bottom, left, zIndex
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

	// Closed-set max-width tokens. Use for constraining content width in fixed-width containers.
	const maxWidthMap: Record<string, string> = {
		"60": "max-w-60",
		xs: "max-w-xs",
		sm: "max-w-sm",
		md: "max-w-md",
		lg: "max-w-lg",
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

	const topMap: Record<string, string> = {
		0: "top-0",
		0.5: "top-0.5",
		1: "top-1",
		2: "top-2",
		3: "top-3",
	};

	const rightMap: Record<string, string> = {
		0: "right-0",
		0.5: "right-0.5",
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

	const leftMap: Record<string, string> = {
		0: "left-0",
		1: "left-1",
		2: "left-2",
		3: "left-3",
		"1/2": "left-1/2",
	};

	const zIndexMap: Record<number, string> = {
		10: "z-10",
		20: "z-20",
		30: "z-30",
		40: "z-40",
		50: "z-50",
	};

	// Transform presets. "center-x" horizontally centres an absolute/fixed element using
	// left-1/2 + -translate-x-1/2. Used for floating overlays (e.g. scroll-to-bottom button).
	const transformMap: Record<string, string> = {
		"center-x": "-translate-x-1/2",
	};

	// Fixed icon-sized square presets. Used as invisible spacers for menu icon column alignment.
	const sizeMap: Record<string, string> = {
		"icon-sm": "h-3.5 w-3.5 shrink-0",
		"icon-md": "h-4 w-4 shrink-0",
	};

	const flexMap: Record<number, string> = {
		0: "flex-none",
		1: "flex-1",
	};

	// Named min-height tokens for semantic step containers and similar layout anchors.
	const minHeightTokenMap: Record<string, string> = {
		// "step" = 12.5rem — prevents wizard/step cards from collapsing during content transitions.
		step: "min-h-50",
	};

	let {
		ref = $bindable<HTMLDivElement | undefined>(undefined),
		height,
		width,
		maxWidth,
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
		transform,
		size,
		truncate,
		role,
		tabindex,
		"aria-label": ariaLabel,
		"aria-hidden": ariaHidden,
		onclick,
		onmouseenter,
		children,
	}: {
		/** Bindable reference to the underlying div element (e.g. for canvas/Cytoscape mounting). */
		ref?: HTMLDivElement;
		/** Fixed height shorthand. */
		height?: "full" | "screen";
		/** Fixed width shorthand. */
		width?: "full" | "screen" | "auto";
		/** Maximum width token. Use "60" for 240px, or xs/sm/md/lg for standard widths. */
		maxWidth?: "60" | "xs" | "sm" | "md" | "lg";
		/** Sets min-h-0 to allow flex children to shrink below content size. Use "step" for 12.5rem wizard card anchors. */
		minHeight?: 0 | "step";
		/** Sets min-w-0 to allow flex children to shrink below content size. */
		minWidth?: 0;
		/** CSS position. Use with top/right/bottom/left or inset for offsets. */
		position?: "relative" | "absolute" | "fixed" | "sticky";
		/** Sets inset-0 (all four offsets to 0). */
		inset?: 0;
		/** Top offset. */
		top?: 0 | 0.5 | 1 | 2 | 3;
		/** Right offset. */
		right?: 0 | 0.5 | 1 | 2 | 3;
		/** Bottom offset. */
		bottom?: 0 | 1 | 2 | 3;
		/** Left offset. Use "1/2" with transform="center-x" to horizontally centre an absolute element. */
		left?: 0 | 1 | 2 | 3 | "1/2";
		/** z-index layer. */
		zIndex?: 10 | 20 | 30 | 40 | 50;
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		/** Transform preset. "center-x" applies -translate-x-1/2 for centering via left-1/2. */
		transform?: "center-x";
		/** Fixed icon-sized square. Used as invisible spacers for icon column alignment in menus. */
		size?: "icon-sm" | "icon-md";
		/** When true, truncates overflowing text with an ellipsis. */
		truncate?: boolean;
		role?: string;
		tabindex?: number;
		"aria-label"?: string;
		"aria-hidden"?: boolean | "true" | "false";
		onclick?: (e: MouseEvent) => void;
		onmouseenter?: (e: MouseEvent) => void;
		children?: Snippet;
	} = $props();

	const heightClass = $derived(height != null ? heightMap[height] : undefined);
	const widthClass = $derived(width != null ? widthMap[width] : undefined);
	const maxWidthClass = $derived(maxWidth != null ? maxWidthMap[maxWidth] : undefined);
	const positionClass = $derived(position != null ? positionMap[position] : undefined);
	const insetClass = $derived(inset != null ? insetMap[inset] : undefined);
	const minHeightTokenClass = $derived(
		typeof minHeight === "string" ? (minHeightTokenMap[minHeight] ?? undefined) : undefined,
	);
	const topClass = $derived(top != null ? topMap[top] : undefined);
	const rightClass = $derived(right != null ? rightMap[right] : undefined);
	const bottomClass = $derived(bottom != null ? bottomMap[bottom] : undefined);
	const leftClass = $derived(left != null ? leftMap[left] : undefined);
	const zIndexClass = $derived(zIndex != null ? zIndexMap[zIndex] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
	const transformClass = $derived(transform != null ? transformMap[transform] : undefined);
	const sizeClass = $derived(size != null ? sizeMap[size] : undefined);
</script>

<div
	bind:this={ref}
	class={cn(
		"overflow-hidden",
		heightClass,
		widthClass,
		maxWidthClass,
		minHeight === 0 && "min-h-0",
		minHeightTokenClass,
		minWidth === 0 && "min-w-0",
		positionClass,
		insetClass,
		topClass,
		rightClass,
		bottomClass,
		leftClass,
		zIndexClass,
		flexClass,
		transformClass,
		sizeClass,
		truncate && "truncate",
		onclick && "cursor-pointer",
	)}
	role={role || undefined}
	tabindex={tabindex != null ? tabindex : undefined}
	aria-label={ariaLabel || undefined}
	aria-hidden={ariaHidden != null ? ariaHidden : undefined}
	{onclick}
	{onmouseenter}
>
	{@render children?.()}
</div>
