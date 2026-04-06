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
		surface: "bg-surface",
	};

	// Border presets. Single-sided borders use "top"/"bottom" for structural
	// dividers between sections; asymmetric left/right borders are intentionally
	// not supported.
	const borderMap: Record<string, string> = {
		none: "",
		all: "border border-border",
		top: "border-t border-border",
		bottom: "border-b border-border",
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
		full = false,
		height,
		width,
		minHeight,
		minWidth,
		flex,
		role,
		tabindex,
		"aria-label": ariaLabel,
		onclick,
		children,
	}: {
		/** Uniform padding preset. */
		padding?: "none" | "tight" | "normal" | "loose";
		/** Background token. */
		background?: "none" | "card" | "muted" | "surface";
		/** Border treatment. `all` = box border; `top`/`bottom` = single divider. */
		border?: "none" | "all" | "top" | "bottom";
		/** Border radius preset. */
		rounded?: "none" | "sm" | "md" | "lg" | "xl";
		/** Fill available height. */
		full?: boolean;
		height?: "full" | "screen";
		width?: "full" | "screen" | "auto";
		minHeight?: 0;
		minWidth?: 0;
		flex?: 0 | 1;
		role?: string;
		tabindex?: number;
		"aria-label"?: string;
		onclick?: (e: MouseEvent) => void;
		children?: Snippet;
	} = $props();

	const paddingClass = $derived(paddingMap[padding] ?? paddingMap.normal);
	const backgroundClass = $derived(backgroundMap[background] ?? "");
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
		full && "h-full",
		height === "full" && "h-full",
		height === "screen" && "h-screen",
		width === "full" && "w-full",
		width === "screen" && "w-screen",
		width === "auto" && "w-auto",
		minHeight === 0 && "min-h-0",
		minWidth === 0 && "min-w-0",
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
