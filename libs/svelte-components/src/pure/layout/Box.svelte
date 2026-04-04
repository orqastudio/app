<!-- General-purpose container primitive. Provides the full layout prop surface for
     cases where Stack/HStack/Center semantics don't fit. Maps all props to literal
     Tailwind classes via Record lookups — no string interpolation. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Maps uniform padding values to Tailwind padding classes.
	const paddingMap: Record<number, string> = {
		0: "p-0",
		0.5: "p-0.5",
		1: "p-1",
		1.5: "p-1.5",
		2: "p-2",
		3: "p-3",
		4: "p-4",
		6: "p-6",
		8: "p-8",
	};

	// Maps horizontal padding values to Tailwind px classes.
	const paddingXMap: Record<number, string> = {
		0: "px-0",
		0.5: "px-0.5",
		1: "px-1",
		1.5: "px-1.5",
		2: "px-2",
		3: "px-3",
		4: "px-4",
		6: "px-6",
		8: "px-8",
	};

	// Maps vertical padding values to Tailwind py classes.
	const paddingYMap: Record<number, string> = {
		0: "py-0",
		0.5: "py-0.5",
		1: "py-1",
		1.5: "py-1.5",
		2: "py-2",
		3: "py-3",
		4: "py-4",
		6: "py-6",
		8: "py-8",
	};

	// Maps top padding values to Tailwind pt classes.
	const paddingTopMap: Record<number, string> = {
		0: "pt-0",
		0.5: "pt-0.5",
		1: "pt-1",
		1.5: "pt-1.5",
		2: "pt-2",
		3: "pt-3",
		4: "pt-4",
		6: "pt-6",
		8: "pt-8",
	};

	// Maps bottom padding values to Tailwind pb classes.
	const paddingBottomMap: Record<number, string> = {
		0: "pb-0",
		0.5: "pb-0.5",
		1: "pb-1",
		1.5: "pb-1.5",
		2: "pb-2",
		3: "pb-3",
		4: "pb-4",
		6: "pb-6",
		8: "pb-8",
	};

	// Maps height values to Tailwind height classes.
	const heightMap: Record<string, string> = {
		full: "h-full",
		screen: "h-screen",
	};

	// Maps width values to Tailwind width classes.
	const widthMap: Record<string, string> = {
		full: "w-full",
		auto: "w-auto",
	};

	// Maps overflow values to Tailwind overflow classes.
	const overflowMap: Record<string, string> = {
		hidden: "overflow-hidden",
		auto: "overflow-auto",
		scroll: "overflow-scroll",
		visible: "overflow-visible",
	};

	// Maps position values to Tailwind position classes.
	const positionMap: Record<string, string> = {
		relative: "relative",
		absolute: "absolute",
		fixed: "fixed",
		sticky: "sticky",
	};

	// Maps inset values to Tailwind inset classes.
	const insetMap: Record<number, string> = {
		0: "inset-0",
	};

	// Maps top offset values to Tailwind top classes.
	const topMap: Record<number, string> = {
		0: "top-0",
		1: "top-1",
		2: "top-2",
		3: "top-3",
	};

	// Maps right offset values to Tailwind right classes.
	const rightMap: Record<number, string> = {
		0: "right-0",
		1: "right-1",
		2: "right-2",
		3: "right-3",
	};

	// Maps bottom offset values to Tailwind bottom classes.
	const bottomMap: Record<number, string> = {
		0: "bottom-0",
		1: "bottom-1",
		2: "bottom-2",
		3: "bottom-3",
	};

	// Maps left offset values to Tailwind left classes.
	const leftMap: Record<number, string> = {
		0: "left-0",
		1: "left-1",
		2: "left-2",
		3: "left-3",
	};

	// Maps z-index values to Tailwind z classes.
	const zIndexMap: Record<number, string> = {
		10: "z-10",
		20: "z-20",
		30: "z-30",
		40: "z-40",
		50: "z-50",
	};

	// Maps rounded values to Tailwind border-radius classes.
	const roundedMap: Record<string, string> = {
		none: "rounded-none",
		sm: "rounded-sm",
		md: "rounded-md",
		lg: "rounded-lg",
		xl: "rounded-xl",
		full: "rounded-full",
	};

	// Maps background shorthand values to Tailwind background classes using CSS variables.
	const backgroundMap: Record<string, string> = {
		card: "bg-card",
		muted: "bg-muted",
		surface: "bg-surface",
		transparent: "bg-transparent",
	};

	// Maps marginTop values to Tailwind margin-top classes.
	const marginTopMap: Record<number, string> = {
		0: "mt-0",
		1: "mt-1",
		2: "mt-2",
		3: "mt-3",
		4: "mt-4",
		6: "mt-6",
		8: "mt-8",
	};

	// Maps flex values to Tailwind flex-shrink/grow classes.
	const flexMap: Record<number, string> = {
		0: "flex-none",
		1: "flex-1",
	};

	let {
		padding,
		paddingX,
		paddingY,
		paddingTop,
		paddingBottom,
		height,
		width,
		overflow,
		minHeight,
		minWidth,
		position,
		inset,
		top,
		right,
		bottom,
		left,
		zIndex,
		border = false,
		borderTop = false,
		borderBottom = false,
		rounded,
		background,
		flex,
		marginTop,
		role,
		tabindex,
		"aria-label": ariaLabel,
		onclick,
		children,
	}: {
		/** Uniform padding on all sides. */
		padding?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		/** Horizontal (left + right) padding. */
		paddingX?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		/** Vertical (top + bottom) padding. */
		paddingY?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		/** Top padding. */
		paddingTop?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		/** Bottom padding. */
		paddingBottom?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		/** Fixed height shorthand. */
		height?: "full" | "screen";
		/** Fixed width shorthand. */
		width?: "full" | "auto";
		/** Overflow behaviour. */
		overflow?: "hidden" | "auto" | "scroll" | "visible";
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
		/** Adds a full border on all sides. */
		border?: boolean;
		/** Adds a top border. */
		borderTop?: boolean;
		/** Adds a bottom border. */
		borderBottom?: boolean;
		/** Border radius shorthand. */
		rounded?: "none" | "sm" | "md" | "lg" | "xl" | "full";
		/** Background colour from design token set. */
		background?: "card" | "muted" | "surface" | "transparent";
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		/** Top margin. */
		marginTop?: 0 | 1 | 2 | 3 | 4 | 6 | 8;
		role?: string;
		tabindex?: number;
		"aria-label"?: string;
		onclick?: (e: MouseEvent) => void;
		children?: Snippet;
	} = $props();

	const paddingClass = $derived(padding != null ? paddingMap[padding] : undefined);
	const paddingXClass = $derived(paddingX != null ? paddingXMap[paddingX] : undefined);
	const paddingYClass = $derived(paddingY != null ? paddingYMap[paddingY] : undefined);
	const paddingTopClass = $derived(paddingTop != null ? paddingTopMap[paddingTop] : undefined);
	const paddingBottomClass = $derived(paddingBottom != null ? paddingBottomMap[paddingBottom] : undefined);
	const heightClass = $derived(height != null ? heightMap[height] : undefined);
	const widthClass = $derived(width != null ? widthMap[width] : undefined);
	const overflowClass = $derived(overflow != null ? overflowMap[overflow] : undefined);
	const positionClass = $derived(position != null ? positionMap[position] : undefined);
	const insetClass = $derived(inset != null ? insetMap[inset] : undefined);
	const topClass = $derived(top != null ? topMap[top] : undefined);
	const rightClass = $derived(right != null ? rightMap[right] : undefined);
	const bottomClass = $derived(bottom != null ? bottomMap[bottom] : undefined);
	const leftClass = $derived(left != null ? leftMap[left] : undefined);
	const zIndexClass = $derived(zIndex != null ? zIndexMap[zIndex] : undefined);
	const roundedClass = $derived(rounded != null ? roundedMap[rounded] : undefined);
	const backgroundClass = $derived(background != null ? backgroundMap[background] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
	const marginTopClass = $derived(marginTop != null ? marginTopMap[marginTop] : undefined);
</script>

<div
	class={cn(
		paddingClass,
		paddingXClass,
		paddingYClass,
		paddingTopClass,
		paddingBottomClass,
		heightClass,
		widthClass,
		overflowClass,
		minHeight === 0 && "min-h-0",
		minWidth === 0 && "min-w-0",
		positionClass,
		insetClass,
		topClass,
		rightClass,
		bottomClass,
		leftClass,
		zIndexClass,
		border && "border",
		borderTop && "border-t",
		borderBottom && "border-b",
		roundedClass,
		backgroundClass,
		flexClass,
		marginTopClass,
		onclick && "cursor-pointer",
	)}
	{role}
	{tabindex}
	aria-label={ariaLabel}
	{onclick}
>
	{@render children?.()}
</div>
