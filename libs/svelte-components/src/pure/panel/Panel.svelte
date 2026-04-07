<!-- Panel — padded visual container, a semantic lego block.

Panel is the home for "I need a visual container with padding, background,
border, rounded". All visual state is selected via closed-set variant props —
there are no free spacing, color, or size values.

Use Panel instead of a Box that was passed padding/background/border/rounded.
For card layouts, use Card. For top/bottom bars with borders and tight padding,
use SectionHeader or SectionFooter. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Padding presets — one value each for all four sides. Asymmetric padding is
	// not supported by design; if you need it, create a dedicated semantic
	// component (e.g. SectionHeader for padX>padY horizontal bars).
	const paddingMap: Record<string, string> = {
		none: "p-0",
		tight: "p-2",
		normal: "p-4",
		loose: "p-6",
	};

	// Background presets — tied to the design token palette.
	const backgroundMap: Record<string, string> = {
		none: "",
		card: "bg-card text-card-foreground",
		muted: "bg-muted",
		"muted-subtle": "bg-muted/30",
		"muted-faint": "bg-muted/10",
		surface: "bg-surface",
	};

	// Border presets. Single-sided borders cover all four sides for structural
	// dividers and sidebar panels. "dashed" adds a dashed full border for empty states.
	const borderMap: Record<string, string> = {
		none: "",
		all: "border border-border",
		top: "border-t border-border",
		bottom: "border-b border-border",
		left: "border-l border-border",
		right: "border-r border-border",
		dashed: "border border-dashed border-border",
	};

	// Fixed-width presets for sidebar panels and toolbars.
	const fixedWidthMap: Record<string, string> = {
		"icon-bar": "w-12 shrink-0",
		"nav-sm": "w-[200px] shrink-0",
		"nav-md": "w-[240px] shrink-0",
		"nav-lg": "w-56 shrink-0",
	};

	// Layout direction — default is block; "column" makes Panel a flex container.
	const directionMap: Record<string, string> = {
		column: "flex flex-col",
	};

	// Align-items for column direction panels.
	const alignMap: Record<string, string> = {
		center: "items-center",
		start: "items-start",
		end: "items-end",
		stretch: "items-stretch",
	};

	// Border radius presets.
	const roundedMap: Record<string, string> = {
		none: "",
		sm: "rounded-sm",
		md: "rounded-md",
		lg: "rounded-lg",
		xl: "rounded-xl",
	};

	// Flex participation.
	const flexMap: Record<number, string> = {
		0: "flex-none",
		1: "flex-1",
	};

	let {
		padding = "normal",
		background = "none",
		border = "none",
		rounded = "none",
		fixedWidth,
		direction,
		align,
		full = false,
		height,
		width,
		minHeight,
		minWidth,
		flex,
		role,
		"aria-label": ariaLabel,
		children,
	}: {
		/** Uniform padding preset. */
		padding?: "none" | "tight" | "normal" | "loose";
		/** Background token. */
		background?: "none" | "card" | "muted" | "muted-subtle" | "muted-faint" | "surface";
		/** Border treatment. "dashed" adds a dashed full border for empty states. */
		border?: "none" | "all" | "top" | "bottom" | "left" | "right" | "dashed";
		/** Border radius preset. */
		rounded?: "none" | "sm" | "md" | "lg" | "xl";
		/** Fixed-width preset for sidebar panels: icon-bar (w-12), nav-sm (200px), nav-md (240px), nav-lg (w-56). */
		fixedWidth?: "icon-bar" | "nav-sm" | "nav-md" | "nav-lg";
		/** Layout direction — omit for block layout, "column" for flex-col sidebar panels. */
		direction?: "column";
		/** Align-items for flex direction panels. */
		align?: "center" | "start" | "end" | "stretch";
		/** Fill available height. */
		full?: boolean;
		height?: "full" | "screen";
		width?: "full" | "screen" | "auto";
		minHeight?: 0;
		minWidth?: 0;
		flex?: 0 | 1;
		role?: string;
		"aria-label"?: string;
		children?: Snippet;
	} = $props();

	const paddingClass = $derived(paddingMap[padding] ?? paddingMap.normal);
	const backgroundClass = $derived(backgroundMap[background] ?? "");
	const fixedWidthClass = $derived(fixedWidth != null ? fixedWidthMap[fixedWidth] : undefined);
	const directionClass = $derived(direction != null ? directionMap[direction] : undefined);
	const alignClass = $derived(align != null ? alignMap[align] : undefined);
	const borderClass = $derived(borderMap[border] ?? "");
	const roundedClass = $derived(roundedMap[rounded] ?? "");
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
</script>

<div
	class={cn(
		"overflow-hidden",
		paddingClass,
		backgroundClass,
		borderClass,
		roundedClass,
		fixedWidthClass,
		directionClass,
		alignClass,
		full && "h-full",
		height === "full" && "h-full",
		height === "screen" && "h-screen",
		width === "full" && "w-full",
		width === "screen" && "w-screen",
		width === "auto" && "w-auto",
		minHeight === 0 && "min-h-0",
		minWidth === 0 && "min-w-0",
		flexClass,
	)}
	role={role || undefined}
	aria-label={ariaLabel || undefined}
>
	{@render children?.()}
</div>
