<!-- Vertical stack (flex column) — a structural lego block.

Stack expresses HOW children arrange vertically. It is not a visual container —
it has NO padding, NO background, NO border, NO rounded, NO margin. Anything
decorative belongs in a purpose-built component (Panel, Card, SectionHeader).

The only props Stack exposes are:
  • STRUCTURAL  — gap, align, flex, height, width, minHeight, full
  • WIRING      — children, role, aria-*

Defaults: align="stretch", overflow hidden. Scrollable panels wrap content in a
ScrollArea. There is no overflow prop — by design. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Closed-set gap vocabulary. Any gap that isn't in this map is a design-system
	// violation — adding new values means extending the design tokens first.
	const gapMap: Record<number, string> = {
		0: "gap-0",
		1: "gap-1",
		2: "gap-2",
		3: "gap-3",
		4: "gap-4",
		6: "gap-6",
		8: "gap-8",
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

	let {
		gap = 2,
		align = "stretch",
		full = false,
		height,
		width,
		minHeight,
		minWidth,
		flex,
		role,
		"aria-label": ariaLabel,
		"aria-multiselectable": ariaMultiselectable,
		children,
	}: {
		gap?: 0 | 1 | 2 | 3 | 4 | 6 | 8;
		align?: "start" | "center" | "end" | "stretch";
		/** When true, expands to fill the full height of the parent. */
		full?: boolean;
		/** Fixed height shorthand. */
		height?: "full" | "screen";
		/** Fixed width shorthand. */
		width?: "full" | "screen";
		/** Sets min-h-0 to allow flex children to shrink below content size. */
		minHeight?: 0;
		/** Sets min-w-0 to allow flex children to shrink below content size. */
		minWidth?: 0;
		/** flex-none (0) or flex-1 (1) shorthand. */
		flex?: 0 | 1;
		role?: string;
		"aria-label"?: string;
		"aria-multiselectable"?: boolean | "true" | "false";
		children?: Snippet;
	} = $props();

	const gapClass = $derived(gapMap[gap] ?? "gap-2");
	const heightClass = $derived(height != null ? heightMap[height] : undefined);
	const widthClass = $derived(width != null ? widthMap[width] : undefined);
	const flexClass = $derived(flex != null ? flexMap[flex] : undefined);
</script>

<div
	class={cn(
		"flex flex-col overflow-hidden",
		gapClass,
		align === "center" && "items-center",
		align === "end" && "items-end",
		align === "stretch" && "items-stretch",
		align === "start" && "items-start",
		full && "h-full",
		heightClass,
		widthClass,
		minHeight === 0 && "min-h-0",
		minWidth === 0 && "min-w-0",
		flexClass,
	)}
	role={role || undefined}
	aria-label={ariaLabel || undefined}
	aria-multiselectable={ariaMultiselectable}
>
	{@render children?.()}
</div>
