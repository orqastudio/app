<!-- Horizontal stack (flex row) layout primitive. Supports semantic interactive
     attributes (onclick, role, tabindex, aria-*) for accessible interactive rows. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Maps numeric gap values to Tailwind gap classes.
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

	// Maps justify prop values to Tailwind justify-content classes.
	const justifyMap: Record<string, string> = {
		start: "justify-start",
		center: "justify-center",
		end: "justify-end",
		between: "justify-between",
		around: "justify-around",
	};

	// Maps align prop values to Tailwind align-items classes.
	const alignMap: Record<string, string> = {
		start: "items-start",
		center: "items-center",
		end: "items-end",
		baseline: "items-baseline",
		stretch: "items-stretch",
	};

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

	// Maps overflow values to Tailwind overflow classes.
	const overflowMap: Record<string, string> = {
		hidden: "overflow-hidden",
		auto: "overflow-auto",
		scroll: "overflow-scroll",
		visible: "overflow-visible",
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
		gap = 2,
		align = "center",
		justify = "start",
		wrap = false,
		full = false,
		padding,
		paddingX,
		paddingY,
		paddingTop,
		paddingBottom,
		height,
		overflow,
		minHeight,
		flex,
		borderTop = false,
		borderBottom = false,
		marginTop,
		role,
		tabindex,
		"aria-selected": ariaSelected,
		"aria-label": ariaLabel,
		onclick,
		onkeydown,
		style,
		children,
	}: {
		gap?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		align?: "start" | "center" | "end" | "baseline" | "stretch";
		justify?: "start" | "center" | "end" | "between" | "around";
		wrap?: boolean;
		/** Fill available width. */
		full?: boolean;
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
		/** Overflow behaviour. */
		overflow?: "hidden" | "auto" | "scroll" | "visible";
		/** Sets min-h-0 to allow flex children to shrink below content size. */
		minHeight?: 0;
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		/** Adds a top border. */
		borderTop?: boolean;
		/** Adds a bottom border. */
		borderBottom?: boolean;
		/** Top margin. */
		marginTop?: 0 | 1 | 2 | 3 | 4 | 6 | 8;
		role?: string;
		tabindex?: number;
		"aria-selected"?: boolean;
		"aria-label"?: string;
		onclick?: (e: MouseEvent) => void;
		onkeydown?: (e: KeyboardEvent) => void;
		/** Inline style for dynamic positioning (e.g. indentation). */
		style?: string;
		children?: Snippet;
	} = $props();

	const gapClass = $derived(gapMap[gap] ?? "gap-2");
	const justifyClass = $derived(justifyMap[justify] ?? "justify-start");
	const alignClass = $derived(alignMap[align] ?? "items-center");
	const paddingClass = $derived(padding != null ? paddingMap[padding] : undefined);
	const paddingXClass = $derived(paddingX != null ? paddingXMap[paddingX] : undefined);
	const paddingYClass = $derived(paddingY != null ? paddingYMap[paddingY] : undefined);
	const paddingTopClass = $derived(paddingTop != null ? paddingTopMap[paddingTop] : undefined);
	const paddingBottomClass = $derived(paddingBottom != null ? paddingBottomMap[paddingBottom] : undefined);
	const heightClass = $derived(height != null ? heightMap[height] : undefined);
	const overflowClass = $derived(overflow != null ? overflowMap[overflow] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
	const marginTopClass = $derived(marginTop != null ? marginTopMap[marginTop] : undefined);
</script>

<div
	class={cn(
		"flex",
		gapClass,
		justifyClass,
		alignClass,
		wrap && "flex-wrap",
		full && "w-full",
		onclick && "cursor-pointer",
		paddingClass,
		paddingXClass,
		paddingYClass,
		paddingTopClass,
		paddingBottomClass,
		heightClass,
		overflowClass,
		minHeight === 0 && "min-h-0",
		flexClass,
		borderTop && "border-t",
		borderBottom && "border-b",
		marginTopClass,
	)}
	{role}
	{tabindex}
	aria-selected={ariaSelected}
	aria-label={ariaLabel}
	{onclick}
	{onkeydown}
	{style}
>
	{@render children?.()}
</div>
